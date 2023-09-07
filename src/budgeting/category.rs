use crate::budgeting::budgeting_errors::BudgetingErrors;
use crate::budgeting::transaction::{Transaction, TransactionModel, TransactionType};
use crate::schema::categories;
use crate::DbConnection;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(
    Debug, PartialOrd, PartialEq, Serialize, Deserialize, Default, Clone, Queryable, Identifiable,
)]
#[diesel(table_name = categories)]
pub struct Category {
    id: i32,
    name: String,
    allocated: f64,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewTransactionCategory<'a> {
    name: &'a str,
    allocated: f64,
}

#[derive(AsChangeset)]
#[diesel(table_name = categories)]
pub struct CategoryForm {
    pub name: Option<String>,
    pub allocated: Option<f64>,
}

/// Only way to create transaction category.
/// as we need to maintain the budget_account_id
pub struct CategoryBuilder {
    name: String,
    allocated: f64,
    conn: DbConnection,
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
}

impl CategoryBuilder {
    pub(crate) fn new(conn: DbConnection, name: &str) -> Self {
        Self {
            name: name.to_string(),
            allocated: 0.0,
            conn,
        }
    }

    pub fn allocated(&mut self, allocated: f64) -> &mut Self {
        self.allocated = allocated;
        self
    }

    // put the transaction category details together and save to database, returned the new category
    pub fn done(&self) -> Result<Category, BudgetingErrors> {
        let mut t = NewTransactionCategory {
            name: self.name.as_str(),
            allocated: self.allocated,
        };
        imp_db!(categories);
        let category = save_model!(
            gc!(self.conn), categories, t, Category
        )?;
        Ok(category)
    }
}

pub struct CategoryModel {
    conn: DbConnection,
    category: Category,
}

impl CategoryModel {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>, category: Category) -> Self {
        Self { conn, category }
    }

    pub(crate) fn update(
        conn: &mut SqliteConnection,
        category_id: i32,
        new_name: Option<String>,
        new_allocated: Option<f64>,
    ) -> Result<usize, BudgetingErrors> {
        imp_db!(categories);
        let r = diesel::update(categories.find(category_id))
            .set(&CategoryForm {
                name: new_name,
                allocated: new_allocated,
            })
            .execute(conn);
        match r {
            Ok(a) => Ok(a),
            Err(_) => Err(BudgetingErrors::CategoryUpdateFailed),
        }
    }

    pub(crate) fn find_by_name(
        conn: &mut SqliteConnection,
        _name: &str,
    ) -> Result<Category, BudgetingErrors> {
        imp_db!(categories);
        match categories.filter(name.eq(_name)).first::<Category>(conn) {
            Ok(c) => Ok(c),
            Err(diesel::result::Error::NotFound) => Err(BudgetingErrors::CategoryNotFound),
            Err(e) => Err(BudgetingErrors::UnspecifiedDatabaseError(e)),
        }
    }

    pub(crate) fn delete(
        conn: &mut SqliteConnection,
        category_id: i32,
    ) -> Result<usize, BudgetingErrors> {
        imp_db!(categories);
        let r = diesel::delete(categories.filter(id.eq(&category_id))).execute(conn);
        match r {
            Ok(how_many) => Ok(how_many),
            Err(_) => Err(BudgetingErrors::CategoryDeleteFailed),
        }
    }

    pub(crate) fn load(
        conn: Rc<RefCell<SqliteConnection>>,
        cid: i32,
    ) -> Result<CategoryModel, BudgetingErrors> {
        imp_db!(categories);
        let res = categories
            .find(cid)
            .first::<Category>(conn.borrow_mut().deref_mut());
        match res {
            Ok(c) => Ok(CategoryModel::new(conn, c)),
            Err(diesel::result::Error::NotFound) => Err(BudgetingErrors::CategoryNotFound),
            Err(e) => Err(BudgetingErrors::UnspecifiedDatabaseError(e)),
        }
    }

    pub(crate) fn update_allocation(&mut self, new_allocation: f64) -> QueryResult<usize> {
        imp_db!(categories);
        diesel::update(categories)
            .set(allocated.eq(new_allocation))
            .execute(gc!(self.conn))
    }

    pub(crate) fn _balance(
        conn: &mut SqliteConnection,
        _budget_account_id: Option<i32>,
        category: &str,
    ) -> Result<f64, BudgetingErrors> {
        let c = CategoryModel::find_by_name(conn, category)?;
        Ok(TransactionModel::total(conn, None, Some(c.id), None))
    }
    pub fn category(&mut self) -> Category {
        imp_db!(categories);
        let c = categories
            .find(self.category.id)
            .first::<Category>(gc!(self.conn))
            .unwrap();
        self.category = c;
        self.category.clone()
    }

    pub fn allocated(&self) -> f64 {
        self.category.allocated()
    }

    pub fn find_by_transfer_type(&mut self, transfer_type: TransactionType) -> f64 {
        TransactionModel::total(
            gc!(self.conn),
            Some(transfer_type),
            Some(self.category.id),
            None,
        )
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

    pub fn balance(&mut self) -> f64 {
        TransactionModel::total(
            gc!(self.conn),
            None,
            Some(self.category.id),
            None,
        )
    }

    pub fn transactions(&mut self) -> Vec<Transaction> {
        TransactionModel::find_all(
            gc!(self.conn),
            Some(self.category.id),
            None,
        )
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
                _ => BudgetingErrors::UnspecifiedDatabaseError(value),
            },
            _ => BudgetingErrors::UnspecifiedDatabaseError(value),
        };
    }
}
