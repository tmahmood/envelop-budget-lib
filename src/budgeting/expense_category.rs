pub struct ExpenseCategory {
    name: String,
    max_budget: i32,
    transactions: Vec<i32>,
}

impl ExpenseCategory {
    pub fn take_from(&mut self, src: &mut ExpenseCategory, amount: i32) -> &mut Self {
        src.add_expense(amount.into());
        self.add_fund(amount.into())
    }

    pub fn add_fund(&mut self, amount: i32) -> &mut Self {
        self.transactions.push(amount);
        self
    }

    pub fn with_max_budget(name: &str, max_budget: i32) -> Self {
        ExpenseCategory {
            name: name.to_string(), max_budget,
            transactions: Vec::new(),
        }
    }

    pub fn new(name: &str) -> Self {
        ExpenseCategory {
            name: name.to_string(),
            max_budget: 0,
            transactions: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_max_budget(&mut self, max_budget: i32) -> &mut Self {
        self.max_budget = max_budget;
        self
    }

    pub fn get_max_budget(&self) -> i32 {
        self.max_budget.into()
    }

    pub fn add_expense(&mut self, amount: i32) -> &mut Self {
        self.transactions.push(-1i32 * amount);
        self
    }

    pub fn available(&self) -> i32 {
        self.get_max_budget() + self.transactions.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::budgeting::expense_category::ExpenseCategory;

    #[test]
    fn get_name() {
        let e = ExpenseCategory::new("Some name");
        assert_eq!(
            e.get_name(), "Some name"
        )
    }

    #[test]
    fn create_expense_category_with_max_budget() {
        let mut a = ExpenseCategory::with_max_budget("Others", 1000);
        assert_eq!(1000, a.available());
    }

    #[test]
    fn transferring_fund_from_other_category() {
        let mut a2 = ExpenseCategory::new("Others");
        a2.set_max_budget(5000);
        let mut a1 = ExpenseCategory::new("Bills");
        a1.set_max_budget(4000).take_from(&mut a2, 2000);
        assert_eq!(a2.available(), 3000);
        assert_eq!(a1.available(), 6000);
    }

    #[test]
    fn adding_more_fund_to_category() {
        assert_eq!(ExpenseCategory::new("Bills")
                       .set_max_budget(5000)
                       .add_fund(3000)
                       .available(),
                   8000);
    }

    #[test]
    fn spending_from_category() {
        assert_eq!(ExpenseCategory::new("Bills")
                       .set_max_budget(5000)
                       .add_expense(3000)
                       .available(),
                   2000);
    }

    #[test]
    fn adding_fund_to_category() {
        // creating new category and set maximum budget
        assert_eq!(ExpenseCategory::new("Bills")
                       .set_max_budget(3000)
                       .get_max_budget(),
                   3000
        );
    }
}

