mod imp;

use crate::summary::summary_object::SummaryObject;
use adw::glib::{BindingFlags, ObjectExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::glib;

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

    pub fn bind_summary(self, summary_object: &SummaryObject) -> Self {
        let transfer_in = self.imp().transfer_in.get();
        let transfer_out = self.imp().transfer_out.get();
        let total_expense = self.imp().total_expense.get();
        let total_income = self.imp().total_income.get();

        summary_object
            .bind_property("transfer-in", &transfer_in, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("transfer-out", &transfer_out, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("total-income", &total_income, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("total-expense", &total_expense, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        self
    }
}
