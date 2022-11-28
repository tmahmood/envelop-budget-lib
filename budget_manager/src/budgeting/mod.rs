use crate::budgeting::budget_account::{BudgetAccount, BudgetAccountBuilder, NewBudgetAccount};
use crate::budgeting::category::{Category, CategoryBuilder, CategoryModel};
use crate::budgeting::transaction::{Transaction, TransactionType};
use crate::budgeting::Error::{BudgetAccountNotFound, CategoryNotFound, FailedToCreateBudget};
use crate::transaction_op::TransactionAddToCategoryOps;
use crate::{establish_connection, DEFAULT_CATEGORY};
use diesel::connection::BoxableConnection;
use diesel::dsl::sum;
use diesel::{QueryResult, SqliteConnection};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub mod budget_account;
pub mod category;
pub mod storage;
pub mod transaction;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Error transferring fund from one category to other")]
    FundTransferError,
    #[error("Category does not exist")]
    CategoryNotFound,
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Category update failed")]
    CategoryUpdateFailed,
    #[error("Budget Account not found")]
    BudgetAccountNotFound(String),
    #[error("Failed to create budget")]
    FailedToCreateBudget(String),
}

mod builder {
    pub trait Builder {
        fn build(&self) -> Self;
    }
}

pub mod prelude {
    use super::budget_account::*;
    use super::category::*;
    use super::transaction::*;
}

pub struct Budgeting {
    conn: Rc<SqliteConnection>,
    budget: Option<BudgetAccount>,
}

impl Default for Budgeting {
    fn default() -> Self {
        Budgeting::new()
    }
}

