pub mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib};
use gtk::glib::Object;
use budget_manager::budgeting::Budgeting;
use crate::fix_float;
use crate::summary::summary_object::imp::SummaryData;

glib::wrapper! {
    pub struct SummaryObject(ObjectSubclass<imp::SummaryObject>);
}

impl SummaryObject {
    pub fn new(summary_data: SummaryData) -> Self {
        Object::builder()
            .property("balance", summary_data.balance)
            .property("transfer-in", summary_data.transfer_in)
            .property("transfer-out", summary_data.transfer_out)
            .property("total-income", summary_data.total_income)
            .property("total-expense", summary_data.total_expense)
            .build()
    }

}
