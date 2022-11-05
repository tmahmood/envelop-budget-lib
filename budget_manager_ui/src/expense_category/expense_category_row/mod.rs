mod imp;

use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};
use crate::expense_category::expense_category_object::ExpenseCategoryObject;


glib::wrapper! {
    pub struct ExpenseCategoryRow(ObjectSubclass<imp::ExpenseCategoryRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for ExpenseCategoryRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpenseCategoryRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, expense_category_object: &ExpenseCategoryObject) {
        // Get state
        let imp = imp::ExpenseCategoryRow::from_instance(self);
        let expense_category_row = imp.data_row.get();
        let name_label = imp.name_label.get();
        let max_budget = imp.max_budget_label.get();
        let mut bindings = imp.bindings.borrow_mut();

        let data_row_binding = expense_category_object
            .bind_property("name", &expense_category_row, "title")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        // Save binding
        bindings.push(data_row_binding);

        let data_row_binding = expense_category_object
            .bind_property("maxbudget", &name_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        // Save binding
        bindings.push(data_row_binding);
    }

    pub fn unbind(&self) {
        // Get state
        let imp = imp::ExpenseCategoryRow::from_instance(self);

        // Unbind all stored bindings
        for binding in imp.bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}