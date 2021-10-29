mod imp;

use crate::transaction_object::TransactionObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};


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
        Object::new(&[]).expect("Failed to create `TransactionRow`.")
    }

    pub fn bind(&self, transaction_object: &TransactionObject) {
        // Get state
        let imp = imp::TransactionRow::from_instance(self);
        let note_label = imp.note_label.get();
        let amount_label = imp.amount_label.get();
        let mut bindings = imp.bindings.borrow_mut();

        let note_label_binding = transaction_object
            .bind_property("note", &note_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .expect("Could not bind properties");
        // Save binding
        bindings.push(note_label_binding);

        // Bind `todo_object.content` to `todo_row.content_label.label`
        let amount_label_binding = transaction_object
            .bind_property("amount", &amount_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build()
            .expect("Could not bind properties");
        // Save binding
        bindings.push(amount_label_binding);
    }

    pub fn unbind(&self) {
        // Get state
        let imp = imp::TransactionRow::from_instance(self);

        // Unbind all stored bindings
        for binding in imp.bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}