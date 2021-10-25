use gtk::prelude::*;
use std::env::args;
use gtk::prelude::*;
use adw::prelude::*;

use adw::{ActionRow, HeaderBar, ViewStack, ViewStackPage, ViewSwitcher};
use gtk::{Box, ListBox, Orientation, Button, Application, ApplicationWindow, StackPage, Stack};
// use adw::{ActionRow, HeaderBar, ApplicationWindow};
// use gtk::{Box, ListBox, Orientation, Button, Application};

use budget_manager::budgeting::budget_account::BudgetAccount;


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

    let builder = gtk::Builder::from_string(include_str!("../resources/main_window.ui"));

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    // let button: Button = builder.object("button").expect("Couldn't get button");
    // let list: ListBox = builder.object("list").expect("Couldn't get list");
    // let row1: ActionRow = builder.object("row1").expect("Couldn't get action row");
    // let row2: ActionRow = builder.object("row2").expect("Couldn't get action row");

    window.set_application(Some(app));


    // row1.connect_activated(|_| {
    //     eprintln!("Clicked!");
    // });

    // row2.connect_activated(|_| {
    //     eprintln!("Other row Clicked!");
    // });

    // list.set_css_classes(&["content"]);

    // // Connect to "clicked" signal of `button`
    // button.connect_clicked(move |button| {
    //     // Set the label to "Hello World!" after the button has been clicked on
    //     button.set_label("Hello World!");
    // });

    window.show();
}