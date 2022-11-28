use crate::budgeting::Error::{CategoryAlreadyExists, CategoryNotFound, CategoryUpdateFailed};
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
use std::borrow::BorrowMut;
use std::collections::btree_map::BTreeMap;
use std::hash::Hash;

use crate::budgeting::transaction::{Transaction, TransactionBuilder};
use crate::budgeting::category::{Category, CategoryBuilder};
use crate::budgeting::Error;
use crate::transaction_op::TransactionAddToCategoryOps;
use crate::{current_date, establish_connection, DEFAULT_CATEGORY};

/// Budget is used to store all the transaction categories and store their details in a file
#[derive(Default, Serialize, Deserialize, Queryable, Debug, Identifiable, Clone)]
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
        imp_db!(budget_accounts);
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