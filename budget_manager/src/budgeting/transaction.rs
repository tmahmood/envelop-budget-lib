use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Default)]
pub struct Transaction {
    payee: String,
    note: String,
    amount: f32,
    income: bool,
}

impl Transaction {
    pub fn new(payee: &str, note: &str, amount: f32) -> Transaction {
        Transaction {
            payee: payee.to_string(),
            note: note.to_string(),
            amount, income: amount > 0.
        }
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }

    pub fn get_only_amount(&self) -> f32 {
        let a = self.amount;
        if a < 0. { -1. * a} else { a }
    }

    pub fn is_income(&self) -> bool {
        self.amount > 0.
    }

    pub fn get_note(&self) -> String {
        self.note.clone()
    }

    pub fn get_payee(&self) -> String {
        self.payee.clone()
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.income = amount > 0.;
        self.amount = amount;
    }

    pub fn set_payee(&mut self, payee: String) {
        self.payee = payee;
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }
}

