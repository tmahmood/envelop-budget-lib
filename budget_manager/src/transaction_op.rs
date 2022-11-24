use std::rc::Rc;
use crate::budgeting::budget_account::BudgetAccount;
use crate::budgeting::transaction::Transaction;
use crate::budgeting::transaction_category::Category;
use diesel::SqliteConnection;

pub struct TransactionOp<'a> {
    budget: &'a BudgetAccount,
    category: Category,
    conn: &'a mut SqliteConnection,
    amount: Option<f64>,
    payee: Option<&'a str>,
    note: Option<&'a str>,
    income: Option<bool>,
}

impl<'a> TransactionOp<'a> {
    pub fn new(
        conn: &'a mut SqliteConnection,
        budget: &'a BudgetAccount,
        category: &'a str,
    ) -> Self {
        TransactionOp {
            budget,
            category: budget.find_category(conn, category).unwrap(),
            conn,
            amount: None,
            payee: None,
            note: None,
            income: None,
        }
    }

    fn reset(&mut self) {
        self.amount = None;
        self.payee = None;
        self.note = None;
        self.income = None;
    }
    pub fn expense(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self.income = Some(false);
        self
    }

    pub fn income(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self.income = Some(true);
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
        let mut n = if let (Some(true)) = self.income {
            self.category.new_income(self.amount.unwrap())
        } else {
            self.category.new_expense(self.amount.unwrap())
        };
        let t = n.payee("Some").note("Other").done(self.conn);
        self.reset();
        t
    }
}