impl Budgeting {
    /// Starts a new transaction belonging to given category.
    /// it's not completed until `done` method is called
    pub fn new_transaction_to_category<'b>(
        &'b mut self,
        category: &'b str,
    ) -> TransactionAddToCategoryOps {
        let b = self.current_budget();
        let mut cm = self.get_category_model(category);
        TransactionAddToCategoryOps::new(b, cm)
    }

    /// Transfers fund from one category to another
    pub(crate) fn transfer_fund(
        &mut self,
        src: &str,
        dest: &str,
        amount: f64,
    ) -> Result<(), Error> {
        let mut cm = self.get_category_model(src);
        self.new_transaction_to_category(src)
            .transfer_from(amount)
            .payee(&format!("{}", dest))
            .note(&format!("Transferred to {}", dest))
            .done();
        let mut cm = self.get_category_model(dest);
        self.new_transaction_to_category(dest)
            .transfer_to(amount)
            .payee(&format!("{}", dest))
            .note(&format!("Received from {}", src))
            .done();
        Ok(())
    }

    // creates a new category and allocates the budget
    pub fn create_category_and_allocate(
        &mut self,
        category: &str,
        allocate: f64,
    ) -> Result<Category, Error> {
        let mut budget = self.current_budget();
        let t = self.category_builder(category).allocated(allocate).done();
        self.transfer_fund(DEFAULT_CATEGORY, category, allocate)?;
        Ok(t)
    }

    pub fn category_model(&mut self, category: Category) -> CategoryModel {
        CategoryModel::new(self.conn(), category)
    }

    pub fn get_category_model(&mut self, category_name: &str) -> CategoryModel {
        let c = self.find_category(category_name).unwrap();
        self.category_model(c)
    }

    pub fn find_category(&mut self, category_name: &str) -> Result<Category, Error> {
        imp_db!(categories);
        imp_db!(budget_accounts);
        let mut budget = { self.current_budget() };
        let conn = self.conn();
        let result: QueryResult<Category> = Category::belonging_to(&budget)
            .filter(name.eq(category_name))
            .first(conn);
        result.map_err(|e| CategoryNotFound)
    }

    pub fn category_balance(&mut self, category: &str) -> f64 {
        imp_db!(categories);
        imp_db!(budget_accounts);
        self.get_category_model(category).balance()
    }

    pub fn find_budget(&mut self, filed_as: &str) -> QueryResult<BudgetAccount> {
        imp_db!(budget_accounts);
        let budget_account: QueryResult<BudgetAccount> = budget_accounts
            .filter(filed_as.eq(filed_as))
            .first(self.conn());
        budget_account
    }

    pub fn current_budget(&mut self) -> BudgetAccount {
        let b = self.budget.as_ref().unwrap();
        b.clone()
    }

    pub fn set_current_budget(
        &mut self,
        filed_as: &str,
    ) -> Result<BudgetAccount, crate::budgeting::Error> {
        let b = self.find_budget(filed_as);
        if b.is_err() {
            return Err(BudgetAccountNotFound(filed_as.to_string()));
        }
        let b = b.unwrap();
        self.budget = Some(b.clone());
        Ok(b)
    }

    pub fn category_builder(&mut self, category_name: &str) -> CategoryBuilder {
        let id = self.current_budget().id();
        CategoryBuilder::new(self.conn(), id, category_name)
    }

    pub fn new_budget(
        &mut self,
        filed_as: &str,
        amount: f64,
    ) -> Result<BudgetAccount, crate::budgeting::Error> {
        let budget_account = self.find_budget(filed_as);
        if budget_account.is_ok() {
            return Err(FailedToCreateBudget(filed_as.to_string()));
        }
        let mut b = BudgetAccountBuilder::new(self.conn(), "main").build();
        self.budget = Some(b.clone());
        // create the default category
        self.category_builder(DEFAULT_CATEGORY).allocated(0.).done();
        self.new_transaction_to_category(DEFAULT_CATEGORY)
            .income(amount)
            .payee("Self")
            .note("Initial Balance")
            .done();
        Ok(b)
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn categories(&mut self) -> Vec<Category> {
        imp_db!(categories);
        Category::belonging_to(&self.current_budget())
            .filter(name.ne(DEFAULT_CATEGORY))
            .load::<Category>(self.conn())
            .unwrap()
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn total_category_balance(&mut self) -> f64 {
        imp_db!(transactions);
        self.categories()
            .iter()
            .map(|v| CategoryModel::new(self.conn(), v.clone()).balance())
            .sum::<f64>()
    }

    pub fn total_allocated(&mut self) -> f64 {
        imp_db!(categories);
        let result_option: QueryResult<Option<f64>> =
            Category::belonging_to(&self.current_budget())
                .select(sum(allocated))
                .filter(name.ne(DEFAULT_CATEGORY))
                .first::<Option<f64>>(self.conn());
        return_sum!(result_option)
    }

    pub fn default_category(&mut self) -> Category {
        imp_db!(categories);
        Category::belonging_to(&self.current_budget())
            .filter(name.eq(DEFAULT_CATEGORY))
            .first::<Category>(self.conn())
            .unwrap()
    }

    /// actual total balance? it is the real money available
    /// sum of all the category balances + unallocated balance
    /// unallocated balance would be balance unused + all the transactions in unallocated category
    pub fn actual_total_balance(&mut self) -> f64 {
        imp_db!(transactions);
        let result = transactions
            .select(sum(amount))
            .first::<Option<f64>>(self.conn());
        return_sum!(result);
    }

    /// returns the total unallocated balance
    pub fn uncategorized_balance(&mut self) -> f64 {
        let c = self.default_category();
        imp_db!(transactions);
        let result_option = Transaction::belonging_to(&c)
            .select(sum(amount))
            .first::<Option<f64>>(self.conn());
        return_sum!(result_option)
    }

    pub fn total_income(&mut self) -> f64 {
        self.categories()
            .iter()
            .map(|v| CategoryModel::new(self.conn(), v.clone()).income())
            .sum::<f64>()
    }

    pub fn total_expense(&mut self) -> f64 {
        self.categories()
            .iter()
            .map(|v| CategoryModel::new(self.conn(), v.clone()).expense())
            .sum::<f64>()
            * -1.
    }

    pub fn transactions(&mut self) -> Vec<Transaction> {
        imp_db!(transactions);
        transactions.load::<Transaction>(self.conn()).unwrap()
    }

    pub(crate) fn conn(&mut self) -> &mut SqliteConnection {
        Rc::get_mut(&mut self.conn).unwrap()
    }

    pub fn new() -> Self {
        let r = Rc::new(establish_connection());
        Budgeting {
            conn: r,
            budget: None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::budgeting::transaction::Transaction;
    use crate::budgeting::Error::FundTransferError;
    use crate::test_helpers::{new_budget_using_budgeting, DbDropper};
    use crate::tests::{BILLS, DEFAULT_ID, INITIAL, TRAVEL, UNUSED};
    use diesel::prelude::*;

    #[test]
    fn initial_budget_account_details() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        blib.new_budget("main", 10000.);
        assert_eq!(blib.uncategorized_balance(), INITIAL);
        assert_eq!(blib.actual_total_balance(), INITIAL);
    }

    #[test]
    fn allocating_money_behaviour() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        blib.new_budget("main", 10000.);
        blib.create_category_and_allocate("Bills", BILLS)
            .expect("Failed to create category");
        blib.create_category_and_allocate("Travel", TRAVEL)
            .expect("Failed to create category");
        let v = blib.get_category_model(DEFAULT_CATEGORY).transactions();
        println!("{:#?}", v);
        assert_eq!(blib.uncategorized_balance(), 5000.);
    }

    #[test]
    fn total_allocation_check() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        assert_eq!(blib.total_allocated(), 5000.);
    }

    #[test]
    fn total_balance_is_actual_money() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        // a transaction without any category
        let mut tc = blib
            .new_transaction_to_category(DEFAULT_CATEGORY)
            .expense(1000.)
            .payee("Some")
            .note("Other")
            .done();
        assert_eq!(blib.uncategorized_balance(), 4000.);
    }

    #[test]
    fn transactions_in_default_category_should_change_balance() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let mut def = blib.new_transaction_to_category(DEFAULT_CATEGORY);
        def.expense(1000.).payee("Some").note("Other").done();
        def.income(5000.).payee("Some").note("Other").done();
        imp_db!(budget_accounts);
        assert_eq!(blib.category_balance(DEFAULT_CATEGORY), -1000. + 10000.)
    }

    #[test]
    pub fn total_balance_should_be_sum_of_all_categories_balance() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let mut travel = blib.new_transaction_to_category("Travel");
        let mut tb = travel.expense(1000.).payee("Some").note("Other").done();
        let mut tb = travel.income(500.).payee("Some").note("Other").done();
        assert_eq!(blib.actual_total_balance(), 5000. + BILLS + TRAVEL - 500.);
    }

    #[test]
    fn finding_category_by_name_in_budget_account() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        {
            let category = blib.find_category("Bills").unwrap();
            let mut bills = CategoryModel::new(blib.conn(), category);
            assert_eq!(bills.allocated(), BILLS);
            assert_eq!(bills.balance(), BILLS);
        }
        let mut bills = blib.new_transaction_to_category("Bills");
        let mut tb = bills.income(500.).payee("Some").note("Other").done();
        {
            let category = blib.find_category("Bills").unwrap();
            let mut bills = CategoryModel::new(blib.conn(), category);
            assert_eq!(bills.allocated(), BILLS);
            assert_eq!(bills.balance(), BILLS + 500.);
        }
    }

    #[test]
    fn creating_category_and_do_transactions() {
        let _dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let _home = {
            let home = blib.create_category_and_allocate("Home", 3000.).unwrap();
            assert_eq!(home.allocated(), 3000.0);
            assert_eq!(blib.category_balance("Home"), 3000.0);
            home
        };
        let mut home_ops = blib.new_transaction_to_category("Home");
        home_ops.expense(2000.).payee("someone").note("test").done();
        home_ops
            .income(1000.)
            .payee("another someone")
            .note("test some")
            .done();
        assert_eq!(blib.category_balance("Home"), 2000.0);
        let mut cm = blib.get_category_model("Home");
        assert_eq!(cm.allocated(), 3000.);
        assert_eq!(cm.expense(), -2000.);
        assert_eq!(cm.income(), 1000.);
    }

    #[test]
    pub fn spending_from_category() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let bills_available = blib.category_balance("Bills");
        assert_eq!(bills_available, BILLS);
        assert_eq!(blib.actual_total_balance(), BILLS + TRAVEL + UNUSED);
        blib.new_transaction_to_category("Bills")
            .expense(BILLS)
            .payee("someone")
            .note("test")
            .done();
        let bills_available = blib.category_balance("Bills");
        assert_eq!(bills_available, 0.0);
        assert_eq!(blib.actual_total_balance(), TRAVEL + UNUSED);
    }
}
