use crate::budgeting::budget_account::BudgetAccount;
use crate::budgeting::budgeting_errors::BudgetingErrors;
use crate::budgeting::category;
use crate::budgeting::transaction::{Transaction, TransactionType};
use crate::schema::categories;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
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

#[derive(AsChangeset)]
#[diesel(table_name = categories)]
pub struct CategoryForm {
    pub name: Option<String>,
    pub allocated: Option<f64>,
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

    pub fn name_c(&self) -> &str {
        self.name.as_str()
    }

    pub fn allocated(&self) -> f64 {
        self.allocated
    }

    pub fn budget_account_id(&self) -> i32 {
        self.budget_account_id
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_allocated(&mut self, allocated: f64) {
        self.allocated = allocated;
    }
    pub fn set_budget_account_id(&mut self, budget_account_id: i32) {
        self.budget_account_id = budget_account_id;
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
    pub fn done(&mut self) -> Result<Category, BudgetingErrors> {
        let mut t = NewTransactionCategory {
            name: self.name.as_str(),
            allocated: self.allocated,
            budget_account_id: self.budget_account_id,
        };
        imp_db!(categories);
        let r = diesel::insert_into(categories::table)
            .values(&t)
            .execute(self.conn)?;
        let r = categories
            .order(id.desc())
            .limit(1)
            .first::<Category>(self.conn)?;
        Ok(r)
    }
}

impl From<diesel::result::Error> for BudgetingErrors {
    fn from(value: diesel::result::Error) -> Self {
        return match value {
            diesel::result::Error::NotFound => BudgetingErrors::CategoryNotFound,
            DatabaseError(e, _) => match e {
                DatabaseErrorKind::UniqueViolation => BudgetingErrors::CategoryAlreadyExists,
                DatabaseErrorKind::ForeignKeyViolation => {
                    BudgetingErrors::FailedToCreateCategory("Foreign Key Violation".to_string())
                }
                _ => BudgetingErrors::UnspecifiedDatabaseError,
            },
            _ => BudgetingErrors::UnspecifiedDatabaseError,
        };
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

    pub fn update(
        conn: &mut SqliteConnection,
        category_id: i32,
        new_name: Option<String>,
        new_allocated: Option<f64>,
    ) -> Result<i32, BudgetingErrors> {
        imp_db!(categories);
        let r = diesel::update(categories.find(category_id))
            .set(&CategoryForm {
                name: new_name,
                allocated: new_allocated,
            })
            .execute(conn);
        match r {
            Ok(_) => Ok(category_id),
            Err(_) => Err(BudgetingErrors::CategoryUpdateFailed),
        }
    }

    pub(crate) fn delete(conn: &mut SqliteConnection, category_id: i32) -> Result<usize, BudgetingErrors> {
        imp_db!(categories);
        let r = diesel::delete(categories.filter(id.eq(&category_id)))
            .execute(conn);
        match r {
            Ok(how_many) => Ok(how_many),
            Err(_) => Err(BudgetingErrors::CategoryDeleteFailed),
        }
    }

    pub fn category(&mut self) -> Category {
        imp_db!(categories);
        categories
            .find(self.category.id)
            .first::<Category>(self.conn)
            .unwrap()
    }


    pub(crate) fn load(conn: &mut SqliteConnection, cid: i32) -> QueryResult<Category> {
        imp_db!(categories);
        categories.find(cid).first::<Category>(conn)
    }

    pub fn transactions(&mut self) -> Vec<Transaction> {
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

    fn find_by_transfer_type(&mut self, transfer_type: TransactionType) -> f64 {
        imp_db!(transactions);
        let result_option: QueryResult<Option<f64>> = Transaction::belonging_to(&self.category)
            .select(sum(amount))
            .filter(transaction_type_id.eq(i32::from(transfer_type)))
            .first::<Option<f64>>(self.conn);
        return_sum!(result_option)
    }

    pub fn income(&mut self) -> f64 {
        self.find_by_transfer_type(TransactionType::Income)
    }

    pub fn expense(&mut self) -> f64 {
        self.find_by_transfer_type(TransactionType::Expense)
    }

    pub fn transfer_in(&mut self) -> f64 {
        self.find_by_transfer_type(TransactionType::TransferIn)
    }

    pub fn transfer_out(&mut self) -> f64 {
        self.find_by_transfer_type(TransactionType::TransferOut)
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
