use crate::budgeting::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct ExpenseCategory {
    name: String,
    max_budget: f32,
    transactions: Vec<Transaction>,
}

impl ExpenseCategory {

    pub fn take_from(&mut self, src: &mut ExpenseCategory, amount: f32) -> &mut Self {
        src.add_expense(amount.into());
        self.add_fund(amount.into())
    }

    pub fn add_fund(&mut self, amount: f32) -> &mut Self {
        self.transactions.push(Transaction::new("not defined", amount));
        self
    }

    pub fn new_with_max_budget(name: &str, max_budget: f32) -> Self {
        ExpenseCategory {
            name: name.to_string(),
            max_budget,
            transactions: Vec::new(),
        }
    }

    pub fn new(name: &str) -> Self {
        ExpenseCategory {
            name: name.to_string(),
            max_budget: 0.0,
            transactions: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_max_budget(&mut self, max_budget: f32) -> &mut Self {
        self.max_budget = max_budget;
        self
    }

    pub fn get_max_budget(&self) -> f32 {
        self.max_budget.into()
    }

    pub fn add_expense(&mut self, amount: f32) -> &mut Self {
        self.transactions.push(Transaction::new(
            "undefined",
            -1.0 * amount
            )
        );
        self
    }

    pub fn available(&self) -> f32 {
        self.get_max_budget() + self.transactions.iter()
            .map(|v| v.get_amount())
            .sum::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::budgeting::expense_category::ExpenseCategory;

    #[test]
    fn get_expense_category_name() {
        let e = ExpenseCategory::new("Some name");
        assert_eq!(
            e.get_name(), "Some name"
        )
    }

    #[test]
    fn create_expense_category_with_max_budget() {
        let mut a = ExpenseCategory::new_with_max_budget("Others", 1000.0);
        assert_eq!(1000.0, a.available());
    }

    #[test]
    fn transferring_fund_from_other_category() {
        let mut a2 = ExpenseCategory::new("Others");
        a2.set_max_budget(5000.0);
        let mut a1 = ExpenseCategory::new("Bills");
        a1.set_max_budget(4000.0).take_from(&mut a2, 2000.0);
        assert_eq!(a2.available(), 3000.0);
        assert_eq!(a1.available(), 6000.0);
    }

    #[test]
    fn adding_more_fund_to_category() {
        assert_eq!(ExpenseCategory::new("Bills")
                       .set_max_budget(5000.0)
                       .add_fund(3000.0)
                       .available(),
                   8000.0);
    }

    #[test]
    fn spending_from_category() {
        assert_eq!(ExpenseCategory::new("Bills")
                       .set_max_budget(5000.0)
                       .add_expense(3000.0)
                       .available(),
                   2000.0);
    }

    #[test]
    fn adding_fund_to_category() {
        // creating new category and set maximum budget
        assert_eq!(ExpenseCategory::new("Bills")
                       .set_max_budget(3000.0)
                       .get_max_budget(),
                   3000.0
        );
    }
}
