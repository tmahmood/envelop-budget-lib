use std::collections::HashMap;

use crate::budgeting::expense_category::ExpenseCategory;

pub struct Budget {
    categories: HashMap<String, ExpenseCategory>,
    filed_as: String,
}

impl Budget {
    pub(crate) fn find_category_by_name(&mut self, category_name: &str) -> &mut ExpenseCategory {
        self.categories
            .entry(category_name.to_string())
            .or_insert(ExpenseCategory::new(category_name))
    }

    pub(crate) fn add_expense(&mut self, expense_category: &str, amount_spent: i32) {
        self.find_category_by_name(expense_category)
            .add_expense(amount_spent);
    }

    pub(crate) fn total_balance(&self) -> i32 {
        self.categories.iter().map(|(c, x)| x.available()).sum::<i32>()
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
        categories.insert("Bills".to_string(), ExpenseCategory::with_max_budget("Bills", 2000));
        categories.insert("Travel".to_string(), ExpenseCategory::with_max_budget("Travel", 3000));
        Budget::new("main", categories)
    }

    #[test]
    pub fn make_new_budget() {
        let budget = new_budget();
        assert_eq!(budget.total_balance(), 5000);
    }

    #[test]
    fn finding_budget_from_list() {
        let mut budget = new_budget();
        let bills = budget.find_category_by_name("Bills");
        assert_eq!(bills.available(), 2000);
    }

    #[test]
    pub fn spending_from_category() {
        let mut budget = new_budget();
        budget.add_expense("Bills", 2000);
        assert_eq!(budget.total_balance(), 3000);
    }
}
