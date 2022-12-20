mod imp;

use crate::calender_button::CalendarButton;
use crate::transaction::transaction_object::TransactionObject;
use adw::glib::{clone, closure_local};
use adw::subclass::prelude::ObjectSubclassIsExt;
use chrono::{NaiveDate, NaiveDateTime};
use glib::Object;
use gtk::glib::DateTime;
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

    pub fn setup_dialog_action(&self, window: &Window) {
        let entry_payee = self.imp().entry_payee.get();
        let entry_note = self.imp().entry_note.get();
        let entry_amount = self.imp().entry_amount.get();
        let toggle_income = self.imp().toggle_income.get();
        let entry_date = self.imp().transaction_date.get();

        self.connect_closure(
            "valid-transaction-entered",
            false,
            closure_local!(@watch window => move |dialog: NewTransactionDialog| {
                let payee = entry_payee.buffer().text();
                let note = entry_note.buffer().text();
                let amount = entry_amount.value();
                let date = entry_date.imp().date().unwrap();
                dialog.destroy();
                {
                    let current_id = window.current_category_id();
                    let mut budgeting = window.imp().budgeting.borrow_mut();
                    let category_name = {
                        let mut cm = budgeting.get_category_model_by_id(current_id).unwrap();
                        cm.category().name()
                    };
                    let mut tb = budgeting.new_transaction_to_category(&category_name);
                    if toggle_income.is_active() {
                        tb.income(amount);
                    } else {
                        tb.expense(amount);
                    }
                    let transaction = tb.payee(&payee).date_created(date).note(&note).done();
                    let mut tm = budgeting.transaction_model(transaction.clone());
                    let transactions = window.transactions();
                    transactions.append(&TransactionObject::new(&mut tm));
                }
                window.update_budget_details();
            }),
        );
    }
}
