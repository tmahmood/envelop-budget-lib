use std::borrow::BorrowMut;
use gtk::prelude::*;
use std::env::args;
use gtk::prelude::*;
use adw::prelude::*;
use budget_manager;


use adw::{Application, ActionRow, HeaderBar, ViewStack, ViewStackPage, ViewSwitcher};
use adw::gdk::Display;
use adw::gio::Settings;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Box, ListBox, Orientation, Button, StackPage, Stack, glib, CssProvider, StyleContext};
use gtk::glib::Object;
use rand::{Rng, thread_rng};
// use adw::{ActionRow, HeaderBar, ApplicationWindow};
// use gtk::{Box, ListBox, Orientation, Button, Application};

use budget_manager::budgeting::budget_account::BudgetAccount;
use budget_manager::budgeting::transaction::Transaction;
use crate::transaction::transaction_object::TransactionObject;
use crate::window::Window;

mod window;
mod transaction;
mod expense_category;

const APP_ID: &str = "org.tmn.budgetTracker";

fn main() {
    let application = Application::builder()
        .application_id(APP_ID)
        .build();
    application.connect_startup(setup_shortcuts);
    application.connect_activate(build_ui);
    application.run();
}

fn setup_shortcuts(app: &Application) {
    &app.set_accels_for_action("win.new-transaction", &["<Ctrl>a"]);
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.show();
}