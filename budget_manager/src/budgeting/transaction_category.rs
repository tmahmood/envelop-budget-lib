use crate::budgeting::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionCategory {
    name: String,
    allocated: f32,
}

impl TransactionCategory {

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

    pub(crate) fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn allocated(&self) -> f32 {
        self.allocated.into()
    }

    pub(crate) fn set_allocated(&mut self, amount: f32) {
        self.allocated = amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
