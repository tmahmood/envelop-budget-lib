use crate::schema::budget_accounts;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

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

    pub fn build(&mut self) -> BudgetAccount {
        let new_budget = NewBudgetAccount {
            filed_as: &self.filed_as,
            date_created: self.date_created.unwrap_or_else(current_date),
        };
        let mut conn = (*self.conn).borrow_mut();
        let b: QueryResult<BudgetAccount> = {
            use crate::schema::budget_accounts;
            use crate::schema::budget_accounts::dsl::*;
            use diesel::prelude::*;
            diesel::insert_into(budget_accounts::table)
                .values(new_budget)
                .execute(conn.deref_mut())
                .expect("Error saving");
            budget_accounts
                .order(id.desc())
                .limit(1)
                .first::<BudgetAccount>(conn.deref_mut())
        };
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
                println!("{e:?}");
                Err(BudgetingErrors::UnspecifiedDatabaseError)
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

    pub fn budget_account(&mut self) -> BudgetAccount {
        imp_db!(budget_accounts);
        let b = budget_accounts
            .find(self.budget_account.id)
            .first::<BudgetAccount>((*self.conn).borrow_mut().deref_mut())
            .unwrap();
        self.budget_account = b;
        self.budget_account.clone()
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
