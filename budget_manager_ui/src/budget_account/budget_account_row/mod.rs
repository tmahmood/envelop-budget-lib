mod imp;

use adw::glib;
use adw::prelude::ActionRowExt;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Label};
use crate::budget_account::budget_account_object::BudgetAccountObject;

glib::wrapper! {
    pub struct BudgetAccountRow(ObjectSubclass<imp::BudgetAccountRow>)
    @extends adw::ActionRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::Actionable;
}

impl Default for BudgetAccountRow {
    fn default() -> Self {
        Self::new()
    }
}

impl BudgetAccountRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind_objects(self, budget_account_object: &BudgetAccountObject) -> Self {
        self.imp().budget_account_id.replace(budget_account_object.property("id"));
        let id_label = self.imp().budget_account_id_label.get();
        budget_account_object
            .bind_property("id", &id_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        budget_account_object
            .bind_property("filed_as", &self, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        budget_account_object
            .bind_property("date_created", &self, "subtitle")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        self
    }
}
