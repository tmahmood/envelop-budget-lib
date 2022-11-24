use diesel::{QueryResult, SqliteConnection};
use diesel::dsl::sum;
use crate::budgeting::budget_account::{BudgetAccount, BudgetAccountBuilder, NewBudgetAccount};
use crate::{DEFAULT_CATEGORY, establish_connection};
use crate::budgeting::transaction::Transaction;
use crate::budgeting::transaction_category::Category;

pub mod budget_account;
pub mod storage;
pub mod transaction;
pub mod transaction_category;

pub mod prelude {
    use super::budget_account::*;
    use super::transaction::*;
    use super::transaction_category::*;
}
