use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use log::{debug, error, info, warn};
use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budgeting::budget_account::{BudgetAccount, BudgetAccountBuilder};
    use crate::budgeting::transaction;
    use crate::budgeting::transaction::Transaction;
    use crate::budgeting::transaction_category::TransactionCategoryBuilder;
    use rand::Rng;
    use std::collections::BTreeMap;

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
                use crate::schema::transaction_categories::dsl::*;
                diesel::delete(transaction_categories)
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
        b.create_and_allocate(conn, "Bills", BILLS).unwrap();
        b.create_and_allocate(conn, "Travel", TRAVEL).unwrap();
        b
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
        // now let's made some transactions
        let travel = budget.find_category(conn, "Travel").unwrap();
        let bills = budget.find_category(conn, "Bills").unwrap();
        let default = budget.find_category(conn, DEFAULT_CATEGORY).unwrap();
        travel
            .new_expense(1000.)
            .payee("Some")
            .note("Other")
            .done(conn);
        travel
            .new_expense(1300.)
            .payee("Uber")
            .note("Other")
            .done(conn);
        travel
            .new_income(400.)
            .payee("Other")
            .note("Other")
            .done(conn);
        bills
            .new_expense(300.)
            .payee("Some")
            .note("Other")
            .done(conn);
        default
            .new_expense(1000.)
            .payee("Other")
            .note("Other")
            .done(conn);
        default
            .new_income(5000.)
            .payee("Other")
            .note("Other")
            .done(conn);
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

    #[test]
    fn moving_transaction_from_one_category_to_another() {
        let mut d = DbDropper::new();
        let mut conn = d.conn();
        let mut budget = new_budget(&mut conn);
        // two transaction without categories
        budget
            .find_category(conn, DEFAULT_CATEGORY)
            .unwrap()
            .new_expense(1000.)
            .payee("Other")
            .note("Other")
            .done(&mut conn);
        budget
            .find_category(conn, DEFAULT_CATEGORY)
            .unwrap()
            .new_expense(5000.)
            .payee("Other")
            .note("Other")
            .done(&mut conn);
        // we want to move the new transactions to travel
        //budget.move_transaction(t1, "Travel");
    }

    pub fn generate_random_str(length: usize) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(&rand::distributions::Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}
