mod imp;

use crate::transaction::transaction_object::TransactionObject;
use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::prelude::*;
use gtk::{glib, Button, Editable, Entry, ResponseType, SpinButton};

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
        let date = self.imp().transaction_date.imp().date();

        let validate_input =
            |dialog: &NewTransactionDialog, e1: &Entry, e2: &Entry, a: &SpinButton| -> bool {
                let dialog_button = dialog
                    .widget_for_response(ResponseType::Accept)
                    .expect("The dialog needs to have a widget for response type `Accept`.");
                // let's assume all is good
                let mut no_error = true;
                if e1.text().is_empty() {
                    e1.add_css_class("error");
                    no_error = false;
                }
                if e2.text().is_empty() {
                    e2.add_css_class("error");
                    no_error = false;
                }
                if a.value().is_nan() || a.value() == 0. {
                    a.add_css_class("error");
                    no_error = false;
                }
                no_error
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

        entry_payee.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.text().is_empty() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        entry_note.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.text().is_empty() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        entry_amount.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.value().is_nan() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        // Connect response to dialog
        self.connect_response(clone!(
            @weak window, @weak entry_payee => move |dialog, response| {
                if response != ResponseType::Accept {
                    dialog.destroy();
                    return;
                }
                if validate_input(dialog, &entry_payee, &entry_note, &entry_amount) {
                    let payee = entry_payee.buffer().text();
                    let note = entry_note.buffer().text();
                    let amount = entry_amount.value();
                    on_dialog_action(&window, dialog, response, payee, note, amount, toggle_income.is_active());
                }
            }
        ));
    }
}
