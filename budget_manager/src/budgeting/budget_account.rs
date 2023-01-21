use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use crate::schema::budget_accounts;

use crate::budgeting::budgeting_errors::BudgetingErrors;
use crate::current_date;

/// Budget is used to store all the transaction categories and store their details in a file
#[derive(Default, Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct BudgetAccount {
    id: i32,
    filed_as: String,
    date_created: NaiveDateTime,
}

pub struct BudgetAccountBuilder<'a> {
    filed_as: String,
    date_created: Option<NaiveDateTime>,
    conn: &'a mut SqliteConnection,
}

impl<'a> BudgetAccountBuilder<'a> {
    pub fn new(conn: &'a mut SqliteConnection, filed_as: &str) -> BudgetAccountBuilder<'a> {
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

    pub fn build(&mut self) -> BudgetAccount {
        let conn = self.conn.borrow_mut();
        let new_budget = NewBudgetAccount {
            filed_as: &self.filed_as,
            date_created: self.date_created.unwrap_or_else(current_date),
        };
        let b: QueryResult<BudgetAccount> =
            save_model!(conn, budget_accounts, new_budget, BudgetAccount);
        if b.is_err() {
            panic!("Failed to create budget account");
        }
        b.unwrap()
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

pub struct BudgetAccountModel<'a> {
    conn: &'a mut SqliteConnection,
    budget_account: BudgetAccount,
}

impl<'a> BudgetAccountModel<'a> {
    pub fn new(conn: &'a mut SqliteConnection, budget_account: BudgetAccount) -> Self {
        Self {
            conn,
            budget_account,
        }
    }

    pub fn load(
        conn: &mut SqliteConnection,
        bid: i32,
    ) -> Result<BudgetAccountModel, BudgetingErrors> {
        imp_db!(budget_accounts);
        match budget_accounts.find(bid).first::<BudgetAccount>(conn) {
            Ok(c) => Ok(BudgetAccountModel::new(conn, c)),
            Err(diesel::result::Error::NotFound) => Err(BudgetingErrors::BudgetAccountNotFound),
            Err(_) => Err(BudgetingErrors::UnspecifiedDatabaseError),
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
            Err(_) => Err(BudgetingErrors::UnspecifiedDatabaseError),
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
            Err(_) => Err(BudgetingErrors::UnspecifiedDatabaseError),
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
