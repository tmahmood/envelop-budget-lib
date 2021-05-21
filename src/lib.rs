///
/// # Envelope budgeting
/// * We create categories and have budget for every category
/// * We can not spend more money then what we have allocated in that category
/// * We can transfer money from one category to other
/// * Save it to a binary file initially, (The Pragmatic Programmer),
///
///
pub mod budgeting;
use budgeting::budget::Budget;
use budgeting::expense_category::ExpenseCategory;


#[cfg(test)]
mod tests {
    use super::*;


}