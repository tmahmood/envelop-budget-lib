mod imp;

use crate::calender_button::CalendarButton;
use crate::transaction::transaction_object::TransactionObject;
use adw::glib::{clone, closure_local};
use adw::subclass::prelude::ObjectSubclassIsExt;
use chrono::{NaiveDate, NaiveDateTime};
use glib::Object;
use gtk::glib::DateTime;
use gtk::prelude::*;
use gtk::{glib, Button, Editable, Entry, ResponseType, SpinButton, StringList};
use budget_manager::budgeting::category::Category;
use budget_manager::budgeting::transaction::Transaction;

use crate::window::Window;

glib::wrapper! {
    pub struct NewTransactionDialog(ObjectSubclass<imp::NewTransactionDialog>)
    @extends gtk::Dialog, gtk::Window, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl NewTransactionDialog {
    pub fn new(parent: &Window, categories: Vec<Category>, transaction: Option<Transaction>) -> Self {
        let d: Self = Object::builder()
            .property("use-header-bar", 1.to_value())
            .property("destroy-with-parent", true.to_value())
            .build();
        d.set_transient_for(Some(parent));
        d.set_default_response(ResponseType::Accept);
        d.imp().set_transaction_and_categories(transaction, categories, parent.current_category_id());
        d
    }

}
