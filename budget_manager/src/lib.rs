

pub const DEFAULT_CATEGORY: &str = "Unallocated";

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
        budget.new_expense(Some("Bills"), 300., "Some", "Other");
        budget.new_expense(Some("Travel"), 1300., "Now", "Other");
        budget.new_expense(None, 1000., "Other", "Other");
        budget.new_income(None, 5000., "Other", "Other");
        budget.new_income(Some("Travel"), 400., "Other", "Other");
        // check total balance
        assert_eq!(
            budget.total_balance(),
            INITIAL - 2600. + 5400.
        );
        assert_eq!(
            budget.category_balance("Bills"),
            BILLS - 300.
        );
        assert_eq!(
            budget.category_balance("Travel"),
            TRAVEL - 1300. + 400.
        );
        assert_eq!(
            budget.find_or_create_category_by_name("Bills").allocated(),
            BILLS
        );
        budget.update_allocation("Bills", 3000.).unwrap();
        assert_eq!(
            budget.find_or_create_category_by_name("Bills").allocated(),
            3000.
        );
        // where does this 3000. comes from? It has to come from default unallocated category.
        assert_eq!(
            budget.unallocated(),
            4000.
        );
    }
}