mod imp;

use glib::{Object};
use gtk::{glib};



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
}