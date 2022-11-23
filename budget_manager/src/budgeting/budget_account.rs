use crate::budgeting::budget_account::Error::{
    CategoryAlreadyExists, CategoryNotFound, CategoryUpdateFailed,
};
use crate::schema::budget_accounts;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;
use diesel::{Insertable, Queryable};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::hash::Hash;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Error transferring fund from one category to other")]
    FundTransferError,
    #[error("Category does not exist")]
    CategoryNotFound,
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Transaction Category update failed")]
    CategoryUpdateFailed,
}

use crate::budgeting::transaction::{Transaction, TransactionBuilder};
use crate::budgeting::transaction_category::{TransactionCategory, TransactionCategoryBuilder};
use crate::{current_date, establish_connection, DEFAULT_CATEGORY};

/// Budget is used to store all the transaction categories and store their details in a file
#[derive(Default, Serialize, Deserialize, Queryable, Debug, Identifiable, Clone)]
pub struct BudgetAccount {
    id: i32,
    filed_as: String,
    date_created: NaiveDateTime,
    #[diesel(sql_type = Double)]
    balance: f64,
}

impl BudgetAccount {
    pub(crate) fn create_and_allocate(
        &mut self,
        conn: &mut SqliteConnection,
        category: &str,
        allocate: f64,
    ) -> Result<TransactionCategory, Error> {
        let t = self
            .category_builder(category)
            .allocated(allocate)
            .done(conn);
        self.transfer_fund(conn, DEFAULT_CATEGORY, category, allocate)?;
        Ok(t)
    }
}

impl BudgetAccount {}

pub struct BudgetAccountBuilder {
    filed_as: String,
    date_created: Option<NaiveDateTime>,
    balance: Option<f64>,
}

impl BudgetAccountBuilder {
    pub fn new(filed_as: &str) -> BudgetAccountBuilder {
        BudgetAccountBuilder {
            filed_as: filed_as.to_string(),
            date_created: Some(current_date()),
            balance: None,
        }
    }

    pub fn balance(&mut self, balance: f64) -> &mut Self {
        self.balance = Some(balance);
        self
    }

    pub fn date_created(&mut self, date_created: NaiveDateTime) -> &mut Self {
        self.date_created = Some(date_created);
        self
    }

    pub fn done(&mut self, conn: &mut SqliteConnection) -> BudgetAccount {
        imp_db!(budget_accounts);
        let b = BudgetAccount::find_by_name(conn, &self.filed_as);
        if b.is_ok() {
            panic!("Budget account already exists!")
        }
        let initial_balance = self.balance.unwrap_or(0.);
        let new_budget = NewBudgetAccount {
            filed_as: &self.filed_as,
            balance: initial_balance,
            date_created: Default::default(),
        };
        let mut b: QueryResult<BudgetAccount> =
            save_model!(conn, budget_accounts, new_budget, BudgetAccount);
        if b.is_err() {
            panic!("Failed to create budget account");
        }
        let mut b = b.unwrap().clone();
        // create the default category
        let t = b
            .category_builder(DEFAULT_CATEGORY)
            .allocated(0.)
            .done(conn);
        // create entry for initial balance in the default category
        t.new_income(initial_balance)
            .payee("Self")
            .note("Initial Balance")
            .done(conn);
        b
    }
}

#[derive(Insertable)]
#[diesel(table_name = budget_accounts)]
pub struct NewBudgetAccount<'a> {
    filed_as: &'a str,
    balance: f64,
    date_created: NaiveDateTime,
}

impl BudgetAccount {
    pub fn find_by_name(conn: &mut SqliteConnection, target: &str) -> QueryResult<BudgetAccount> {
        imp_db!(budget_accounts);
        budget_accounts
            .filter(filed_as.eq(target))
            .first::<BudgetAccount>(conn)
    }

    pub fn save(&mut self, conn: &mut SqliteConnection) {
        use chrono::NaiveDateTime;
        let account = self.get_new_budget_account();
        let budget_account = save_model!(conn, budget_accounts, account, BudgetAccount);
    }

    pub fn load(conn: &mut SqliteConnection, id: i32) -> BudgetAccount {
        imp_db!(budget_accounts);
        budget_accounts.find(id).first(conn).unwrap()
    }

    pub(crate) fn new_expense_to_category(
        &self,
        conn: &mut SqliteConnection,
        category_name: &str,
        amount: f64,
    ) -> Result<TransactionBuilder, Error> {
        let category = self.find_category(conn, &category_name)?;
        Ok(category.new_expense(amount))
    }

    pub fn get_new_budget_account(&self) -> NewBudgetAccount {
        NewBudgetAccount {
            filed_as: self.filed_as.as_str(),
            balance: self.balance,
            date_created: self.date_created,
        }
    }

    pub fn default_category(&self, conn: &mut SqliteConnection) -> TransactionCategory {
        imp_db!(transaction_categories);
        transaction_categories
            .filter(budget_account_id.eq(self.id))
            .filter(name.eq(DEFAULT_CATEGORY))
            .first::<TransactionCategory>(conn)
            .unwrap()
    }

