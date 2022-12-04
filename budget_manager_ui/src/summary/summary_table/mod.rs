mod imp;

use adw::glib::{BindingFlags, ObjectExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::{Object};
use gtk::{glib};
use crate::summary::summary_object::SummaryObject;


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
        let budget_details_available = self.imp().budget_details_available.get();
        let budget_unallocated = self.imp().budget_unallocated.get();
        let budget_allocated = self.imp().budget_allocated.get();
        let budget_total_expense = self.imp().budget_total_expense.get();
        let budget_total_income = self.imp().budget_total_income.get();
        summary_object
            .bind_property(
                "budget-details-available",
                &budget_details_available,
                "label",
            )
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("budget-unallocated", &budget_unallocated, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("budget-allocated", &budget_allocated, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("budget-total-income", &budget_total_income, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        summary_object
            .bind_property("budget-total-expense", &budget_total_expense, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        self
    }
}