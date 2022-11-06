mod imp;

use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};
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

    pub fn bind(&self, transaction_object: &TransactionObject) {
    }

    pub fn unbind(&self) {
        let imp = imp::TransactionRow::from_instance(self);
        for binding in imp.bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}