use crate::budgeting::budget_account::BudgetAccount;
use crate::budgeting::category::{Category, CategoryModel};
use crate::budgeting::transaction::{Transaction, TransactionType};
use diesel::SqliteConnection;
use std::rc::Rc;

pub struct TransactionAddToCategoryOps<'a> {
    budget: BudgetAccount,
    amount: Option<f64>,
    payee: Option<&'a str>,
    note: Option<&'a str>,
    income: Option<bool>,
    transaction_type: TransactionType,
    category_model: CategoryModel<'a>,
}

impl<'a> TransactionAddToCategoryOps<'a> {
    pub fn new(budget: BudgetAccount, category_model: CategoryModel<'a>) -> Self {
        TransactionAddToCategoryOps {
            budget,
            amount: None,
            payee: None,
            note: None,
            income: None,
            transaction_type: TransactionType::Expense,
            category_model,
        }
    }

    fn reset(&mut self) {
        self.amount = None;
        self.payee = None;
        self.note = None;
        self.income = None;
    }

    pub fn transfer_from(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self.income = Some(false);
        self.transaction_type = TransactionType::TransferOut;
        self
    }

    pub fn transfer_to(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self.income = Some(true);
        self.transaction_type = TransactionType::TransferIn;
        self
    }
    pub fn expense(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self.income = Some(false);
        self.transaction_type = TransactionType::Expense;
        self
    }

    pub fn income(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self.income = Some(true);
        self.transaction_type = TransactionType::Income;
        self
    }

    pub fn payee(&mut self, payee: &'a str) -> &mut Self {
        self.payee = Some(payee);
        self
    }

    pub fn note(&mut self, note: &'a str) -> &mut Self {
        self.note = Some(note);
        self
    }

    pub fn done(&mut self) -> Transaction {
        if self.amount.is_none() {
            panic!("Amount needs to be set");
        }
        if self.income.is_none() {
            panic!("transaction type not set");
        }
        let t = self.category_model
            .new_transaction(self.amount.unwrap(), &self.transaction_type)
            .payee(self.payee.unwrap())
            .note(self.note.unwrap())
            .done();
        self.reset();
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budgeting::Budgeting;
    use crate::test_helpers::new_budget_using_budgeting;
    use crate::test_helpers::DbDropper;

    #[test]
    fn transaction_op_struct_handles_full_transaction_details() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let mut d = blib.new_transaction_to_category("Travel");
        d.income(1000.).payee("Some").note("Other").done();
        d.expense(2000.).payee("Some").note("Other").done();
        assert_eq!(blib.category_balance("Travel"), 2000.);
    }
}
