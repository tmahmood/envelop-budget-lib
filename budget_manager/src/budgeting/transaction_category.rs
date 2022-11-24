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
pub struct CategoryBuilder<'a> {
    name: String,
    allocated: f64,
    budget_account_id: i32,
    conn: &'a mut SqliteConnection,
}

impl Category {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn allocated(&self) -> f64 {
        self.allocated.into()
    }

    pub fn budget_account_id(&self) -> i32 {
        self.budget_account_id
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl<'a> CategoryBuilder<'a> {
    pub fn new(conn: &'a mut SqliteConnection, budget_account_id: i32, name: &str) -> Self {
        Self {
            name: name.to_string(),
            allocated: 0.0,
            budget_account_id,
            conn,
        }
    }

    pub fn allocated(&mut self, allocated: f64) -> &mut Self {
        self.allocated = allocated;
        self
    }

    // put the transaction category details together and save to database, returned the saved transaction
    pub fn done(&mut self) -> Category {
        let mut t = NewTransactionCategory {
            name: self.name.as_str(),
            allocated: self.allocated,
            budget_account_id: self.budget_account_id,
        };
        imp_db!(categories);
        diesel::insert_into(categories::table)
            .values(&t)
            .execute(self.conn)
            .expect("Error saving new category");
        categories
            .order(id.desc())
            .limit(1)
            .first::<Category>(self.conn)
            .unwrap()
    }
}

pub struct CategoryModel<'a> {
    conn: &'a mut SqliteConnection,
    category: Category,
}

impl<'a> CategoryModel<'a> {
    pub fn new(conn: &'a mut SqliteConnection, category: Category) -> Self {
        Self { conn, category }
    }

    pub fn category(&mut self) -> Category {
        imp_db!(categories);
        categories
            .find(self.category.id)
            .first::<Category>(self.conn)
            .unwrap()
    }

    fn new_transaction(&mut self, amount: f64, income: bool) -> TransactionBuilder {
        let mut tb = if income {
            TransactionBuilder::new_income(self.conn, amount)
        } else {
            TransactionBuilder::new_expense(self.conn, amount)
        };
        tb.category(self.category.id);
        tb
    }

    pub(crate) fn new_expense(&mut self, amount: f64) -> TransactionBuilder {
        self.new_transaction(amount, false)
    }

    pub(crate) fn new_income(&mut self, amount: f64) -> TransactionBuilder {
        self.new_transaction(amount, true)
    }

    pub(crate) fn delete(conn: &mut SqliteConnection, id: i32) -> usize {
        imp_db!(categories);
        diesel::delete(categories.filter(id.eq(&id)))
            .execute(conn)
            .expect("Error deleting transaction category")
    }

    pub(crate) fn load(conn: &mut SqliteConnection, id: i32) -> QueryResult<Category> {
        imp_db!(categories);
        categories.find(id).first(conn)
    }

    pub(crate) fn transactions(&mut self) -> Vec<Transaction> {
        imp_db!(transactions);
        transactions
            .filter(category_id.eq(self.category.id))
            .load::<Transaction>(self.conn)
            .unwrap()
    }

    pub(crate) fn update_allocation(&mut self, new_allocation: f64) -> QueryResult<usize> {
        imp_db!(categories);
        diesel::update(categories)
            .set(allocated.eq(new_allocation))
            .execute(self.conn)
    }

    pub fn income(&mut self) -> f64 {
        imp_db!(transactions);
        let result_option = Transaction::belonging_to(&self.category)
            .select(sum(amount))
            .filter(amount.gt(0.))
            .first::<Option<f64>>(self.conn);
        return_sum!(result_option)
    }

    pub fn expense(&mut self) -> f64 {
        imp_db!(transactions);
        let result_option = Transaction::belonging_to(&self.category)
            .select(sum(amount))
            .filter(amount.lt(0.))
            .first::<Option<f64>>(self.conn);
        return_sum!(result_option)
    }

    pub fn allocated(&self) -> f64 {
        self.category.allocated()
    }

    pub fn balance(&mut self) -> f64 {
        imp_db!(transactions);
        let result_option = Transaction::belonging_to(&self.category)
            .select(sum(amount))
            .first::<Option<f64>>(self.conn);
        return_sum!(result_option)
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
        let mut conn = &mut d.conn();
        let mut category = CategoryBuilder::new(conn, 1, "Testing")
            .allocated(6000.)
            .done();
        let mut cm = CategoryModel::new(conn, category);
        cm.new_expense(3000.)
            .payee("Payee 1")
            .note("Test Note Payee 1")
            .done();
        cm.new_expense(300.)
            .payee("Payee 3")
            .note("Test Note Payee 3")
            .done();
        cm.new_income(500.)
            .payee("Payee 4")
            .note("Test Note Payee 4")
            .done();
        cm.new_income(600.)
            .payee("Payee 2")
            .note("Test Note Payee 2")
            .done();
        let c = cm.transactions();
        println!("{:#?}", c);
        assert_eq!(cm.expense(), -3300.);
        assert_eq!(cm.income(), 1100.);
        // we have not funded this category, so only transactions are available
        assert_eq!(cm.balance(), 1100. - 3300.);
        cm.new_expense(1000.)
            .payee("Payee 5")
            .note("Test Note Payee 5")
            .done();
        assert_eq!(cm.expense(), -4300.);
        CategoryModel::delete(&mut conn, 1);
    }
}
