use crate::budgeting::transaction_category::Category;
use crate::{current_date, establish_connection, parse_date};
use chrono::{Local, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    income: bool,
    #[diesel(sql_type = Double)]
    amount: f64,
    category_id: i32,
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
            category_id: category_id,
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

    pub fn get_new_transaction(&self) -> NewTransaction {
        NewTransaction {
            payee: self.payee.as_str(),
            note: self.note.as_str(),
            amount: self.amount,
            date_created: self.date_created,
            income: self.income,
            category_id: self.category_id,
        }
    }

    pub fn save(&mut self, conn: &mut SqliteConnection) {
        let t1 = self.get_new_transaction();
        use crate::schema::transactions;
        use crate::schema::transactions::dsl::*;
        diesel::insert_into(transactions::table)
            .values(&self.get_new_transaction())
            .execute(conn)
            .expect("Error saving new transaction");
        let results = transactions.limit(1).load::<Transaction>(conn).unwrap();
    }
}

use crate::schema::transactions;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub note: &'a str,
    pub payee: &'a str,
    pub date_created: NaiveDateTime,
    pub income: bool,
    pub amount: f64,
    pub category_id: i32,
}

pub struct TransactionBuilder {
    income: bool,
    category_id: Option<i32>,
    note: Option<String>,
    payee: Option<String>,
    amount: f64,
    date_created: Option<NaiveDateTime>,
}

impl TransactionBuilder {
    pub(crate) fn new_income(amount: f64) -> Self {
        Self {
            income: true,
            category_id: None,
            note: None,
            payee: None,
            amount,
            date_created: None,
        }
    }

    pub(crate) fn new_expense(amount: f64) -> Self {
        Self {
            income: false,
            category_id: None,
            note: None,
            payee: None,
            amount,
            date_created: None,
        }
    }

    pub(crate) fn category(&mut self, category_id: i32) -> &mut Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn note(&mut self, note: &str) -> &mut Self {
        self.note = Some(note.to_string());
        self
    }

    pub fn payee(&mut self, payee: &str) -> &mut Self {
        self.payee = Some(payee.to_string());
        self
    }

    pub fn date_created(&mut self, date_created: NaiveDateTime) -> &mut Self {
        self.date_created = Some(date_created);
        self
    }

    pub fn done(&self, conn: &mut SqliteConnection) -> Transaction {
        if self.category_id.is_none() || self.note.is_none() || self.payee.is_none() {
            panic!("Not all field set")
        }
        let signed_amount = if self.income {
            self.amount
        } else {
            -1. * self.amount
        };
        let new_transaction = NewTransaction {
            note: self.note.as_ref().unwrap(),
            payee: self.payee.as_ref().unwrap(),
            date_created: self.date_created.unwrap_or(current_date()),
            income: self.income,
            amount: signed_amount,
            category_id: self.category_id.unwrap(),
        };
        imp_db!(transactions);
        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)
            .expect("Error saving new transaction");
        let results = transactions
            .order(id.desc())
            .limit(1)
            .load::<Transaction>(conn)
            .unwrap();
        results.first().unwrap().clone()
    }
}
