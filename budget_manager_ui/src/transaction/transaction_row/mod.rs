mod imp;

use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};
use crate::transaction::transaction_object::TransactionObject;


glib::wrapper! {
    pub struct TransactionRow(ObjectSubclass<imp::TransactionRow>)
    @extends gtk::Box, gtk::Widget,
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

    pub fn bind(&self, transaction_object: &TransactionObject) {
        // Get state
        let imp = imp::TransactionRow::from_instance(self);
        let transaction_row = imp.data_row.get();
        let note_label = imp.note_label.get();
        let payee_label = imp.payee_label.get();
        let mut bindings = imp.bindings.borrow_mut();

        let data_row_binding = transaction_object
            .bind_property("payee", &payee_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        bindings.push(data_row_binding);

        let data_row_binding = transaction_object
            .bind_property("note", &transaction_row, "title")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        bindings.push(data_row_binding);

        let data_row_binding = transaction_object
            .bind_property("amount", &note_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        bindings.push(data_row_binding);
    }

    pub fn unbind(&self) {
        let imp = imp::TransactionRow::from_instance(self);
        for binding in imp.bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}