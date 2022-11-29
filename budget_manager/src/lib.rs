use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::{debug, error, info, warn};
use std::env;
use std::rc::Rc;

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
    }};
}

macro_rules! return_sum {
    ($query_result: expr) => {
        match $query_result {
            Ok(Some(n)) => n,
            _ => 0.0,
        }
    }
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
pub mod budget_account_op;
pub mod budgeting;
pub mod schema;
#[cfg(test)]
mod test_helpers;
pub mod transaction_op;

/// This should be used whenever date time is needed
pub fn current_date() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn parse_date(date_created: &str) -> NaiveDateTime {
    let f1 = "%Y-%m-%d %H:%M:%S%.f";
    let f2 = "%Y-%m-%d %H:%M:%S";
    let f3 = "%Y-%m-%d";
    let k = NaiveDateTime::parse_from_str(date_created, f1);
    if let Ok(n) = k {
        return n;
    }
    let k = NaiveDateTime::parse_from_str(date_created, f2);
    if let Ok(n) = k {
        return n;
    }
    let k = NaiveDateTime::parse_from_str(date_created, f3);
    if let Ok(n) = k {
        return n;
    }
    error!("Invalid date provided");
    NaiveDateTime::default()
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = "sqlite://db.sqlite";
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budgeting::budget_account::{BudgetAccount, BudgetAccountBuilder};

    use crate::budgeting::category::CategoryBuilder;
    use crate::budgeting::Budgeting;
    use crate::test_helpers;
    use crate::test_helpers::DbDropper;

    // test all the possible things!

    pub const DEFAULT_ID: i32 = 1;
    pub const BILL_ID: i32 = 2;
    pub const TRAVEL_ID: i32 = 3;
    pub const BILLS: f64 = 2000.;
    pub const TRAVEL: f64 = 3000.;
    pub const UNUSED: f64 = 5000.;
    pub const INITIAL: f64 = 10000.;

    #[test]
    fn transfer_should_not_be_counted_as_income_or_expense() {
        let _dd = DbDropper::new();
        let mut blib = Budgeting::new();
        test_helpers::new_budget_using_budgeting(&mut blib);
        assert_eq!(blib.get_category_model("Bills").income(), 0.);
        assert_eq!(blib.get_category_model(DEFAULT_CATEGORY).expense(), 0.);
    }

    #[test]
    fn creating_budget_and_adding_transaction() {
        let _db = DbDropper::new();
        let mut blib = Budgeting::new();
        test_helpers::new_budget_using_budgeting(&mut blib);
        // initial + allocation to bills * 2 + allocation to travel * 2
        assert_eq!(blib.transactions().len(), 5);
        assert_eq!(blib.actual_total_balance(), INITIAL);
        assert_eq!(blib.uncategorized_balance(), INITIAL - (BILLS + TRAVEL));
        // now let's do some transactions
        {
            let mut travel = blib.new_transaction_to_category("Travel");
            travel.expense(1000.).payee("Some").note("Other").done();
            travel.expense(1300.).payee("Uber").note("Other").done();
            travel.income(400.).payee("Other").note("Other").done();
        }
        {
            let mut bills = blib.new_transaction_to_category("Bills");
            bills.expense(300.).payee("Some").note("Other").done();
        }
        {
            let mut default = blib.new_transaction_to_category(DEFAULT_CATEGORY);
            default.expense(1000.).payee("Other").note("Other").done();
            default.income(5000.).payee("Other").note("Other").done();
        }
        // check total balance
        assert_eq!(blib.actual_total_balance(), INITIAL - 3600. + 5400.);
        assert_eq!(
            blib.category_balance("Travel"),
            TRAVEL - 1000. - 1300. + 400.
        );
        assert_eq!(blib.category_balance("Bills"), BILLS - 300.);
        assert_eq!(
            blib.category_balance(DEFAULT_CATEGORY),
            INITIAL + 5000. - 1000. - 3000. - 2000.
        );
    }

    #[test]
    fn transfer_fund_from_balance() {
        let dd = DbDropper::new();
        let mut blib = Budgeting::new();
        test_helpers::new_budget_using_budgeting(&mut blib);
        assert!(blib.transfer_fund("Bills", "Travel", BILLS).is_ok());
        //
        assert_eq!(blib.category_balance("Bills"), 0.);
        assert_eq!(blib.category_balance("Travel"), BILLS + TRAVEL);
        //
        let v = blib.get_category_model("Bills").transactions();
        println!("{:?}", v);

        //
        assert_eq!(blib.get_category_model("Bills").expense(), 0.);
        assert_eq!(blib.get_category_model("Travel").income(), 0.);
    }
}
