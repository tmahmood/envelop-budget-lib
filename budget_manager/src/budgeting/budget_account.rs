use std::collections::{BTreeMap};
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Error transferring fund from one category to other")]
    FundTransferError
}

use crate::budgeting::transaction_category::TransactionCategory;
use crate::budgeting::transaction::Transaction;
use crate::DEFAULT_CATEGORY;


/// Budget is used to store all the transaction categories and store their details in a file
#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetAccount {
    categories: BTreeMap<String, TransactionCategory>,
    filed_as: String,
    initial_balance: f32,
    transactions: Vec<Transaction>,
}


fn keys_match<T: Eq + Hash + Ord, U, V>(
    map1: &BTreeMap<T, U>,
    map2: &BTreeMap<T, V>,
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
    /// Create new Budget name and transaction categories in a vector of tuples
    /// # Arguments
    /// * filed_as: Name of the budget
    /// * transaction_categories: Provide a list of transaction categories and max_budget of each categories
    pub fn new(filed_as: &str, initial_balance: f32, categories: Vec<(&str, f32)>) -> BudgetAccount {
        let mut proc_categories = BTreeMap::new();
        let mut total_allocated = 0.;
        for category in categories {
            proc_categories.insert(category.0.to_string(),
                                   TransactionCategory::new_with_allocated(
                                       category.0,
                                       category.1,
                                   ));
            total_allocated += category.1;
        }
        proc_categories.insert(
            DEFAULT_CATEGORY.to_string(),
            TransactionCategory::new_with_allocated("Unused", initial_balance - total_allocated)
        );
        BudgetAccount {
            categories: proc_categories,
            filed_as: filed_as.to_string(),
            initial_balance,
            transactions: vec![],
        }
    }

    pub(crate) fn unallocated(&self) -> f32 {
        self.categories.get(DEFAULT_CATEGORY).unwrap().allocated()
    }

    // pub(crate) fn transfer_fund(&mut self, src: Option<&str>, dest: &str, amount: f32) -> Result<(), Error> {
    //     let d = self.find_or_create_category_by_name(dest);
    //     let d = dest.allocated();
    //     let (c, s) = if src.is_some() {
    //         (Some(src.unwrap().get_name().as_str()), src.unwrap().allocated())
    //     } else {
    //         (None, self.unallocated())
    //     };
    //     self.add_transaction(c, -1. * amount);
    //     self.add_transaction(Some(&dest.get_name()),
    //         amount,
    //     );
    //     Ok(())
    // }


    pub(crate) fn category_balance(&self, category_name: &str) -> f32 {
        let sum = self.transactions.iter()
            .filter(|v| v.category_name() == category_name)
            .map(|v| v.amount())
            .sum::<f32>();
        self.categories.get(category_name)
            .and_then(|v| Some(v.allocated() + sum))
            .unwrap()
    }

    pub fn find_or_create_category_by_name(&mut self, category_name: &str) -> &mut TransactionCategory {
        self.categories
            .entry(category_name.to_string())
            .or_insert(TransactionCategory::new(category_name))
    }

    pub fn new_expense(&mut self, transaction_category: Option<&str>, spent: f32) {
        self.add_transaction(
            transaction_category, -1. * spent
        )
    }

    pub fn new_income(&mut self, transaction_category: Option<&str>, spent: f32) {
        self.add_transaction(
            transaction_category, spent
        )
    }
    fn add_transaction(&mut self, transaction_category: Option<&str>, amount: f32) {
        let category = transaction_category.unwrap_or(DEFAULT_CATEGORY);
        self.transactions.push(
            Transaction::new(
                "",
                "undefined",
                amount,
                category,
            )
        );
    }

    pub fn total_balance(&self) -> f32 {
        self.initial_balance + self.transactions.iter().map(|v| v.amount()).sum::<f32>()
    }


    pub fn all_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::budgeting::transaction::Transaction;
    use super::*;
    use crate::tests::{BILLS, TRAVEL, UNUSED, INITIAL, new_budget};

    #[test]
    fn make_json_file_from_budget() {
        let budget = new_budget();
        let result = serde_json::to_string(&budget);
        assert_eq!(r#"{"categories":{"Bills":{"name":"Bills","allocated":2000.0},"Travel":{"name":"Travel","allocated":3000.0},"Unused":{"name":"Unused","allocated":5000.0}},"filed_as":"main","initial_balance":10000.0,"transactions":[]}"#, result.unwrap());
    }

    #[test]
    fn get_all_transactions() {
        let mut budget = new_budget();
        let transactions: Vec<Transaction> = budget.all_transactions();
        assert_eq!(transactions.len(), 0);
        budget.add_transaction(Some("Bills"), -2000.0);
        let transactions: Vec<Transaction> = budget.all_transactions();
        assert_eq!(transactions.len(), 1);
    }

    #[test]
    pub fn check_actual_total_balance() {
        let budget = new_budget();
        assert_eq!(budget.total_balance(), TRAVEL + BILLS + UNUSED);
    }

    #[test]
    fn finding_category_by_name_in_budget_account() {
        let mut budget = new_budget();
        let bills = budget.find_or_create_category_by_name("Bills");
        assert_eq!(bills.allocated(), BILLS);
    }

    #[test]
    pub fn spending_from_category() {
        let mut budget = new_budget();
        budget.add_transaction(Some("Bills"), -1. * BILLS);
        let bills_available = budget.category_balance("Bills");
        assert_eq!(bills_available, 0.0);
        assert_eq!(budget.total_balance(), TRAVEL + UNUSED);
    }

    // #[test]
    // fn transfer_fund() {
    //     let mut budget = new_budget();
    //     let bills = budget.find_or_create_category_by_name("Bills");
    //     let travel = budget.find_or_create_category_by_name("Travel");
    //     assert!(budget.transfer_fund(bills, travel, BILLS).is_ok());
    //     assert_eq!(bills.allocated(), 0.);
    //     assert_eq!(travel.allocated(), BILLS + TRAVEL);
    // }

    // #[test]
    // fn transfer_fund_from_balance() {
    //     let mut budget = new_budget();
    //     assert!(budget.transfer_fund(Some("Bills"), "Travel", BILLS).is_ok());
    //     assert_eq!(bills.allocated(), 0.);
    //     assert_eq!(travel.allocated(), BILLS + TRAVEL);
    // }
}
