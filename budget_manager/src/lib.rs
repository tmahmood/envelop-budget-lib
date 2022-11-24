use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use log::{debug, error, info, warn};
use std::env;
use std::rc::Rc;

pub mod transaction_op;

pub const DEFAULT_CATEGORY: &str = "Unallocated";

macro_rules! save_model {
    ($conn: ident, $t: ident, $model: ident, $mtype: ident) => {{
        use crate::schema::$t;
        use crate::schema::$t::dsl::*;
        use diesel::prelude::*;
        diesel::insert_into($t::table)
            .values($model)
            .execute($conn)
            .expect("Error saving");
        $t.order(id.desc()).limit(1).first::<$mtype>($conn)
    };};
}

macro_rules! return_sum {
    ($query_result: expr) => {
        if $query_result.is_ok() {
            let option = $query_result.unwrap();
            if option.is_none() {
                return 0.0;
            }
            return option.unwrap();
        } else {
            return 0.0;
        }
    };
}

macro_rules! imp_db {
    ($t: ident) => {
        use crate::schema::$t;
        use crate::schema::$t::dsl::*;
        use diesel::prelude::*;
    };
}
///
/// # Envelope budgeting
/// * We create categories and have budget for every category
/// * We can not spend more money then what we have allocated in that category
/// * We can transfer money from one category to other
///
pub mod budgeting;
pub mod schema;

use crate::budgeting::budget_account;
use crate::budgeting::budget_account::BudgetAccount;
use crate::transaction_op::TransactionOp;
use budgeting::prelude::*;

/// This should be used whenever date time is needed
pub fn current_date() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn parse_date(date_created: &str) -> NaiveDateTime {
    let f1 = "%Y-%m-%d %H:%M:%S%.f";
    let f2 = "%Y-%m-%d %H:%M:%S";
    let f3 = "%Y-%m-%d";
    let k = NaiveDateTime::parse_from_str(&date_created, f1);
    if k.is_ok() {
        return k.unwrap();
    }
    let k = NaiveDateTime::parse_from_str(&date_created, f2);
    if k.is_ok() {
        return k.unwrap();
    }
    let k = NaiveDateTime::parse_from_str(&date_created, f3);
    if k.is_ok() {
        return k.unwrap();
    }
    error!("Invalid date provided");
    NaiveDateTime::default()
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub(crate) fn new_transaction_to_category<'a>(
    conn: &'a mut SqliteConnection,
    budget: &'a mut BudgetAccount,
    category: &'a str,
) -> TransactionOp<'a> {
    TransactionOp::new(conn, budget, category)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budgeting::budget_account::{BudgetAccount, BudgetAccountBuilder};
    use crate::budgeting::transaction;
    use crate::budgeting::transaction::Transaction;
    use crate::budgeting::transaction_category::CategoryBuilder;
    use rand::Rng;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    // test all the possible things!

    pub const DEFAULT_ID: i32 = 1;
    pub const BILL_ID: i32 = 2;
    pub const TRAVEL_ID: i32 = 3;
    pub const BILLS: f64 = 2000.;
    pub const TRAVEL: f64 = 3000.;
    pub const UNUSED: f64 = 5000.;
    pub const INITIAL: f64 = 10000.;

    pub struct DbDropper {
        conn: SqliteConnection,
    }

    impl DbDropper {
        pub fn new() -> Self {
            DbDropper {
                conn: establish_connection(),
            }
        }

        pub fn conn(&mut self) -> &mut SqliteConnection {
            &mut self.conn
        }
    }

    impl Drop for DbDropper {
        fn drop(&mut self) {
            let mut num_deleted = 0;
            num_deleted += {
                use crate::schema::budget_accounts::dsl::*;
                diesel::delete(budget_accounts)
                    .execute(self.conn())
                    .expect("Error deleting budget accounts")
            };
            num_deleted += {
                use crate::schema::categories::dsl::*;
                diesel::delete(categories)
                    .execute(self.conn())
                    .expect("Error deleting transaction categories")
            };
            num_deleted += {
                use crate::schema::transactions::dsl::*;
                diesel::delete(transactions)
                    .execute(self.conn())
                    .expect("Error deleting transactions")
            };
            println!("deleted: {}", num_deleted);
        }
    }

    pub fn new_budget(conn: &mut SqliteConnection) -> BudgetAccount {
        let mut b = BudgetAccountBuilder::new("main").balance(10000.).done(conn);
        // this will not change the actual balance
        b.create_category_and_allocate(conn, "Bills", BILLS).unwrap();
        b.create_category_and_allocate(conn, "Travel", TRAVEL).unwrap();
        b
    }

    #[test]
    fn transaction_op_struct_handles_full_transaction_details() {
        let mut db = DbDropper::new();
        let mut conn = db.conn();
        let mut budget = new_budget(conn);
        let mut d = new_transaction_to_category(conn, &mut budget, "Travel");
        d.income(1000.).payee("Some").note("Other").done();
        d.expense(2000.).payee("Some").note("Other").done();
        assert_eq!(budget.category_balance(conn, "Travel"), 2000.);
    }

    #[test]
    fn creating_budget_and_adding_transaction() {
        let mut db = DbDropper::new();
        let mut conn = db.conn();
        let mut budget = new_budget(conn);
        // initial + allocation to bills * 2 + allocation to travel * 2
        assert_eq!(budget.transactions(conn).len(), 5);
        assert_eq!(budget.actual_total_balance(conn), INITIAL);
        assert_eq!(
            budget.uncategorized_balance(conn),
            INITIAL - (BILLS + TRAVEL)
        );
        // now let's do some transactions
        {
            let mut travel = new_transaction_to_category(conn, &mut budget, "Travel");
            travel.expense(1000.).payee("Some").note("Other").done();
            travel.expense(1300.).payee("Uber").note("Other").done();
            travel.income(400.).payee("Other").note("Other").done();
        }
        {
            let mut bills = new_transaction_to_category(conn, &mut budget, "Bills");
            bills.expense(300.).payee("Some").note("Other").done();
        }
        {
            let mut default = new_transaction_to_category(conn, &mut budget, DEFAULT_CATEGORY);
            default.expense(1000.).payee("Other").note("Other").done();
            default.income(5000.).payee("Other").note("Other").done();
        }
        // check total balance
        assert_eq!(budget.actual_total_balance(conn), INITIAL - 3600. + 5400.);
        assert_eq!(
            budget.category_balance(conn, "Travel"),
            TRAVEL - 1000. - 1300. + 400.
        );
        assert_eq!(budget.category_balance(conn, "Bills"), BILLS - 300.);
        assert_eq!(
            budget.category_balance(conn, DEFAULT_CATEGORY),
            INITIAL + 5000. - 1000. - 3000. - 2000.
        );
    }

    pub fn generate_random_str(length: usize) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(&rand::distributions::Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}
