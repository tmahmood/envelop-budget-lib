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
        let payee_label = self.imp().payee_label.get();
        let id_label = self.imp().transaction_id_btn.get();
        let note_label = self.imp().note_label.get();
        let amount_label = self.imp().amount_label.get();
        let category_name_label = self.imp().category_name_label.get();
        let image = self.imp().transaction_type.get();
        image.set_icon_size(IconSize::Large);
        let date_created_label = self.imp().date_created_label.get();
        match transaction_object.transaction_type().as_str() {
            "Income" => {
                self.imp().amount_label.set_css_classes(&["success"]);
                image.set_icon_name(Some("zoom-in-symbolic"));
            }
            "Expense" => {
                self.imp().amount_label.set_css_classes(&["error"]);
                image.set_icon_name(Some("zoom-out-symbolic"));
            }
            "Transfer In" => {
                self.imp().amount_label.set_css_classes(&["success"]);
                image.set_icon_name(Some("go-previous"));
            }
            "Transfer Out" => {
                self.imp().amount_label.set_css_classes(&["error"]);
                image.set_icon_name(Some("go-next"));
            }
            _ => {
                println!("What! {}", transaction_object.transaction_type().as_str());
            }
        }
        transaction_object
            .bind_property("id", &id_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("payee", &payee_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("note", &note_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("only-amount", &amount_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("category-name", &category_name_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object
            .bind_property("date-created", &date_created_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        self
    }
}