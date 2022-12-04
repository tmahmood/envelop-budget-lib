mod imp;

use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::prelude::*;
use gtk::{glib, Entry, ResponseType};
use crate::transaction::transaction_object::TransactionObject;

use crate::window::Window;

glib::wrapper! {
    pub struct NewTransactionDialog(ObjectSubclass<imp::NewTransactionDialog>)
    @extends gtk::Dialog, gtk::Window, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl NewTransactionDialog {
    pub fn new(parent: &Window) -> Self {
        let d: Self = Object::builder()
            .property("use-header-bar", 1.to_value())
            .property("destroy-with-parent", true.to_value())
            .build();
        d.set_transient_for(Some(parent));
        d.set_default_response(ResponseType::Accept);
        d
    }

    pub fn do_things(&self, window: &Window) {
        let dialog_button = self
            .widget_for_response(ResponseType::Accept)
            .expect("The dialog needs to have a widget for response type `Accept`.");
        dialog_button.set_sensitive(false);

        let entry_payee = self.imp().entry_payee.get();
        let entry_note = self.imp().entry_note.get();
        let entry_amount = self.imp().entry_amount.get();
        let toggle_income = self.imp().toggle_income.get();

        let safe_entry = |dialog: &NewTransactionDialog,
                          current_entry: &Entry,
                          is_num: bool,
                          e1: &Entry,
                          e2: &Entry,
                          e3: &Entry|
         -> bool {
            let dialog_button = dialog
                .widget_for_response(ResponseType::Accept)
                .expect("The dialog needs to have a widget for response type `Accept`.");

            let f = |entry: &Entry| {
                dialog_button.set_sensitive(false);
                entry.add_css_class("error");
            };
            if current_entry.text().is_empty() {
                f(current_entry);
                return false;
            }
            if is_num && current_entry.text().parse::<f64>().is_err() {
                f(current_entry);
                return false;
            }
            if e1.text().is_empty() || e2.text().is_empty() || e3.text().is_empty() {
                dialog_button.set_sensitive(false);
            } else {
                dialog_button.set_sensitive(true);
            }
            current_entry.remove_css_class("error");
            return true;
        };

        let on_dialog_action = move |window: &Window,
                                     dialog: &NewTransactionDialog,
                                     _response: ResponseType,
                                     payee: String,
                                     note: String,
                                     amount: f64,
                                     is_income: bool| {
            dialog.destroy();
            {
                let current_id = window.current_category_id();
                let mut budgeting = window.imp().budgeting.borrow_mut();
                let category_name = {
                    let mut cm = budgeting.get_category_model_by_id(current_id).unwrap();
                    cm.category().name()
                };
                let transaction = if is_income {
                    budgeting
                        .new_transaction_to_category(&category_name)
                        .income(amount)
                        .payee(&payee)
                        .note(&note)
                        .done()
                } else {
                    budgeting
                        .new_transaction_to_category(&category_name)
                        .expense(amount)
                        .payee(&payee)
                        .note(&note)
                        .done()
                };
                let mut tm = budgeting.transaction_model(transaction.clone());
                let transactions = window.transactions();
                transactions.append(&TransactionObject::new(&mut tm));
            }
            window.update_budget_details();
        };
        entry_payee.connect_changed(clone!(
            @weak self as dialog, @weak entry_payee, @weak entry_amount, @weak entry_note =>
            move |entry|safe_entry(&dialog, entry, false, &entry_amount, &entry_note, &entry_payee);));
        entry_amount.connect_changed(clone!(
            @weak self as dialog, @weak entry_payee, @weak entry_amount, @weak entry_note =>
            move |entry|safe_entry(&dialog, entry, false, &entry_amount, &entry_note, &entry_payee);));
        entry_note.connect_changed(clone!(
            @weak self as dialog, @weak entry_payee, @weak entry_amount, @weak entry_note =>
            move |entry|safe_entry(&dialog, entry, false, &entry_amount, &entry_note, &entry_payee);));

        // Connect response to dialog
        self.connect_response(clone!(
            @weak window, @weak entry_payee => move |dialog, response| {
                // Return if the user chose a response different than `Accept`
                if response != ResponseType::Accept {
                    dialog.destroy();
                    return;
                }
                let payee = entry_payee.buffer().text();
                let note = entry_note.buffer().text();
                let amount = entry_amount.buffer().text().parse::<f64>().unwrap();
                on_dialog_action(&window, dialog, response, payee, note, amount, toggle_income.state());
            }
        ));
    }
}
