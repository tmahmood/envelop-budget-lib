use std::env;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::{error, info, warn};

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
#[cfg(test)]
mod test_helpers;

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
    let database_url = env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&database_url)
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

