use crate::budgeting::category::{Category, CategoryModel};
use crate::schema::transactions;
use crate::{current_date, establish_connection, parse_date};
use chrono::{Local, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone)]
pub enum TransactionType {
    Income,
    Expense,
    TransferIn,
    TransferOut,
}

impl From<i32> for TransactionType {
    fn from(t: i32) -> TransactionType {
        match t {
            1 => TransactionType::Expense,
            2 => TransactionType::Income,
            3 => TransactionType::TransferIn,
            4 => TransactionType::TransferOut,
            _ => panic!("Invalid transaction type")
        }
    }
}
impl From<TransactionType> for i32 {
    fn from(t: TransactionType) -> i32 {
        match t {
            TransactionType::Income => 2,
            TransactionType::Expense => 1,
            TransactionType::TransferIn => 3,
            TransactionType::TransferOut => 4,
        }
    }
}

impl From<TransactionType> for String {
    fn from(t: TransactionType) -> String {
        match t {
            TransactionType::Income => "Income".to_string(),
            TransactionType::Expense => "Expense".to_string(),
            TransactionType::TransferIn => "Transfer In".to_string(),
            TransactionType::TransferOut => "Transfer Out".to_string(),
        }
    }
}

#[derive(
    Debug,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Queryable,
    Associations,
    Identifiable,
)]
#[diesel(belongs_to(Category))]
pub struct Transaction {
    id: i32,
    note: String,
    payee: String,
    date_created: NaiveDateTime,
    #[diesel(sql_type = Double)]
    amount: f64,
    category_id: i32,
    #[deprecated]
    income: bool,
    transfer_type_id: i32,
}

impl Transaction {
    pub fn new(
        payee: &str,
        note: &str,
        amount: f64,
        category_id: i32,
        date_created: NaiveDateTime,
    ) -> Transaction {
        Transaction {
            id: 0,
            payee: payee.to_string(),
            note: note.to_string(),
            amount,
            income: amount > 0.,
            date_created,
            category_id,
            transfer_type_id: 1,
        }
    }

    pub fn category_id(&self) -> i32 {
        self.category_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn only_amount(&self) -> f64 {
        let a = self.amount;
        if a < 0. {
            -1. * a
        } else {
            a
        }
    }

    pub fn income(&self) -> bool {
        self.income
    }

    pub fn note(&self) -> String {
        self.note.clone()
    }

    pub fn payee(&self) -> String {
        self.payee.clone()
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.income = amount > 0.;
        self.amount = amount;
    }

    pub fn set_payee(&mut self, payee: String) {
        self.payee = payee;
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }

    pub fn set_category_id(&mut self, category_id: i32) {
        self.category_id = category_id;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn date_created(&self) -> NaiveDate {
        self.date_created.date()
    }

    pub fn date_created_str(&self) -> String {
        self.date_created.date().to_string()
    }

    pub fn set_date_created(&mut self, date_created: NaiveDateTime) {
        self.date_created = date_created;
    }

    pub fn set_date_created_from_str(&mut self, date_created: String) {
        self.date_created = parse_date(&date_created);
    }

    pub fn transfer_type_id(&self) -> i32 {
        self.transfer_type_id
    }

    pub fn set_transfer_type_id(&mut self, transfer_type_id: i32) {
        self.transfer_type_id = transfer_type_id;
    }
}

pub struct TransactionModel<'a> {
    transaction: Transaction,
    conn: &'a mut SqliteConnection,
}

impl<'a> TransactionModel<'a> {
    pub fn new(conn: &'a mut SqliteConnection, transaction: Transaction) -> Self {
        TransactionModel { transaction, conn }
    }
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    pub fn category_name(&mut self) -> String {
        match CategoryModel::load(self.conn, self.transaction.category_id) {
            Ok(c) => c.name(),
            Err(diesel::result::Error::NotFound) => "Not Found!".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub note: &'a str,
    pub payee: &'a str,
    pub date_created: NaiveDateTime,
    pub amount: f64,
    pub category_id: i32,
    pub income: bool,
    pub transaction_type_id: i32,
}


pub struct TransactionBuilder<'a> {
    amount: Option<f64>,
    payee: Option<&'a str>,
    note: Option<&'a str>,
    income: Option<bool>,
    date_created: Option<NaiveDateTime>,
    transaction_type: TransactionType,
    category_id: i32,
    conn: &'a mut SqliteConnection
}

impl<'a> TransactionBuilder<'a> {
    pub fn new(conn: &'a mut SqliteConnection, category_id: i32) -> Self {
        TransactionBuilder {
            amount: None,
            payee: None,
            note: None,
            income: None,
            date_created: None,
            transaction_type: TransactionType::Expense,
            category_id,
            conn
        }
    }

    fn reset(&mut self) {
        self.amount = None;
        self.payee = None;
        self.note = None;
        self.income = None;
        self.date_created = None;
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

    pub fn date_created(&mut self, date_created: NaiveDateTime) -> &mut Self {
        self.date_created = Some(date_created);
        self
    }

    pub fn done(&mut self) -> Transaction {
        if self.note.is_none() || self.payee.is_none() || self.amount.is_none() {
            panic!("Not all field set")
        }

        let signed_amount = match self.transaction_type {
            TransactionType::Income | TransactionType::TransferIn => self.amount.unwrap(),
            TransactionType::Expense | TransactionType::TransferOut => -1. * self.amount.unwrap(),
        };
        let new_transaction = NewTransaction {
            note: self.note.as_ref().unwrap(),
            payee: self.payee.as_ref().unwrap(),
            date_created: self.date_created.unwrap_or_else(current_date),
            income: self.income.unwrap(),
            amount: signed_amount,
            category_id: self.category_id,
            transaction_type_id: i32::from(self.transaction_type.clone()),
        };
        imp_db!(transactions);
        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(self.conn)
            .expect("Error saving new transaction");
        let results = transactions
            .order(id.desc())
            .limit(1)
            .load::<Transaction>(self.conn)
            .unwrap();
        self.reset();
        results.first().unwrap().clone()
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
