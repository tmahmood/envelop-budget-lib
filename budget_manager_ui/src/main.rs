use adw::prelude::*;

use adw::{gio, Application};
use chrono::{NaiveDate, NaiveDateTime};
use gtk::glib::DateTime;

use crate::window::Window;

mod summary;
mod category;
mod new_transaction_dialog;
mod transaction;
mod window;
mod calender_button;
mod new_category_dialog;
mod budget_account;

const APP_ID: &str = "org.tmn.budgetTracker";

fn main() {
    gio::resources_register_include!("app.gresource").expect("Failed to register resources.");

    let application = Application::builder().application_id(APP_ID).build();
    application.connect_startup(setup_shortcuts);
    application.connect_activate(build_ui);
    application.run();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.new-transaction", &["<Ctrl>a"]);
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.show();
}

fn fix_float(float: f64) -> String {
    // format with the given computed precision
    format!("{0:.2}", float)
}

fn from_gdate_to_naive_date_time(_d: DateTime) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(_d.year(), _d.month() as u32, _d.day_of_month() as u32)
        .unwrap()
        .and_hms_opt(0, 0, 0)
}

pub fn date_time_to_string(date: NaiveDateTime) -> String {
    date.format("%l %p on %b %-d, %Y").to_string()
}

pub fn date_to_string(date: NaiveDateTime) -> String {
    date.format("%b %-d, %Y").to_string()
}
