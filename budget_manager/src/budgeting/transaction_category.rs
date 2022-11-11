use crate::budgeting::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionCategory {
    name: String,
    allocated: f32,
}

impl TransactionCategory {

    pub(crate) fn update_allocation(&mut self, amount: f32) -> &mut Self {
        self.allocated += amount;
        self
    }

    pub(crate) fn new_with_allocated(name: &str, allocated: f32) -> Self {
        TransactionCategory {
            name: name.to_string(),
            allocated,
        }
    }

    pub(crate) fn new(name: &str) -> Self {
        TransactionCategory {
            name: name.to_string(),
            allocated: 0.0,
        }
    }

    pub(crate) fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub(crate) fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub(crate) fn allocated(&self) -> f32 {
        self.allocated.into()
    }

    pub(crate) fn available(&self, transactions: &Vec<Transaction>) -> f32 {
        let sum = transactions.iter()
            .map(|v| v.amount())
            .sum::<f32>();
        self.allocated() +  sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
