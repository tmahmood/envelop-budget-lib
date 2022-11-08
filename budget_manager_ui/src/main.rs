use std::borrow::BorrowMut;
use gtk::prelude::*;
use std::env::args;
use gtk::prelude::*;
use adw::prelude::*;
use budget_manager;


use adw::{Application, ActionRow, HeaderBar, ViewStack, ViewStackPage, ViewSwitcher};
use adw::gdk::Display;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Box, ListBox, Orientation, Button, ApplicationWindow, StackPage, Stack, glib, CssProvider, StyleContext};
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


fn main() {
    let application = Application::builder()
        .application_id("com.gtk.budgetTracker")
        .build();
    application.connect_activate(build_ui);
    application.run();
}


fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.show();
}