


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
use budgeting::expense_category::ExpenseCategory;

#[cfg(test)]
pub mod test_dir;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_dir::test_dir::TestDir;
    use std::path::Path;

    #[test]
    fn test_temp_dir() {
        let mut full_path = format!("");
        {
            let test_dir = TestDir::new();
            full_path = format!("test_dirs/{}", test_dir.folder_name);
            let p = Path::new(&full_path);
            assert!(p.exists());
        };
        let p = Path::new(&full_path);
        assert!(!p.exists());
    }
}