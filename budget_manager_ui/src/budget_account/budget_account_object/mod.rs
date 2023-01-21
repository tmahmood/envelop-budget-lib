pub mod imp;

use std::borrow::BorrowMut;
use gtk::glib::Object;
use gtk::glib;
use budget_manager::budgeting::budget_account::BudgetAccountModel;
use crate::date_time_to_string;

glib::wrapper! {
    pub struct BudgetAccountObject(ObjectSubclass<imp::BudgetAccountObject>);
}

impl BudgetAccountObject {
    pub fn new(bm: &mut BudgetAccountModel) -> Self {
        let b = bm.budget_account();
        Object::builder()
            .property("id", &b.id())
            .property("filed-as", &b.filed_as())
            .property("date-created", date_time_to_string(b.date_created()))
            .build()
    }
}
