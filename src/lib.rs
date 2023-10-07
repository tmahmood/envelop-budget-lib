use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use dotenvy::dotenv;
use tracing::error;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::str::FromStr;

pub use diesel::SqliteConnection;

type DbConnection = Rc<RefCell<SqliteConnection>>;
pub type Currency = f64;
pub const DEFAULT_CATEGORY: &str = "Unallocated";

macro_rules! gc {
    ($conn: expr) => {
        $conn.borrow_mut().deref_mut()
    };
}

macro_rules! save_model {
    ($conn: expr, $schema: ident, $model: ident, $mtype: ident) => {{
        use crate::schema::$schema;
        use crate::schema::$schema::dsl::*;
        use diesel::prelude::*;
        diesel::insert_into($schema::table)
            .values($model)
            .execute($conn)?;
        $schema
            .order(id.desc())
            .limit(1)
            .first::<$mtype>($conn)
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

pub(crate) mod m {
    use diesel::sql_function;
    sql_function! {fn lower(a: diesel::sql_types::Text) -> diesel::sql_types::Text}
}

///
/// # Envelope budgeting
/// * We create categories and have budget for every category
/// * We can not spend more money then what we have allocated in that category
/// * We can transfer money from one category to other
///
pub mod budgeting;
pub mod schema;
pub(crate) mod speller;
#[cfg(test)]
mod test_helpers;

/// This should be used whenever date time is needed
pub fn current_date() -> NaiveDateTime {
    Local::now().naive_local()
}

#[tracing::instrument]
pub fn parse_date(date_created: &str) -> NaiveDateTime {
    let formats = vec![
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
    ];
    for format in formats {
        if let Ok(n) = NaiveDateTime::parse_from_str(
            date_created, format
        ) {
            return n;
        }
    }
    if let Ok(n) = NaiveDate::parse_from_str(date_created, "%Y-%m-%d") {
        let s = NaiveTime::from_str("00:00:00").unwrap();
        return NaiveDateTime::new(
            n, s
        );
    }
    error!("Invalid date provided {}", date_created);
    NaiveDateTime::default()
}

/// creates database connection
// TODO: should return a result object instead of connection for error handling
pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| {
            error!(
                "Error connecting to {}, Please check your DATABASE_URL env variable",
                database_url);
            panic!("Exiting due to unrecoverable error")
        })
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
