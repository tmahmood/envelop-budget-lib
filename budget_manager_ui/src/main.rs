use gtk::prelude::*;
use std::env::args;
use gtk::prelude::*;
use adw::prelude::*;
use budget_manager;


use adw::{Application, ActionRow, HeaderBar, ViewStack, ViewStackPage, ViewSwitcher};
use adw::gdk::Display;
use gtk::{Box, ListBox, Orientation, Button, ApplicationWindow, StackPage, Stack, glib, CssProvider, StyleContext};
use gtk::glib::Object;
// use adw::{ActionRow, HeaderBar, ApplicationWindow};
// use gtk::{Box, ListBox, Orientation, Button, Application};

use budget_manager::budgeting::budget_account::BudgetAccount;
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
    let mut window = Window::new(app);
    window.show();
}