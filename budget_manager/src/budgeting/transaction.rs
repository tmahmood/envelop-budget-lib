use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Default)]
pub struct Transaction {
    payee: String,
    note: String,
    amount: f32,
}

impl Transaction {
    pub fn new(payee: &str, note: &str, amount: f32) -> Transaction {
        Transaction {
            payee: payee.to_string(),
            note: note.to_string(),
            amount,
        }
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }

    pub fn get_note(&self) -> String {
        self.note.clone()
    }

    pub fn get_payee(&self) -> String {
        self.payee.clone()
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount;
    }

    pub fn set_payee(&mut self, payee: String) {
        self.payee = payee;
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }
}

