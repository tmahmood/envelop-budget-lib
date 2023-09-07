use crate::schema::budget_accounts;
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use diesel::result::Error as DieselError;

use crate::budgeting::budgeting_errors::BudgetingErrors;
use crate::{current_date, DbConnection};

/// Budget is used to store all the transaction categories and store their details in a file
#[derive(Default, Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct BudgetAccount {
    id: i32,
    filed_as: String,
    date_created: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = budget_accounts)]
pub struct BudgetAccountForm {
    pub filed_as: Option<String>,
}

pub struct BudgetAccountBuilder {
    filed_as: String,
    date_created: Option<NaiveDateTime>,
    conn: DbConnection,
}

impl BudgetAccountBuilder {
    pub fn new(conn: DbConnection, filed_as: &str) -> BudgetAccountBuilder {
        BudgetAccountBuilder {
            filed_as: filed_as.to_string(),
            date_created: None,
            conn,
        }
    }

    pub fn date_created(&mut self, date_created: NaiveDateTime) -> &mut Self {
        self.date_created = Some(date_created);
        self
    }

    pub fn build(&mut self) -> Result<BudgetAccount, BudgetingErrors> {
        let new_budget = NewBudgetAccount {
            filed_as: &self.filed_as,
            date_created: self.date_created.unwrap_or_else(current_date),
        };
        let mut _conn = (*self.conn).borrow_mut();
        let conn = _conn.deref_mut();
        save_model!(conn, budget_accounts, new_budget, BudgetAccount).map_err(|e|
            match e {
                DieselError::NotFound => BudgetingErrors::BudgetAccountNotFound,
                e => BudgetingErrors::UnspecifiedDatabaseError(e)
            }
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = budget_accounts)]
pub struct NewBudgetAccount<'a> {
    filed_as: &'a str,
    date_created: NaiveDateTime,
}

impl BudgetAccount {
    pub fn date_created(&self) -> NaiveDateTime {
        self.date_created
    }

    pub fn set_date_created(&mut self, date_created: NaiveDateTime) {
        self.date_created = date_created;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn filed_as(&self) -> String {
        self.filed_as.to_string()
    }
}

pub struct BudgetAccountModel {
    conn: DbConnection,
    budget_account: BudgetAccount,
}

impl BudgetAccountModel {
    pub(crate) fn update(
        conn: &mut SqliteConnection,
        budget_account_id: i32,
        _filed_as: Option<String>,
    ) -> Result<usize, BudgetingErrors> {
        imp_db!(budget_accounts);
        let r = diesel::update(budget_accounts.find(budget_account_id))
            .set(&BudgetAccountForm {
                filed_as: _filed_as,
            })
            .execute(conn);
        match r {
            Ok(a) => Ok(a),
            Err(_) => Err(BudgetingErrors::BudgetAccountUpdateFailed),
        }
    }
}

impl BudgetAccountModel {
    pub fn new(conn: DbConnection, budget_account: BudgetAccount) -> Self {
        Self {
            conn,
            budget_account,
        }
    }

    pub(crate) fn find_all(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<BudgetAccount>, BudgetingErrors> {
        imp_db!(budget_accounts);
        match budget_accounts.load::<BudgetAccount>(conn) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(BudgetingErrors::UnspecifiedDatabaseError(e))
            }
        }
    }

    pub fn load(conn: DbConnection, bid: i32) -> Result<BudgetAccountModel, BudgetingErrors> {
        imp_db!(budget_accounts);
        let res = budget_accounts
            .find(bid)
            .first::<BudgetAccount>((*conn).borrow_mut().deref_mut());
        match res {
            Ok(c) => Ok(BudgetAccountModel::new(conn, c)),
            Err(diesel::result::Error::NotFound) => Err(BudgetingErrors::BudgetAccountNotFound),
            Err(e) => Err(BudgetingErrors::UnspecifiedDatabaseError(e)),
        }
    }

    pub fn load_by_id(
        conn: &mut SqliteConnection,
        bid: i32,
    ) -> Result<BudgetAccount, BudgetingErrors> {
        imp_db!(budget_accounts);
        match budget_accounts.find(bid).first::<BudgetAccount>(conn) {
            Ok(c) => Ok(c),
            Err(diesel::result::Error::NotFound) => Err(BudgetingErrors::BudgetAccountNotFound),
            Err(e) => Err(BudgetingErrors::UnspecifiedDatabaseError(e)),
        }
    }

    pub fn load_by_name(
        conn: &mut SqliteConnection,
        budget_account: &str,
    ) -> Result<BudgetAccount, BudgetingErrors> {
        imp_db!(budget_accounts);
        match budget_accounts
            .filter(filed_as.eq(budget_account))
            .first::<BudgetAccount>(conn)
        {
            Ok(c) => Ok(c),
            Err(diesel::result::Error::NotFound) => Err(BudgetingErrors::BudgetAccountNotFound),
            Err(e) => Err(BudgetingErrors::UnspecifiedDatabaseError(e)),
        }
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
