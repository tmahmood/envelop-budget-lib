use crate::budgeting::budget_account::BudgetAccount;
use crate::budgeting::transaction::{Transaction, TransactionBuilder};
use crate::budgeting::transaction_category;
use crate::schema::categories;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Queryable,
    Associations,
    Identifiable,
)]
#[diesel(belongs_to(BudgetAccount))]
#[diesel(table_name = categories)]
pub struct Category {
    id: i32,
    name: String,
    allocated: f64,
    budget_account_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewTransactionCategory<'a> {
    name: &'a str,
    allocated: f64,
    budget_account_id: i32,
}

/// Only way to create transaction category.
/// as we need to maintain the budget_account_id
pub struct CategoryBuilder {
    name: String,
    allocated: f64,
    budget_account_id: i32,
}

impl Category {
    fn new_transaction(&self, amount: f64, income: bool) -> TransactionBuilder {
        let mut tb = if income {
            TransactionBuilder::new_income(amount)
        } else {
            TransactionBuilder::new_expense(amount)
        };
        tb.category(self.id);
        tb
    }
    pub(crate) fn new_expense(&self, amount: f64) -> TransactionBuilder {
        self.new_transaction(amount, false)
    }

    pub(crate) fn new_income(&self, amount: f64) -> TransactionBuilder {
        self.new_transaction(amount, true)
    }

    pub(crate) fn delete(conn: &mut SqliteConnection, id: i32) -> usize {
        imp_db!(categories);
        diesel::delete(categories.filter(id.eq(&id)))
            .execute(conn)
            .expect("Error deleting transaction category")
    }

    pub fn load(conn: &mut SqliteConnection, id: i32) -> QueryResult<Category> {
        imp_db!(categories);
        categories.find(id).first(conn)
    }

    pub fn find_by_name(
        conn: &mut SqliteConnection,
        name: &str,
        budget_account_id: i32,
    ) -> QueryResult<Category> {
        imp_db!(categories);
        categories
            .filter(budget_account_id.eq(&budget_account_id))
            .filter(name.eq(name))
            .first(conn)
    }

    pub fn transactions(&self, conn: &mut SqliteConnection) -> Vec<Transaction> {
        imp_db!(transactions);
        transactions
            .filter(category_id.eq(self.id))
            .load::<Transaction>(conn)
            .unwrap()
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn allocated(&self) -> f64 {
        self.allocated.into()
    }

    pub fn update_allocation(
        &self,
        conn: &mut SqliteConnection,
        new_allocation: f64,
    ) -> QueryResult<usize> {
        imp_db!(categories);
        diesel::update(categories)
            .set(allocated.eq(new_allocation))
            .execute(conn)
    }

    pub fn budget_account_id(&self) -> i32 {
        self.budget_account_id
    }

    pub fn get_new_transaction_category(&self) -> NewTransactionCategory {
        NewTransactionCategory {
            name: self.name.as_str(),
            allocated: self.allocated,
            budget_account_id: self.budget_account_id,
        }
    }

    pub fn income(&self, conn: &mut SqliteConnection) -> f64 {
        imp_db!(transactions);
        let result_option: QueryResult<Option<f64>> = transactions::table
            .select(sum(amount))
            .filter(category_id.eq(self.id))
            .filter(amount.gt(0.))
            .first::<Option<f64>>(conn);
        return_sum!(result_option)
    }

    pub fn expense(&self, conn: &mut SqliteConnection) -> f64 {
        imp_db!(transactions);
        let result_option = transactions::table
            .select(sum(amount))
            .filter(category_id.eq(self.id))
            .filter(amount.lt(0.))
            .first::<Option<f64>>(conn);
        return_sum!(result_option)
    }

    pub fn balance(&self, conn: &mut SqliteConnection) -> f64 {
        imp_db!(transactions);
        let result_option: QueryResult<Option<f64>> = transactions::table
            .select(sum(amount))
            .filter(category_id.eq(self.id))
            .first::<Option<f64>>(conn);
        return_sum!(result_option)
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl CategoryBuilder {
    pub fn new(budget_account_id: i32, name: &str) -> Self {
        Self {
            name: name.to_string(),
            allocated: 0.0,
            budget_account_id,
        }
    }

    pub fn allocated(&mut self, allocated: f64) -> &mut Self {
        self.allocated = allocated;
        self
    }

    // put the transaction category details together and save to database, returned the saved transaction
    pub fn done(&mut self, conn: &mut SqliteConnection) -> Category {
        let mut t = NewTransactionCategory {
            name: self.name.as_str(),
            allocated: self.allocated,
            budget_account_id: self.budget_account_id,
        };
        imp_db!(categories);
        diesel::insert_into(categories::table)
            .values(&t)
            .execute(conn)
            .expect("Error saving new category");
        categories
            .order(id.desc())
            .limit(1)
            .first::<Category>(conn)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budgeting::transaction::TransactionBuilder;
    use crate::establish_connection;
    use crate::tests::DbDropper;

    #[test]
    fn new_transaction_category_with_transactions() {
        let mut d = DbDropper::new();
        let mut conn = d.conn();
        let mut category = CategoryBuilder::new(1, "Testing")
            .allocated(6000.)
            .done(conn);
        category
            .new_expense(3000.)
            .payee("Payee 1")
            .note("Test Note Payee 1")
            .done(conn);
        category
            .new_expense(300.)
            .payee("Payee 3")
            .note("Test Note Payee 3")
            .done(conn);
        category
            .new_income(500.)
            .payee("Payee 4")
            .note("Test Note Payee 4")
            .done(conn);
        category
            .new_income(600.)
            .payee("Payee 2")
            .note("Test Note Payee 2")
            .done(conn);
        let c = category.transactions(conn);
        println!("{:#?}", c);
        assert_eq!(category.expense(conn), -3300.);
        assert_eq!(category.income(conn), 1100.);
        // we have not funded this category, so only transactions are available
        assert_eq!(category.balance(conn), 1100. - 3300.);
        category
            .new_expense(1000.)
            .payee("Payee 5")
            .note("Test Note Payee 5")
            .done(conn);
        assert_eq!(category.expense(conn), -4300.);
        Category::delete(&mut conn, 1);
    }
}
