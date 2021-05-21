use std::collections::HashMap;
use std::hash::Hash;

use crate::budgeting::expense_category::ExpenseCategory;

/// Budget is used to store all the expense categories and store their details in a file
#[derive(Debug)]
pub struct Budget {
    categories: HashMap<String, ExpenseCategory>,
    filed_as: String,
}

fn keys_match<T: Eq + Hash, U, V>(
    map1: &HashMap<T, U>,
    map2: &HashMap<T, V>,
) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}

impl Eq for Budget {}

impl PartialEq for Budget {
    fn eq(&self, other: &Self) -> bool {
        self.filed_as == other.filed_as &&
            keys_match(&self.categories, &other.categories)
    }

    fn ne(&self, other: &Self) -> bool {
        self.filed_as != other.filed_as ||
            !(keys_match(&self.categories, &other.categories))
    }
}

impl Budget {
    /// Create new Budget name and expense categories in a vector of tuples
    /// # Arguments
    /// * filed_as: Name of the budget
    /// * expense_categories: Provide a list of expense categories and max_budget of each categories
    pub fn new_from_list(filed_as: &str, expense_categories: Vec<(&str, f32)>) -> Budget {
        let mut categories = HashMap::new();
        for expense_category in expense_categories {
            categories.insert(expense_category.0.to_string(),
                              ExpenseCategory::with_max_budget(
                                  expense_category.0,
                                  expense_category.1,
                              ),
            );
        }
        Budget {
            categories,
            filed_as: filed_as.to_string(),
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
        self.categories.iter().map(|(c, x)| x.available()).sum::<f32>()
    }

    pub fn new(filed_as: &str, categories: HashMap<String, ExpenseCategory>) -> Budget {
        Budget {
            categories,
            filed_as: filed_as.to_string(),
        }
    }

}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn new_budget() -> Budget {
        let mut categories = HashMap::new();
        categories.insert("Bills".to_string(), ExpenseCategory::with_max_budget("Bills", 2000.0));
        categories.insert("Travel".to_string(), ExpenseCategory::with_max_budget("Travel", 3000.0));
        Budget::new("main", categories)
    }

    #[test]
    fn make_new_budget_constructor() {
        let budget = Budget::new_from_list(
            "main",
            vec![
                ("Bills", 2000.0),
                ("Travel", 3000.0)
            ]);
        assert_eq!(budget, new_budget());
    }

    #[test]
    pub fn make_new_budget() {
        let budget = new_budget();
        assert_eq!(budget.total_balance(), 5000.0);
    }

    #[test]
    fn finding_category_in_budget() {
        let mut budget = new_budget();
        let bills = budget.find_category_by_name("Bills");
        assert_eq!(bills.available(), 2000.0);
    }

    #[test]
    pub fn spending_from_category() {
        let mut budget = new_budget();
        budget.add_expense("Bills", 2000.0);
        assert_eq!(budget.total_balance(), 3000.0);
    }
}
