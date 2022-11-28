use adw::prelude::*;
use budget_manager;
use gtk::prelude::*;
use gtk::prelude::*;
use std::borrow::BorrowMut;
use std::env::args;

use adw::gdk::Display;
use adw::gio::Settings;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{gio, ActionRow, Application, HeaderBar, ViewStack, ViewStackPage, ViewSwitcher};
use gtk::glib::Object;
use gtk::{glib, Box, Button, CssProvider, ListBox, Orientation, Stack, StackPage, StyleContext};
use rand::{thread_rng, Rng};

use crate::transaction::transaction_object::TransactionObject;
use crate::window::Window;

mod expense_category;
mod new_transaction_dialog;
mod transaction;
mod window;

const APP_ID: &str = "org.tmn.budgetTracker";

fn main() {
    gio::resources_register_include!("app.gresource").expect("Failed to register resources.");

    let application = Application::builder().application_id(APP_ID).build();
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
