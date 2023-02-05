pub mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use clap::builder::Str;
use budget_manager::budgeting::transaction::{Transaction, TransactionModel};
use gtk::glib;
use gtk::glib::Object;
use crate::transaction::transaction_object::imp::{from_transaction_to_transfer_inner, TransactionInner};

glib::wrapper! {
    pub struct TransactionObject(ObjectSubclass<imp::TransactionObject>);
}

impl TransactionObject {
    pub fn new(tm: &mut TransactionModel, category_name: &str) -> Self {
        let transaction_inner = from_transaction_to_transfer_inner(tm, category_name.to_string());
        Object::builder()
            .property("id", &transaction_inner.id)
            .property("payee", &transaction_inner.payee)
            .property("note", &transaction_inner.note)
            .property("amount", &transaction_inner.amount)
            .property("only-amount", &transaction_inner.only_amount)
            .property("category-name", &transaction_inner.category_name)
            .property("date-created", &transaction_inner.date_created)
            .property("transaction-type", &transaction_inner.transaction_type)
            .build()
    }

    pub fn id(&self) -> i32 {
        self.imp().data.borrow().id
    }

    pub fn category_name(&self) -> String {
        self.imp().data.borrow().category_name.clone()
    }

    pub fn payee(&self) -> String {
        self.imp().data.borrow().payee.clone()
    }

    pub fn note(&self) -> String {
        self.imp().data.borrow().note.clone()
    }

    pub fn amount(&self) -> String {
        self.imp().data.borrow().amount.clone()
    }

    pub fn only_amount(&self) -> String {
        self.imp().data.borrow().only_amount.clone()
    }

    pub fn transaction_type(&self) -> String {
        self.imp().data.borrow().transaction_type.clone()
    }

    pub fn date_created(&self) -> String {
        self.imp().data.borrow().date_created.clone()
    }
}
