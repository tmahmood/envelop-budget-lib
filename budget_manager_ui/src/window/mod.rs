mod imp;

use adw::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use adw::{ActionRow, Application};
use adw::ffi::AdwHeaderBar;
use adw::gio::Settings;
use adw::glib::{BindingFlags, closure_local};
use adw::prelude::ComboRowExt;
use gtk::{glib, gio, NoSelection, SignalListItemFactory, Entry, ListItemFactory, ListView, ListBoxRow, Label, Dialog, DialogFlags, ResponseType, ToggleButton, Switch};
use gtk::builders::BoxBuilder;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use budget_manager::budgeting::budget_account::BudgetAccount;
use budget_manager::budgeting::transaction::Transaction;
use budget_manager::budgeting::transaction_category::TransactionCategory;
use crate::APP_ID;
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

    pub fn setup_budget_account(&self) {
        // this section is a stab, in reality, it will be loaded from data file.
        let mut budget = BudgetAccount::new("main", 10000.0, vec![
            ("Bills", 3000.),
            ("Travel", 2000.),
        ]);
        budget.new_expense(Some("Bills"), 300.34, "Uber", "someplace");
        budget.new_expense(Some("Travel"), 1300.23, "Foodpanda", "food");
        budget.new_expense(None, 1000., "SCB", "Card payment");
        budget.new_income(None, 5000., "Work", "Some payment");
        budget.new_income(Some("Travel"), 400., "UP", "Salary");
        // end stab
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
        let transactions = self.transactions();
        self.set_transactions_list_visible(&transactions);
        self.transactions().connect_items_changed(
            clone!(@weak self as window => move |transactions, _, _, _| {
                window.set_transactions_list_visible(transactions);
            }),
        );
        budget.all_transactions().iter().for_each(|transaction| {
            let transaction_object = TransactionObject::from_transaction_data(transaction);
            transactions.append(&transaction_object);
        });

        self.imp().budget.replace(budget);
    }

    fn update_budget_details(&self) {
        let mut budget = self.imp().budget.borrow_mut();

        let budget_details_available = self.imp().budget_details_available.get();
        budget_details_available.set_title(&budget.total_balance().to_string());

        let budget_total_income = self.imp().budget_total_income.get();
        budget_total_income.set_text(&budget.total_income().to_string());

        let budget_total_expense = self.imp().budget_total_expense.get();
        budget_total_expense.set_text(&budget.total_expense().to_string());

        let budget_unallocated = self.imp().budget_unallocated.get();
        budget_unallocated.set_text(&budget.unallocated().to_string());
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
        let category_name_label = row.imp().category_name_label.get();
        let image = row.imp().transaction_type.get();
        if transaction_object.is_income() {
            row.imp().amount_label.set_css_classes(&["success"]);
            image.set_icon_name(Some("go-up"));
        } else {
            row.imp().amount_label.set_css_classes(&["error"]);
            image.set_icon_name(Some("go-down"));
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
        transaction_object.bind_property("category-name", &category_name_label, "label")
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

        let safe_entry = |dialog: &NewTransactionDialog,
                          current_entry: &Entry, is_num: bool,
                          e1: &Entry, e2: &Entry, e3: &Entry| -> bool {
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
            if e1.text().is_empty() || e2.text().is_empty() || e3.text().is_empty() {
                dialog_button.set_sensitive(false);
            } else {
                dialog_button.set_sensitive(true);
            }
            current_entry.remove_css_class("error");
            return true;
        };

        entry_payee.connect_changed(clone!(
            @weak dialog, @weak entry_payee, @weak entry_amount, @weak entry_note =>
            move |entry|safe_entry(&dialog, entry, false, &entry_amount, &entry_note, &entry_payee);));
        entry_amount.connect_changed(clone!(
            @weak dialog, @weak entry_payee, @weak entry_amount, @weak entry_note =>
            move |entry|safe_entry(&dialog, entry, false, &entry_amount, &entry_note, &entry_payee);));
        entry_note.connect_changed(clone!(
            @weak dialog, @weak entry_payee, @weak entry_amount, @weak entry_note =>
            move |entry|safe_entry(&dialog, entry, false, &entry_amount, &entry_note, &entry_payee);));

        let on_dialog_action = move |window: &Window, dialog: &NewTransactionDialog,
                                     response: ResponseType, payee: String, note: String, amount: f32, is_income: bool| {
            dialog.destroy();
            // TODO must replace with actual transaction category
            let category = None;
            {
                let mut budget = window.imp().budget.borrow_mut();
                let t = if is_income {
                    budget.new_income(category, amount, &payee, &note)
                } else {
                    budget.new_expense(category, amount, &payee, &note)
                }.unwrap();
                let transactions = window.transactions();
                transactions.append(&TransactionObject::from_transaction_data(t));
            }
            dialog.emit_by_name::<()>("budget-updated", &[&1]);
        };

        // Connect response to dialog
        dialog.connect_response(clone!(
            @weak self as window, @weak entry_payee => move |dialog, response| {
                // Return if the user chose a response different than `Accept`
                if response != ResponseType::Accept {
                    dialog.destroy();
                    return;
                }
                let payee = entry_payee.buffer().text();
                let note = entry_note.buffer().text();
                let amount = entry_amount.buffer().text().parse::<f32>().unwrap();
                on_dialog_action(&window, dialog, response, payee, note, amount, toggle_income.state());
            }
        ));

        let update_subtitle_and_other_things = clone!(@weak self as window => move || {
            window.update_budget_details()
        });

        dialog.connect_closure(
            "budget-updated", false,
            closure_local!(move |_:NewTransactionDialog, _: i32| update_subtitle_and_other_things()));

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
