mod imp;

use crate::category::category_object::CategoryObject;
use adw::prelude::ActionRowExt;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango, Label};
use pango::{AttrList, Attribute};

glib::wrapper! {
    pub struct CategoryRow(ObjectSubclass<imp::CategoryRow>)
    @extends adw::ActionRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::Actionable;
}

impl Default for CategoryRow {
    fn default() -> Self {
        Self::new()
    }
}

impl CategoryRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind_objects(self, category_object: &CategoryObject) -> Self {
        self.imp().category_id.replace(category_object.property("id"));
        let id_label = self.imp().category_id_label.get();
        category_object
            .bind_property("id", &id_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        category_object
            .bind_property("name", &self, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        category_object
            .bind_property("allocated", &self, "subtitle")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        self
    }
}
