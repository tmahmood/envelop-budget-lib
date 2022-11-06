mod imp;

use std::borrow::Borrow;
use adw::{ActionRow, Application};
use adw::glib::BindingFlags;
use adw::prelude::ComboRowExt;
use gtk::{glib, gio, NoSelection, SignalListItemFactory, Entry, ListItemFactory, ListView, ListBoxRow, Label};
use gtk::ffi::{GtkEntry, GtkLabel};
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::expense_category::expense_category_object::ExpenseCategoryObject;
use crate::expense_category::expense_category_row::ExpenseCategoryRow;
use crate::transaction::transaction_object::TransactionObject;
use crate::transaction::transaction_row::TransactionRow;

glib::wrapper! {
pub struct Window(ObjectSubclass<imp::Window>)
    @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

pub fn callback(entry: Entry) {}

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

        transaction_object.bind_property("payee", &payee_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object.bind_property("note", &note_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        transaction_object.bind_property("amount", &amount_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        row
    }


    fn setup_callbacks(&self) {
        let model = self.transactions();
        let payee_entry = self.imp().transaction_payee.get();
        let note_entry = self.imp().transaction_note.get();
        let amount_entry = self.imp().transaction_amount.get();


        self.imp().add_transaction_details.connect_clicked(clone!(@weak model => move | _ | {
            let payee = payee_entry.buffer().text();
            let note = note_entry.buffer().text();
            let amount = amount_entry.buffer().text().parse::<f32>().unwrap();
            let transaction_object = TransactionObject::new(payee.clone(), note.clone(), amount);
            model.append(&transaction_object);
            payee_entry.set_text("");
            note_entry.set_text("");
            amount_entry.set_text("");
        }));

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
