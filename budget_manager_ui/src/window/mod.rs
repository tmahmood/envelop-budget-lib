mod imp;

use crate::new_transaction_dialog::NewTransactionDialog;
use crate::transaction::transaction_object::TransactionObject;
use crate::transaction::transaction_row::TransactionRow;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Error;
use std::num::ParseFloatError;
use std::ops::Deref;

use adw::glib::{closure_local, BindingFlags, DateTime};

use crate::category::category_object::CategoryObject;
use crate::category::category_row::CategoryRow;
use crate::new_category_dialog::NewCategoryDialog;
use crate::summary::summary_object::imp::SummaryData;
use crate::summary::summary_object::SummaryObject;
use crate::summary::summary_table::SummaryTable;
use crate::{fix_float, from_gdate_to_naive_date_time};
use adw::builders::ToastBuilder;
use adw::prelude::*;
use adw::Application;
use budget_manager::budgeting::budgeting_errors::BudgetingErrors;
use budget_manager::budgeting::transaction::{TransactionModel, TransactionType};
use budget_manager::budgeting::Budgeting;
use budget_manager::DEFAULT_CATEGORY;
use gtk::glib::{clone, Object, Variant};
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, Entry, ListBox, ListBoxRow, NoSelection, ResponseType, StringList, ToggleButton,
};
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
        let mut conn = budget_manager::establish_connection();
        budget_manager::run_migrations(&mut conn).expect("Failed to initialize database");
        let mut budgeting = Budgeting::new();
        budgeting
            .set_current_budget("main")
            .or_else(|_| budgeting.new_budget("main", 10000.))
            .expect("Failed to get budget account");
        let c = budgeting.default_category();
        self.imp().current_category_id.replace(c.id());
        self.imp().budgeting.replace(budgeting);
        self.imp()
            .summary_table
            .get()
            .imp()
            .fund_overspent
            .get()
            .set_sensitive(false);
    }

    fn setup_transactions(&self) {
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let model = gio::ListStore::new(TransactionObject::static_type());
        let cid = self.imp().current_category_id.borrow();
        let mut category = budgeting
            .get_category_model_by_id(cid.deref().clone())
            .unwrap();

        let filter = self.imp().summary_table.imp().filter_by.borrow().clone();
        category
            .transactions()
            .iter()
            .filter(|v| {
                if let Some(x) = &filter {
                    &TransactionType::from(v.transfer_type_id()) == x
                } else {
                    true
                }
            })
            .for_each(|transaction| {
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
    }

    pub(crate) fn setup_summary_table(&self) {
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let cid = self.imp().current_category_id.borrow();
        let mut category = budgeting
            .get_category_model_by_id(cid.deref().clone())
            .unwrap();

        let expense = category.expense();
        let allocated = category.allocated();
        let expense_unsigned = expense * expense.signum();
        let _transfer_out = category.transfer_out();
        let total_expense = fix_float(expense_unsigned);
        let total_income = fix_float(category.income());
        let transfer_in = fix_float(category.transfer_in());
        let transfer_out = fix_float(_transfer_out * _transfer_out.signum());
        let b = category.balance();
        let balance = fix_float(b);
        let category_name = category.category().name();
        let heading = self.imp().transaction_title.get();
        heading.set_title(&category_name);
        let summary_table = self.imp().summary_table.borrow().get();
        summary_table.imp().fund_transfer_adjustment.set_value(0.);
        let fund_btn = summary_table.imp().fund_overspent.get();
        if category_name == DEFAULT_CATEGORY || b >= allocated {
            fund_btn.set_sensitive(false);
            fund_btn.set_label("No need");
        } else {
            let r = { budgeting.calculate_amount_to_fund(DEFAULT_CATEGORY, &category_name, true) };
            match r {
                Ok(fund) => summary_table.imp().fund_transfer_adjustment.set_value(fund),
                Err(_) => {}
            }
            fund_btn.set_sensitive(true);
            fund_btn.set_label("Fund");
        }
        let overspent_by = summary_table.imp().overspent_by.get();
        //summary_table
        if category_name != DEFAULT_CATEGORY && expense_unsigned >= allocated {
            heading.add_css_class("error");
            summary_table.add_css_class("error");
            let s = fix_float(expense_unsigned - allocated);
            overspent_by.set_title(&s);
            overspent_by.set_subtitle("Overspent, allocate more money to cover");
            overspent_by.add_css_class("error");
            summary_table
                .imp()
                .allocation_adjustment
                .set_value(expense_unsigned - allocated);
        } else {
            heading.remove_css_class("error");
            summary_table.remove_css_class("error");
            overspent_by.set_title("Looks good");
            overspent_by.set_subtitle("This category is under control");
            overspent_by.remove_css_class("error");
            summary_table.imp().allocation_adjustment.set_value(0.);
        }
        self.imp().summary_table.imp().toggle.set_label(&balance);
        let summary_data = SummaryData {
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
        let c = CategoryRow::new().bind_objects(category_object);
        c.connect_closure(
            "category-selected-for-edit",
            false,
            closure_local!(@watch self as window => move |_: &CategoryRow, category_id: i32| {
                window.category_form(Some(category_id));
            }),
        );
        c
    }

    fn delete_category(&self, category_id: i32) {
        // few things must be done
        // 1. remove all fund transfer
        let r = {
            let mut budget_account = self.imp().budgeting.borrow_mut();
            budget_account.delete_category(category_id)
        };
        match r {
            Ok(_) => {
                let cid = { self.imp().current_category_id.borrow().clone() };
                let default = { self.imp().budgeting.borrow_mut().default_category().id() };
                if cid == category_id {
                    self.imp().current_category_id.replace(default);
                }
                self.setup_categories();
                self.setup_summary_table();
                self.setup_transactions();
            }
            Err(BudgetingErrors::CategoryDeleteFailed) => {
                self.show_toast("Failed to delete category")
            }
            Err(e) => self.show_toast(&format!("Something went wrong. {:?}", e)),
        };
    }

    fn create_transaction_row(&self, transaction_object: &TransactionObject) -> TransactionRow {
        let t = TransactionRow::new().bind_objects(transaction_object);
        t.connect_closure(
            "transaction-selected-for-edit",
            false,
            closure_local!(@watch self as window => move |_: &TransactionRow, transaction_id: i32| {
                window.transaction_form(Some(transaction_id));
            }),
        );
        t
    }

    fn setup_actions(&self) {
        // Create action to create new collection and add to action group "win"
        let action_new_list = gio::SimpleAction::new("new-transaction", None);
        action_new_list.connect_activate(clone!(@weak self as window => move |_, _| {
            window.transaction_form(None);
        }));
        self.add_action(&action_new_list);

        let action_fund_transfer = gio::SimpleAction::new("fund-transfer", None);
        action_fund_transfer.connect_activate(clone!(@weak self as window => move |_, _| {
            window.fund_transfer();
            window.setup_summary_table();
            window.setup_transactions();
        }));
        self.add_action(&action_fund_transfer);

        let action_new_category = gio::SimpleAction::new("new-category", None);
        action_new_category.connect_activate(clone!(@weak self as window => move |_, _| {
            window.category_form(None);
        }));
        self.add_action(&action_new_category);
    }

    fn fund_transfer(&self) {
        let mut budget_account = self.imp().budgeting.borrow_mut();
        let cid = self.imp().current_category_id.borrow();
        let mut category = budget_account
            .get_category_model_by_id(cid.deref().clone())
            .unwrap();
        let category_name = category.category().name();
        let v = self
            .imp()
            .summary_table
            .imp()
            .fund_transfer_adjustment
            .value();
        match budget_account.check_if_funding_possible(DEFAULT_CATEGORY, v, true) {
            Ok(_) => match budget_account.transfer_fund(DEFAULT_CATEGORY, &category_name, v) {
                Ok(_) => self.show_toast("Fund transferred successfully"),
                Err(e) => self.show_toast(&format!("Error occurred: {:?}", e)),
            },
            Err(BudgetingErrors::OverFundingError) => {
                self.show_toast("You do not have enough money to fund this category")
            }
            Err(e) => self.show_toast(&format!("Something went wrong. {:?}", e)),
        }
    }

    fn transaction_form(&self, edit_id: Option<i32>) {
        // Create new Dialog
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let categories = budgeting.categories();
        let transaction =
            edit_id.and_then(|eid| match budgeting.get_transaction_model_by_id(eid) {
                Ok(tm) => Some(tm.transaction().clone()),
                Err(_) => None,
            });
        let dialog = NewTransactionDialog::new(self, categories, transaction);
        dialog.connect_closure(
            "valid-transaction-entered",
            false,
            closure_local!(@watch self as window => move |dialog: NewTransactionDialog,
                _payee: Variant,
                _note: Variant,
                _amount: Variant,
                _is_income: Variant,
                _date: DateTime,
                _category_name: Variant
                | {
                    window.save_transaction(
                        dialog,
                        _payee,
                        _note,
                        _amount,
                        _is_income,
                        _date,
                        _category_name,
                        edit_id
                    );
            }),
        );
        dialog.present();
    }

    fn save_transaction(
        &self,
        dialog: NewTransactionDialog,
        _payee: Variant,
        _note: Variant,
        _amount: Variant,
        _is_income: Variant,
        _date: DateTime,
        _category_name: Variant,
        _edit_id: Option<i32>
    ) {

        let payee = _payee.get::<String>().unwrap();
        let note = _note.get::<String>().unwrap();
        let amount = _amount.get::<f64>().unwrap();
        let is_income = _is_income.get::<bool>().unwrap();
        let date = from_gdate_to_naive_date_time(_date).unwrap();
        let category_name = _category_name.get::<String>().unwrap();
        dialog.destroy();
        let category_id = {
            let mut budgeting = self.imp().budgeting.borrow_mut();
            let cname = if is_income {
                DEFAULT_CATEGORY
            } else {
                &category_name
            };
            let mut tb = budgeting.new_transaction_to_category(cname);
            if is_income {
                tb.income(amount);
            } else {
                tb.expense(amount);
            }
            let t = match tb.payee(&payee).date_created(date).note(&note).done() {
                Ok(v) => v,
                Err(_) => {
                    self.show_toast("Failed to create Transaction");
                    return;
                }
            };
            t.category_id()
        };
        if self.current_category_id() == category_id {
            self.setup_summary_table();
            self.setup_transactions();
        }
    }

    fn category_form(&self, edit_id: Option<i32>) {
        // Create new Dialog
        let mut budgeting = self.imp().budgeting.borrow_mut();
        let categories = budgeting.categories();
        // All the categories are loaded already, so no need to access the database again
        // just search in the list of category and we're done
        let category = edit_id.and_then(|eid| {
            Some(match categories.binary_search_by(|v| v.id().cmp(&eid)) {
                Ok(category_index) => categories[category_index].clone(),
                Err(_) => {
                    self.show_toast("Failed to load category");
                    return None;
                }
            })
        });
        let dialog = NewCategoryDialog::new(self, categories, category);
        dialog.connect_closure(
            "valid-category-entered",
            false,
            closure_local!(@watch self as window => move |dialog: &NewCategoryDialog, name: Variant, amount: Variant, category_id: Variant | {
                window.save_category(dialog, name, amount, category_id);
            }),
        );
        dialog.present();
    }

    fn save_category(
        &self,
        dialog: &NewCategoryDialog,
        _name: Variant,
        _amount: Variant,
        category_id_v: Variant,
    ) {
        let cid = category_id_v.get::<i32>();
        let name = _name.get::<String>();
        let amount = _amount.get::<f64>();
        dialog.destroy();
        let r = {
            let mut budgeting = self.imp().budgeting.borrow_mut();
            if cid.is_some() {
                let category_id = cid.unwrap();
                budgeting.update_category(category_id, name, amount)
            } else {
                budgeting.create_category(&name.unwrap(), amount.unwrap(), false)
            }
        };
        match r {
            Ok(category_id) => {
                self.imp().current_category_id.replace(category_id);
                self.setup_categories();
                self.setup_summary_table();
                self.setup_transactions();
            }
            Err(e) => self.show_toast(&format!("{}", e)),
        };
    }

    fn setup_callbacks(&self) {

        self.transactions().connect_items_changed(
            clone!(@weak self as window => move |transactions, _, _, _| {
                window.set_transactions_list_visible_only_when_there_are_transactions(transactions);
            }),
        );

        self.categories().connect_items_changed(
            clone!(@weak self as window => move |categories, _, _, _| {
                window.set_categories_list_visible_only_when_there_are_categories(categories);
            }),
        );

        self.imp().summary_table.connect_closure(
            "transaction-filter-changed",
            false,
            closure_local!(@watch self as window => move |_: &SummaryTable| {
                window.setup_transactions()
            }),
        );
    }

    fn show_toast(&self, text: &str) {
        let t = self.imp().toast_overlay.get();
        let toast = ToastBuilder::new().title(text).build();
        t.add_toast(&toast);
        t.show();
    }
}