    /// returns the total unallocated balance
    pub fn uncategorized_balance(&self, conn: &mut SqliteConnection) -> f64 {
        let c = self.default_category(conn);
        imp_db!(transactions);
        let result_option = Transaction::belonging_to(&c)
            .select(sum(amount))
            .first::<Option<f64>>(conn);
        return_sum!(result_option)
    }

    pub(crate) fn transfer_fund(
        &mut self,
        conn: &mut SqliteConnection,
        src: &str,
        dest: &str,
        amount: f64,
    ) -> Result<(), Error> {
        self.find_category(conn, src)
            .unwrap()
            .new_expense(amount)
            .payee(&format!("{}", dest))
            .note(&format!("Transferred to {}", dest))
            .done(conn);
        self.find_category(conn, dest)
            .unwrap()
            .new_income(amount)
            .payee(&format!("{}", dest))
            .note(&format!("Received from {}", dest))
            .done(conn);
        Ok(())
    }

    pub fn categories(&self, conn: &mut SqliteConnection) -> Vec<TransactionCategory> {
        imp_db!(transaction_categories);
        TransactionCategory::belonging_to(&self)
            .filter(name.ne(DEFAULT_CATEGORY))
            .load::<TransactionCategory>(conn)
            .unwrap()
    }

    pub(crate) fn category_balance(&self, conn: &mut SqliteConnection, category_name: &str) -> f64 {
        self.find_category(conn, category_name)
            .unwrap()
            .balance(conn)
    }

    pub fn category_builder(&mut self, category_name: &str) -> TransactionCategoryBuilder {
        TransactionCategoryBuilder::new(self.id, category_name)
    }

    pub fn find_category(
        &self,
        conn: &mut SqliteConnection,
        category_name: &str,
    ) -> Result<TransactionCategory, Error> {
        imp_db!(transaction_categories);
        imp_db!(budget_accounts);
        let result: QueryResult<TransactionCategory> = TransactionCategory::belonging_to(&self)
            .filter(name.eq(category_name))
            .filter(budget_account_id.eq(self.id))
            .first(conn);
        result.map_err(|e| CategoryNotFound)
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn total_category_balance(&self, conn: &mut SqliteConnection) -> f64 {
        self.categories(conn)
            .iter()
            .map(|v| v.balance(conn))
            .sum::<f64>()
    }

    pub fn total_allocated(&self, conn: &mut SqliteConnection) -> f64 {
        imp_db!(transaction_categories);
        let result_option: QueryResult<Option<f64>> = transaction_categories::table
            .select(sum(allocated))
            .filter(name.ne(DEFAULT_CATEGORY))
            .filter(budget_account_id.eq(self.id))
            .first::<Option<f64>>(conn);
        return_sum!(result_option)
    }

    /// actual total balance? it is the real money available
    /// sum of all the category balances + unallocated balance
    /// unallocated balance would be balance unused + all the transactions in unallocated category
    pub fn actual_total_balance(&self, conn: &mut SqliteConnection) -> f64 {
        imp_db!(transactions);
        let result = transactions.select(sum(amount)).first::<Option<f64>>(conn);
        return_sum!(result);
    }

    pub fn total_income(&self, conn: &mut SqliteConnection) -> f64 {
        self.categories(conn)
            .iter()
            .map(|v| v.income(conn))
            .sum::<f64>()
    }

    pub fn total_expense(&self, conn: &mut SqliteConnection) -> f64 {
        self.categories(conn)
            .iter()
            .map(|v| v.expense(conn))
            .sum::<f64>()
            * -1.
    }

    pub fn transactions(&self, conn: &mut SqliteConnection) -> Vec<Transaction> {
        imp_db!(transactions);
        transactions.load::<Transaction>(conn).unwrap()
    }

    pub fn date_created(&self) -> NaiveDateTime {
        self.date_created
    }

    pub fn set_date_created(&mut self, date_created: NaiveDateTime) {
        self.date_created = date_created;
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Default)]
struct NewTransactionProc {
    amount: f64,
    payee: String,
    note: String,
    a: bool,
    p: bool,
    n: bool,
}

fn keys_match<T: Eq + Hash + Ord, U, V>(map1: &BTreeMap<T, U>, map2: &BTreeMap<T, V>) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::budgeting::budget_account::Error::FundTransferError;
    use crate::budgeting::transaction::Transaction;
    use crate::tests::{new_budget, DbDropper, BILLS, DEFAULT_ID, INITIAL, TRAVEL, UNUSED};
    use diesel::prelude::*;

