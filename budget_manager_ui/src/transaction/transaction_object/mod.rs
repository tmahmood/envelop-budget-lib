pub mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use clap::builder::Str;
use budget_manager::budgeting::transaction::{Transaction, TransactionModel};
use gtk::glib;
use gtk::glib::Object;
use crate::transaction::transaction_object::imp::TransactionInner;

glib::wrapper! {
    pub struct TransactionObject(ObjectSubclass<imp::TransactionObject>);
}

impl TransactionObject {
    pub fn new(transaction_inner: TransactionInner) -> Self {
        Object::builder()
            .property("id", &transaction_inner.id)
            .property("payee", &transaction_inner.payee)
            .property("note", &transaction_inner.note)
            .property("amount", &transaction_inner.amount)
            .property("only-amount", &transaction_inner.only_amount)
            .property("category-name", &transaction_inner.category_name)
            .property("date-created", &transaction_inner.date_created)
            .property("transaction-type", &transaction_inner.transfer_type)
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
        self.imp().data.borrow().transfer_type.clone()
    }

    pub fn date_created(&self) -> String {
        self.imp().data.borrow().date_created.clone()
    }
}
