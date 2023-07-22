use crate::budgeting::Budgeting;
use crate::run_migrations;
use diesel::{Connection, RunQueryDsl, SqliteConnection, TextExpressionMethods};
use rand::Rng;

// test all the possible things!
pub const DEFAULT_ID: i32 = 1;
pub const BILL_ID: i32 = 2;
pub const TRAVEL_ID: i32 = 3;
pub const BILLS: f64 = 2000.;
pub const TRAVEL: f64 = 3000.;
pub const UNUSED: f64 = 10000.;
pub const INITIAL: f64 = 15000.;

pub fn generate_random_str(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn new_budget_using_budgeting(budgeting: &mut Budgeting) {
    budgeting
        .new_budget("wallet", 5000.)
        .expect("Error creating new budget");
    budgeting
        .new_budget("main", 10000.)
        .expect("Error creating new budget");
    budgeting.create_category("Bills", BILLS, true).unwrap();
    budgeting.create_category("Travel", TRAVEL, true).unwrap();
}

pub fn memory_db() -> SqliteConnection {
    let mut sqlite = SqliteConnection::establish(":memory:")
        .unwrap_or_else(|_| panic!("Failed to load memory db"));
    run_migrations(&mut sqlite).expect("Failed to run migrations");
    sqlite
}
