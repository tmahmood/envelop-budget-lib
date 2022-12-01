pub mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib};
use gtk::glib::Object;
use budget_manager::budgeting::Budgeting;
use crate::fix_float;

glib::wrapper! {
    pub struct SummaryObject(ObjectSubclass<imp::SummaryObject>);
}

impl SummaryObject {
    pub fn new(budget: &mut Budgeting) -> Self {
        let budget_details_available =  fix_float(budget.actual_total_balance());
        let budget_unallocated = fix_float(budget.uncategorized_balance());
        let budget_allocated = fix_float(budget.total_allocated());
        let budget_total_income = fix_float(budget.total_income());
        let budget_total_expense = fix_float(-1. * budget.total_expense());
        Object::builder()
            .property("budget-details-available", budget_details_available)
            .property("budget-unallocated", budget_unallocated)
            .property("budget-allocated", budget_allocated)
            .property("budget-total-income", budget_total_income)
            .property("budget-total-expense", budget_total_expense)
            .build()
    }

}
