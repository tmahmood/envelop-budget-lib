mod imp;

use crate::new_transaction_dialog::NewTransactionDialog;
use crate::transaction::transaction_object::TransactionObject;
use crate::transaction::transaction_row::TransactionRow;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Error;
use std::num::ParseFloatError;
use std::ops::Deref;

use adw::glib::{closure_local, BindingFlags};

use crate::category::category_object::CategoryObject;
use crate::category::category_row::CategoryRow;
use crate::fix_float;
use crate::summary::summary_object::imp::SummaryData;
use crate::summary::summary_object::SummaryObject;
use adw::builders::ToastBuilder;
use adw::prelude::*;
use adw::Application;
use budget_manager::budgeting::budgeting_errors::BudgetingErrors;
use budget_manager::budgeting::Budgeting;
use gtk::glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Entry, ListBox, ListBoxRow, NoSelection, ResponseType, ToggleButton};
use rand::distributions::uniform::SampleBorrow;

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
        self.imp().current_category_id.replace(1);
        let mut c = budget_manager::establish_connection();
        budget_manager::run_migrations(&mut c).expect("Failed to initialize database");
        let mut budgeting = Budgeting::new();
        // TODO: I should allow the user to create and load saved budgets
        budgeting
            .set_current_budget("main")
            .or_else(|_| budgeting.new_budget("main", 0.))
            .expect("Failed to get budget account");
        self.imp().budgeting.replace(budgeting);
    }

    fn setup_transactions(&self) {
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let model = gio::ListStore::new(TransactionObject::static_type());
        let cid = self.imp().current_category_id.borrow();
        let mut category = budgeting
            .get_category_model_by_id(cid.deref().clone())
            .unwrap();
        self.imp()
            .transaction_title
            .set_title(&category.category().name());
        category.transactions().iter().for_each(|transaction| {
            let mut tm = budgeting.transaction_model(transaction.clone());
            let transaction_object = TransactionObject::new(&mut tm);
            model.append(&transaction_object);
        });
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
        self.set_transactions_list_visible_only_when_there_are_transactions(&transactions);
        self.transactions().connect_items_changed(
            clone!(@weak self as window => move |transactions, _, _, _| {
                window.set_transactions_list_visible_only_when_there_are_transactions(transactions);
            }),
        );
    }

    fn setup_categories(&self) {
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let model = gio::ListStore::new(CategoryObject::static_type());
        budgeting.all_categories().iter().for_each(|category| {
            let mut cm = budgeting.category_model(category.clone());
            let category_object = CategoryObject::new(&mut cm);
            model.append(&category_object);
        });
        self.imp().categories.replace(Some(model));
        let selection_model = NoSelection::new(Some(&self.categories()));
        self.imp().categories_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let category_obj = obj.downcast_ref().expect("The object should be of type `CategoryObject`.");
                let row = window.create_category_row(category_obj);
                row.upcast()
            }),
        );
        let categories = self.categories();
        self.set_categories_list_visible_only_when_there_are_categories(&categories);
        self.categories().connect_items_changed(
            clone!(@weak self as window => move |categories, _, _, _| {
                window.set_categories_list_visible_only_when_there_are_categories(categories);
            }),
        );
    }

    pub(crate) fn update_budget_details(&self) {
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let cid = self.imp().current_category_id.borrow();
        let mut category = budgeting
            .get_category_model_by_id(cid.deref().clone())
            .unwrap();
        let total_expense = fix_float(category.expense());
        let total_income = fix_float(category.income());
        let transfer_in = fix_float(category.transfer_in());
        let transfer_out = fix_float(category.transfer_out());
        let balance = fix_float(category.balance());

        let summary_table = self.imp().summary_table.borrow().get();
        let summary_data = SummaryData {
            balance,
            transfer_in,
            transfer_out,
            total_income,
            total_expense,
        };
        let summary_object = SummaryObject::new(summary_data);
        summary_table.bind_summary(&summary_object);
    }

    pub(crate) fn transactions(&self) -> gio::ListStore {
        self.imp().transactions.borrow().clone().unwrap()
    }

    fn categories(&self) -> gio::ListStore {
        self.imp().categories.borrow().clone().unwrap()
    }

    pub(crate) fn current_category_id(&self) -> i32 {
        *self.imp().current_category_id.borrow().deref()
    }

    fn set_transactions_list_visible_only_when_there_are_transactions(
        &self,
        transactions: &gio::ListStore,
    ) {
        self.imp()
            .transactions_list
            .set_visible(transactions.n_items() > 0);
    }

    fn set_categories_list_visible_only_when_there_are_categories(
        &self,
        categories: &gio::ListStore,
    ) {
        self.imp()
            .categories_list
            .set_visible(categories.n_items() > 0);
    }

    fn create_category_row(&self, category_object: &CategoryObject) -> CategoryRow {
        CategoryRow::new().bind_objects(category_object)
    }

    fn create_transaction_row(&self, transaction_object: &TransactionObject) -> TransactionRow {
        TransactionRow::new().bind_objects(transaction_object)
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
        dialog.do_things(&self);
        dialog.present();
    }

    fn setup_callbacks(&self) {
        self.imp()
            .back_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.imp().leaflet.navigate(adw::NavigationDirection::Back);
            }));
    }

    fn show_toast(&self, text: &str) {
        let t = self.imp().toast_overlay.get();
        let toast = ToastBuilder::new().title(text).build();
        t.add_toast(&toast);
        t.show();
    }
}
