mod imp;

use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};
use budget_manager::budgeting::transaction::Transaction;
use crate::transaction::transaction_object::TransactionObject;


glib::wrapper! {
    pub struct TransactionRow(ObjectSubclass<imp::TransactionRow>)
    @extends gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TransactionRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_transaction_row(&self, transaction: &TransactionObject) {
        self.imp().note_label.get().set_text(&transaction.note());
        self.imp().category_name_label.get().set_text(&transaction.category_name());
        self.imp().amount_label.get().set_text(&transaction.only_amount().to_string());
        self.imp().payee_label.get().set_text(&transaction.payee());

    }
}