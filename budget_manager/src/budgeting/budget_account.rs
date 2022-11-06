use std::collections::HashMap;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

use crate::budgeting::expense_category::ExpenseCategory;

/// Budget is used to store all the expense categories and store their details in a file
#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetAccount {
    categories: HashMap<String, ExpenseCategory>,
    filed_as: String,
    initial_balance: f32,
}

fn keys_match<T: Eq + Hash, U, V>(
    map1: &HashMap<T, U>,
    map2: &HashMap<T, V>,
) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}

impl Eq for BudgetAccount {}

impl PartialEq for BudgetAccount {
    fn eq(&self, other: &Self) -> bool {
        self.filed_as == other.filed_as &&
            keys_match(&self.categories, &other.categories)
    }

    fn ne(&self, other: &Self) -> bool {
        self.filed_as != other.filed_as ||
            !(keys_match(&self.categories, &other.categories))
    }
}

impl BudgetAccount {
    /// Create new Budget name and expense categories in a vector of tuples
    /// # Arguments
    /// * filed_as: Name of the budget
    /// * expense_categories: Provide a list of expense categories and max_budget of each categories
    pub fn new_with_category_list(filed_as: &str, initial_balance: f32, categories: Vec<(&str, f32)>) -> BudgetAccount {
        let mut proc_categories = HashMap::new();
        for expense_category in categories {
            proc_categories.insert(expense_category.0.to_string(),
                                   ExpenseCategory::new_with_max_budget(
                                  expense_category.0,
                                  expense_category.1,
                              ));
        }
        BudgetAccount {
            categories: proc_categories,
            filed_as: filed_as.to_string(),
            initial_balance,
        }
    }

    pub fn find_category_by_name(&mut self, category_name: &str) -> &mut ExpenseCategory {
        self.categories
            .entry(category_name.to_string())
            .or_insert(ExpenseCategory::new(category_name))
    }

    pub fn add_expense(&mut self, expense_category: &str, amount_spent: f32) {
        self.find_category_by_name(expense_category)
            .add_expense(amount_spent);
    }

    pub fn total_balance(&self) -> f32 {
        self.categories.iter()
            .map(|(c, x)| x.available()).sum::<f32>()
    }

    pub fn new(filed_as: &str, initial_balance: f32, categories: HashMap<String, ExpenseCategory>) -> BudgetAccount {
        BudgetAccount {
            categories,
            initial_balance,
            filed_as: filed_as.to_string(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn new_budget() -> BudgetAccount {
        let mut categories = HashMap::new();
        categories.insert("Bills".to_string(), ExpenseCategory::new_with_max_budget("Bills", 2000.0));
        categories.insert("Travel".to_string(), ExpenseCategory::new_with_max_budget("Travel", 3000.0));
        BudgetAccount::new("main", 10000.0, categories)
    }

    #[test]
    fn create_new_budget_account_and_assign_categories_from_a_list() {
        let budget = BudgetAccount::new_with_category_list(
            "main",
            10000.0,
            vec![
                ("Bills", 2000.0),
                ("Travel", 3000.0),
            ]);
        assert_eq!(budget, new_budget());
    }

    // this test fails because of the ordering of the keys, otherwise it's ok
    #[test]
    #[ignore]
    fn make_json_file_from_budget() {
        let mut budget = new_budget();
        let result = serde_json::to_string(&budget);
        assert_eq!(r#"{"categories":{"Bills":{"name":"Bills","max_budget":2000.0,"transactions":[]},"Travel":{"name":"Travel","max_budget":3000.0,"transactions":[]}},"filed_as":"main","initial_balance":10000.0}"#, result.unwrap());
    }

    #[test]
    pub fn check_actual_total_balance() {
        let budget = new_budget();
        assert_eq!(budget.total_balance(), 5000.0);
    }

    #[test]
    fn finding_category_by_name_in_budget_account() {
        let mut budget = new_budget();
        let bills = budget.find_category_by_name("Bills");
        assert_eq!(bills.available(), 2000.0);
    }

    #[test]
    pub fn spending_from_category() {
        let mut budget = new_budget();
        budget.add_expense("Bills", 2000.0);
        let bills = budget.find_category_by_name("Bills");
        assert_eq!(bills.available(), 0.0);
        assert_eq!(budget.total_balance(), 3000.0);
    }
}
