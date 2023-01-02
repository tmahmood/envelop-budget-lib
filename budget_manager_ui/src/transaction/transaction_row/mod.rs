mod imp;

use adw::glib::BindingFlags;
use glib::{Object};
use gtk::{glib, IconSize};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::transaction::transaction_object::TransactionObject;



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

    pub fn bind_objects(self, transaction_object: &TransactionObject) -> Self {
        self.imp().transaction_id.replace(transaction_object.id());
        let id_label = self.imp().transaction_id_btn.get();
        let data_inside_row_prefix = self.imp().data_inside_row_prefix.get();
        let data_inside_row_suffix = self.imp().data_inside_row_suffix.get();
        let image = self.imp().transaction_type.get();
        let data_action = self.imp().data_action.get();
        let data_inside_row = self.imp().data_inside_row.get();
        transaction_object
            .bind_property("id", &id_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("payee", &self, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("note", &data_inside_row, "subtitle")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("only-amount", &data_action, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("category-name", &data_inside_row_suffix, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("date-created", &self, "subtitle")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        match transaction_object.transaction_type().as_str() {
            "Income" => {
                image.set_icon_name(Some("zoom-in-symbolic"));
                image.add_css_class("success");
                data_action.add_css_class("success");
            }
            "Expense" => {
                image.set_icon_name(Some("zoom-out-symbolic"));
                image.add_css_class("error");
                data_action.add_css_class("error");
            }
            "Transfer In" => {
                image.add_css_class("success");
                image.set_icon_name(Some("network-receive-symbolic"));
                data_action.add_css_class("success");
            }
            "Transfer Out" => {
                image.set_icon_name(Some("network-transmit-symbolic"));
                image.add_css_class("error");
                data_action.add_css_class("error");
            }
            _ => {
                println!("What! {}", transaction_object.transaction_type().as_str());
            }
        }
        data_action.add_css_class("caption-heading");
        self
    }
}