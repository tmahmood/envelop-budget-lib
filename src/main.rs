use gtk::prelude::*;
use std::env::args;
use gtk::prelude::*;
use adw::prelude::*;

use adw::{ActionRow, HeaderBar, ViewStack, ViewStackPage, ViewSwitcher};
use gtk::{Box, ListBox, Orientation, Button, Application, ApplicationWindow, StackPage, Stack, glib};
use gtk::glib::Object;
// use adw::{ActionRow, HeaderBar, ApplicationWindow};
// use gtk::{Box, ListBox, Orientation, Button, Application};

use budget_manager::budgeting::budget_account::BudgetAccount;
use crate::window::Window;

mod window;
mod transaction_row;
mod transaction_object;



fn main() {
    let application = Application::builder()
        .application_id("com.gtk.budgetTracker")
        .build();
    application.connect_startup(|_| {
        adw::init();
    });
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}