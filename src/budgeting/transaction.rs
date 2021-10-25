use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    note: String,
    amount: f32,
}

impl Transaction {
    pub fn new(note: &str, amount: f32) -> Transaction {
        Transaction {
            note: note.to_string(),
            amount
        }
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }

}

