

const DEFAULT_CATEGORY: &str = "Unused";

///
/// # Envelope budgeting
/// * We create categories and have budget for every category
/// * We can not spend more money then what we have allocated in that category
/// * We can transfer money from one category to other
///
///
///
///
pub mod budgeting;
use budgeting::budget_account::BudgetAccount;
use budgeting::transaction_category::TransactionCategory;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap};
    use super::*;

    // test all the possible things!

    pub const BILLS: f32 = 2000.;
    pub const TRAVEL: f32 = 3000.;
    pub const UNUSED: f32 = 5000.;
    pub const INITIAL: f32 = 10000.;

    pub fn new_budget() -> BudgetAccount {
        let mut categories = vec![
            ("Bills", BILLS),
            ("Travel", TRAVEL)
        ];
        BudgetAccount::new("main", INITIAL, categories)
    }

    #[test]
    fn creating_budget_and_adding_transaction() {
        let categories = vec![
            ("Bills", BILLS),
            ("Travel", TRAVEL)
        ];
        let mut budget = BudgetAccount::new(
            "main",
            INITIAL,
            categories
        );
        // initial state of the budget
        assert_eq!(budget.all_transactions().len(), 0);
        assert_eq!(budget.total_balance(), INITIAL);
        assert_eq!(budget.unallocated(), INITIAL - (BILLS + TRAVEL));
        // now let's made some transactions
        budget.new_expense(Some("Bills"), 300.);
        budget.new_expense(Some("Travel"), 1300.);
        budget.new_expense(None, 1000.);
        // check total balance
        assert_eq!(
            budget.total_balance(),
            INITIAL - 300. - 1300. - 1000.
        );
        assert_eq!(
            budget.category_balance("Bills"),
            BILLS - 300.
        );
        assert_eq!(
            budget.category_balance("Travel"),
            TRAVEL - 1300.
        );
    }
}