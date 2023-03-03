use diesel::{RunQueryDsl, SqliteConnection, TextExpressionMethods};
use rand::Rng;
use crate::budgeting::Budgeting;
use crate::{DEFAULT_CATEGORY, establish_connection};
use crate::tests::{BILLS, TRAVEL};

pub fn generate_random_str(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub struct DbDropper;

impl DbDropper {
    pub(crate) fn conn(&self) -> SqliteConnection {
        establish_connection()
    }
}

impl DbDropper {
    pub fn new() -> Self {
        Self {}
    }
}

impl Drop for DbDropper {
    fn drop(&mut self) {
        clear_database();
    }
}

pub fn clear_database() {
    let mut conn = establish_connection();
    let mut num_deleted = 0;
    num_deleted += {
        use crate::schema::budget_accounts::dsl::*;
        diesel::delete(budget_accounts)
            .execute(&mut conn)
            .expect("Error deleting budget accounts")
    };
    num_deleted += {
        use crate::schema::categories::dsl::*;
        diesel::delete(categories)
            .filter(name.not_like(DEFAULT_CATEGORY))
            .execute(&mut conn)
            .expect("Error deleting transaction categories")
    };
    num_deleted += {
        use crate::schema::transactions::dsl::*;
        diesel::delete(transactions)
            .execute(&mut conn)
            .expect("Error deleting transactions")
    };
    println!("deleted: {}", num_deleted);
}

pub fn new_budget_using_budgeting(budgeting: &mut Budgeting) {
    budgeting.new_budget("wallet", 5000.).expect("Error creating new budget");
    budgeting.new_budget("main", 10000.).expect("Error creating new budget");
    budgeting
        .create_category("Bills", BILLS, true)
        .unwrap();
    budgeting
        .create_category("Travel", TRAVEL, true)
        .unwrap();
}
