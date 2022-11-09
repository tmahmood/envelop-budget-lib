mod imp;

use std::borrow::Borrow;
use adw::{ActionRow, Application};
use adw::gio::Settings;
use adw::glib::BindingFlags;
use adw::prelude::ComboRowExt;
use gtk::{glib, gio, NoSelection, SignalListItemFactory, Entry, ListItemFactory, ListView, ListBoxRow, Label, Dialog, DialogFlags, ResponseType, ToggleButton, Switch};
use gtk::builders::BoxBuilder;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::APP_ID;
use crate::expense_category::expense_category_object::ExpenseCategoryObject;
use crate::expense_category::expense_category_row::ExpenseCategoryRow;
use crate::new_transaction_dialog::NewTransactionDialog;
use crate::transaction::transaction_object::TransactionObject;
use crate::transaction::transaction_row::TransactionRow;

glib::wrapper! {
pub struct Window(ObjectSubclass<imp::Window>)
    @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn transactions(&self) -> gio::ListStore {
        self.imp().transactions.borrow().clone().unwrap()
    }

    fn expense_category(&self) -> gio::ListStore {
        self.imp().expense_categories.borrow().clone().unwrap()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp().settings.set(settings).expect("Settings should not be set before calling `setup_settings`.")
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("Settings should be set in `setup_settings`")
    }

    pub fn save_all_settings(&self) -> Result<(), glib::BoolError> {
        // Get the size of the window

        // Set the window state in `settings`
        // self.settings().set_int("window-width", size.0)?;
        // self.settings().set_int("window-height", size.1)?;
        // self.settings()
        //     .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn setup_transactions(&self) {
        let model = gio::ListStore::new(TransactionObject::static_type());
        self.imp().transactions.replace(Some(model));
        let selection_model = NoSelection::new(Some(&self.transactions()));
        self.imp().transactions_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let transaction_obj = obj.downcast_ref().expect("The object should be of type `TransactionObject`.");
                let row = window.create_transaction_row(transaction_obj);
                row.upcast()
            }),
        );
        self.set_transactions_list_visible(&self.transactions());
        self.transactions().connect_items_changed(
            clone!(@weak self as window => move |transactions, _, _, _| {
                window.set_transactions_list_visible(transactions);
            }),
        );
    }
    /// Assure that `transactions_list` is only visible
    /// if the number of tasks is greater than 0
    fn set_transactions_list_visible(&self, transactions: &gio::ListStore) {
        self.imp().transactions_list.set_visible(transactions.n_items() > 0);
    }

    fn create_transaction_row(&self, transaction_object: &TransactionObject) -> TransactionRow {
        let row = TransactionRow::new();
        let payee_label = row.imp().payee_label.get();
        let note_label = row.imp().note_label.get();
        let amount_label = row.imp().amount_label.get();
        let image = row.imp().transaction_type.get();
        if transaction_object.is_income() {
            image.set_icon_name(Some("zoom-in"));
        } else {
            image.set_icon_name(Some("zoom-out"));
        }
        transaction_object.bind_property("payee", &payee_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object.bind_property("note", &note_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object.bind_property("only_amount", &amount_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        row
    }


    fn setup_actions(&self) {
        // Create action to create new collection and add to action group "win"
        let action_new_list = gio::SimpleAction::new("new-transaction", None);
        action_new_list.connect_activate(clone!(@weak self as window => move |_, _| {
            window.new_transaction();
        }));
        self.add_action(&action_new_list);
    }

    fn new_transaction(&self) {
        // Create new Dialog
        let dialog = NewTransactionDialog::new(self);
        let dialog_button = dialog
            .widget_for_response(ResponseType::Accept)
            .expect("The dialog needs to have a widget for response type `Accept`.");
        dialog_button.set_sensitive(false);

        let entry_payee = dialog.imp().entry_payee.get();
        let entry_note = dialog.imp().entry_note.get();
        let entry_amount = dialog.imp().entry_amount.get();
        let toggle_income = dialog.imp().toggle_income.get();

        let safe_entry = |dialog: &NewTransactionDialog, current_entry: &Entry, is_num: bool| -> bool {
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
            if is_num && current_entry.text().parse::<f32>().is_err() {
                f(current_entry);
                return false;
            }
            dialog_button.set_sensitive(true);
            current_entry.remove_css_class("error");
            return true;
        };

        // Set entry's css class to "error", when there is not text in it
        entry_payee.connect_changed(clone!(@weak dialog, => move |entry| safe_entry(&dialog, entry, false);));
        entry_amount.connect_changed(clone!(@weak dialog, => move |entry| safe_entry(&dialog, entry, true);));
        entry_note.connect_changed(clone!(@weak dialog, => move |entry| safe_entry(&dialog, entry, false);));

        // Connect response to dialog
        dialog.connect_response(
            clone!(@weak self as window, @weak entry_payee => move |dialog, response| {
                let e1 = safe_entry(&dialog, &entry_payee, false);
                let e2 = safe_entry(&dialog, &entry_note, false);
                let e3 = safe_entry(&dialog, &entry_amount, true);
                if ! (e1 && e2 && e3 ) {
                    return;
                }
                dialog.destroy();
                // Return if the user chose a response different than `Accept`
                if response != ResponseType::Accept {
                    dialog.destroy();
                    return;
                }
                let payee = entry_payee.buffer().text();
                let note = entry_note.buffer().text();
                let amount = entry_amount.buffer().text().parse::<f32>().unwrap() * if toggle_income.state() { 1. } else { -1. };
                let transaction_object = TransactionObject::new(payee.clone(), note.clone(), amount);
                let transactions = window.transactions();
                transactions.append(&transaction_object);
            }),
        );
        dialog.present();
    }


    fn setup_callbacks(&self) {
        // let model = self.expense_category();
        // self.imp().expense_category_entry.connect_activate(clone!(@weak model => move |entry| {
        //     let buffer = entry.buffer();
        //     let content = buffer.text();
        //     let mut splited = str::split(&content, '#');
        //     let name = splited.next().unwrap().trim().to_string();
        //     let max_budget = splited.next().unwrap().trim().parse::<f32>().unwrap();
        //     let expense_category_object = ExpenseCategoryObject::new(name, max_budget);
        //     model.append(&expense_category_object);
        //     buffer.set_text("");
        // }));
    }
}