    #[test]
    fn initial_budget_account_details() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = BudgetAccountBuilder::new("main").balance(10000.).done(conn);
        assert_eq!(budget.uncategorized_balance(conn), INITIAL);
        assert_eq!(budget.actual_total_balance(conn), INITIAL);
    }

    #[test]
    fn allocating_money_behaviour() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = BudgetAccountBuilder::new("main").balance(10000.).done(conn);
        // now allocate some money
        budget
            .create_and_allocate(conn, "Bills", BILLS)
            .expect("Failed to create category");
        budget
            .create_and_allocate(conn, "Travel", TRAVEL)
            .expect("Failed to create category");
        let r = budget.default_category(conn).transactions(conn);
        println!("{:#?}", r);
        assert_eq!(budget.uncategorized_balance(conn), 5000.);
    }

    #[test]
    fn total_allocation_check() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        assert_eq!(budget.total_allocated(conn), 5000.);
    }

    #[test]
    fn total_balance_is_actual_money() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        // a transaction without any category
        let mut tc = budget
            .find_category(&mut conn, DEFAULT_CATEGORY)
            .unwrap()
            .new_expense(1000.)
            .payee("Some")
            .note("Other")
            .done(conn);
        assert_eq!(budget.uncategorized_balance(conn), 4000.);
    }

    #[test]
    fn make_transaction() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        let mut tc = budget
            .find_category(&mut conn, "Bills")
            .unwrap()
            .new_expense(1000.)
            .payee("Some")
            .note("Other")
            .done(conn);
    }

    #[test]
    fn transactions_in_default_category_should_change_balance() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        let t = budget.transactions(conn);
        println!("{:#?}", t);
        let mut tb = budget
            .find_category(conn, DEFAULT_CATEGORY)
            .unwrap()
            .new_expense(1000.)
            .payee("Some")
            .note("Other")
            .done(conn);
        let mut tb = budget
            .find_category(conn, DEFAULT_CATEGORY)
            .unwrap()
            .new_income(5000.)
            .payee("Some")
            .note("Other")
            .done(conn);
        imp_db!(budget_accounts);
        assert_eq!(
            budget.category_balance(conn, DEFAULT_CATEGORY),
            -1000. + 10000.
        )
    }

    #[test]
    pub fn total_balance_should_be_sum_of_all_categories_balance() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        assert_eq!(budget.actual_total_balance(conn), TRAVEL + BILLS + UNUSED);
        let mut tb = budget
            .find_category(conn, "Travel")
            .unwrap()
            .new_expense(1000.)
            .payee("Some")
            .note("Other")
            .done(conn);
        let mut tb = budget
            .find_category(conn, "Travel")
            .unwrap()
            .new_income(500.)
            .payee("Some")
            .note("Other")
            .done(conn);
        assert_eq!(
            budget.actual_total_balance(conn),
            TRAVEL + BILLS + UNUSED + 500. - 1000.
        );
    }

    #[test]
    fn finding_category_by_name_in_budget_account() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        {
            let bills = budget.find_category(conn, "Bills").unwrap();
            assert_eq!(bills.allocated(), BILLS);
            assert_eq!(bills.balance(conn), BILLS);
        }
        let mut tb = budget
            .find_category(conn, "Bills")
            .unwrap()
            .new_income(500.)
            .payee("Some")
            .note("Other")
            .done(conn);
        {
            let bills = budget.find_category(conn, "Bills").unwrap();
            assert_eq!(bills.allocated(), BILLS);
            assert_eq!(bills.balance(conn), BILLS + 500.);
        }
    }

    #[test]
    fn creating_category_and_do_transactions() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        let home = {
            let home = budget.create_and_allocate(conn, "Home", 3000.).unwrap();
            assert_eq!(home.allocated(), 3000.0);
            assert_eq!(home.balance(conn), 3000.0);
            home
        };
        let t1 = home
            .new_expense(2000.)
            .payee("someone")
            .note("test")
            .done(conn);
        let t2 = home
            .new_income(1000.)
            .payee("another someone")
            .note("test some")
            .done(conn);
        assert_eq!(home.balance(conn), 2000.);
        assert_eq!(home.allocated(), 3000.);
        assert_eq!(home.expense(conn), -2000.);
        assert_eq!(home.income(conn), 4000.);
    }

    #[test]
    pub fn spending_from_category() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(conn);
        let bills_available = budget.category_balance(conn, "Bills");
        assert_eq!(bills_available, BILLS);
        assert_eq!(budget.actual_total_balance(conn), BILLS + TRAVEL + UNUSED);
        budget
            .find_category(conn, "Bills")
            .unwrap()
            .new_expense(BILLS)
            .payee("someone")
            .note("test")
            .done(conn);
        let bills_available = budget.category_balance(conn, "Bills");
        assert_eq!(bills_available, 0.0);
        assert_eq!(budget.actual_total_balance(conn), TRAVEL + UNUSED);
    }

    #[test]
    fn transfer_fund_from_balance() {
        let mut dd = DbDropper::new();
        let mut conn = dd.conn();
        let mut budget = new_budget(&mut conn);
        assert!(budget.transfer_fund(conn, "Bills", "Travel", BILLS).is_ok());
        let bills = budget.find_category(conn, "Bills").unwrap();
        let travel = budget.find_category(conn, "Travel").unwrap();
        assert_eq!(bills.balance(conn), 0.);
        assert_eq!(travel.balance(conn), BILLS + TRAVEL);
    }
}
