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
use crate::summary::summary_object::SummaryObject;
use crate::summary::summary_table::SummaryTable;
use crate::transaction::transaction_object::imp::{
    from_transaction_to_transfer_inner, TransactionInner,
};
use crate::window::CommandLineErrors::ParsingFloatError;
use adw::builders::ToastBuilder;
use adw::prelude::*;
use adw::Application;
use budget_manager::budgeting::budgeting_errors::BudgetingErrors;
use budget_manager::budgeting::category::Category;
use budget_manager::budgeting::transaction::{Transaction, TransactionModel, TransactionType};
use budget_manager::budgeting::Budgeting;
use budget_manager::DEFAULT_CATEGORY;
use clap::builder::Str;
use gtk::ffi::GtkEntry;
use gtk::glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Entry, NoSelection, ResponseType, ListBox, ToggleButton};
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
        category.transactions().iter().for_each(|transaction| {
            // TODO: Category not set correctly
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
        budgeting.categories().iter().for_each(|category| {
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
        let summary_table = self.imp().summary_table.borrow().get();
        let summary_object = SummaryObject::new(&mut self.imp().budgeting.borrow_mut());
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

    fn set_transactions_list_visible_only_when_there_are_transactions(&self, transactions: &gio::ListStore) {
        self.imp()
            .transactions_list
            .set_visible(transactions.n_items() > 0);
    }

    fn set_categories_list_visible_only_when_there_are_categories(&self, categories: &gio::ListStore) {
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
        let on_entry_callback = |entry: &Entry, window: &Window| {
            let txt = entry.text();
            let (start, info) = match txt.split_once(" ") {
                None => {
                    window.show_toast("Please enter a valid command");
                    return;
                }
                Some(v) => v,
            };
            window.execute_command(start, info);
        };

        self.imp().entry_command.connect_activate(
            clone!(@weak self as window => move |entry| { on_entry_callback(entry, &window)}),
        );

    }

    fn show_toast(&self, text: &str) {
        let t = self.imp().toast_overlay.get();
        let toast = ToastBuilder::new().title(text).build();
        t.add_toast(&toast);
        t.show();
    }

    fn execute_command(&self, start: &str, info: &str) {
        let result = match start {
            "nc" => self.new_category_command(info),
            "sc" => self.select_category_command(info),
            // "nt" => self.new_transaction_command(info),
            _ => Err(CommandLineErrors::UnknownCommand),
        };
        let s = match result {
            Ok(s) => s,
            Err(CommandLineErrors::FailedToCreateCategory(e)) => e,
            Err(_) => "Unknown Error".to_string(),
        };
        if s != "" {
            self.show_toast(&s);
        }
    }

    fn select_category_command(&self, txt: &str) -> Result<String, CommandLineErrors> {
        let info = split_by_pattern(txt, " ");
        if info.len() != 1 {
            return Err(CommandLineErrors::IncompleteCommand);
        }
        let f = match info[0].parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                return Err(CommandLineErrors::FailedToSelectCategory(format!(
                    "Failed to parse int {}, {}",
                    info[1],
                    e.to_string()
                )))
            }
        };
        self.imp().current_category_id.replace(f);
        self.setup_transactions();
        Ok("".to_string())
    }

    fn new_category_command(&self, txt: &str) -> Result<String, CommandLineErrors> {
        let (category_name, amount) = parse_new_category_command(&txt)?;
        match {
            let mut budgeting = self.imp().budgeting.borrow_mut();
            budgeting.create_category_and_allocate(&category_name, amount)
        } {
            Ok(v) => {
                {
                    let mut budgeting = self.imp().budgeting.borrow_mut();
                    let model = self.categories();
                    let mut cm = budgeting.category_model(v.clone());
                    let category_object = CategoryObject::new(&mut cm);
                    model.append(&category_object);
                    self.imp().categories.replace(Some(model));
                }
                self.setup_transactions();
                self.update_budget_details();
                return Ok("New Category added".to_string());
            }
            Err(BudgetingErrors::FailedToCreateCategory(err_str)) => {
                Err(CommandLineErrors::FailedToCreateCategory(format!(
                    "Failed to add category: {}",
                    err_str
                )))
            }
            Err(BudgetingErrors::CategoryAlreadyExists) => Err(
                CommandLineErrors::FailedToCreateCategory("Category already exists".to_string()),
            ),
            _ => Err(CommandLineErrors::FailedToCreateCategory(
                "Failed to create category due to unknown error".to_string(),
            )),
        }
    }

    fn new_transaction_command(&self, txt: &str) {
        todo!()
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum CommandLineErrors {
    #[error("Failed to parse amount, must be valid float")]
    ParsingFloatError(String),
    #[error("Incomplete Command")]
    IncompleteCommand,
    #[error("Unknown Command")]
    UnknownCommand,
    #[error("Failed to create category")]
    FailedToCreateCategory(String),
    #[error("Failed to select category")]
    FailedToSelectCategory(String),
}

fn split_by_pattern(s: &str, p: &str) -> Vec<String> {
    s.split(p).map(|v: &str| v.trim().to_string()).collect()
}

fn parse_new_category_command(info: &str) -> Result<(String, f64), CommandLineErrors> {
    let info = split_by_pattern(info, "@");
    if info.len() < 2 {
        return Err(CommandLineErrors::IncompleteCommand);
    }
    let f = match info[1].parse::<f64>() {
        Ok(v) => v,
        Err(e) => {
            return Err(CommandLineErrors::FailedToCreateCategory(format!(
                "Failed to parse float {}, {}",
                info[1],
                e.to_string()
            )))
        }
    };
    Ok((info[0].clone(), f))
}

impl From<ParseFloatError> for CommandLineErrors {
    fn from(value: ParseFloatError) -> Self {
        ParsingFloatError(value.to_string())
    }
}

#[test]
pub fn test_parsing_command() {
    let cmd = "nc Bills @ 3000";
    let (n, a): (String, f64) = parse_new_category_command(cmd).unwrap();
    assert_eq!(n, "Bills");
    assert_eq!(a, 3000.);
}
