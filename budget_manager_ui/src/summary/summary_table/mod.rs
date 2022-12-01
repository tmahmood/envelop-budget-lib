mod imp;

use glib::{Object};
use gtk::{glib};


glib::wrapper! {
    pub struct SummaryTable(ObjectSubclass<imp::SummaryTable>)
    @extends gtk::ListBox, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for SummaryTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SummaryTable {
    pub fn new() -> Self {
        Object::builder().build()
    }
}