use adw::gio::Settings;
use adw::glib::signal::Inhibit;
use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::glib::subclass::InitializingObject;
use gtk::{glib, Entry, Popover, ToggleButton};

use adw::ToastOverlay;
use gtk::{gio, Button, Label, ListBox};
use std::cell::RefCell;

use gtk::gio::glib::once_cell::sync::OnceCell;
use gtk::CompositeTemplate;

use crate::summary::summary_table::SummaryTable;
use budget_manager::budgeting::Budgeting;
use crate::category::category_row::CategoryRow;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/main_window.ui")]
pub struct Window {
    #[template_child]
    pub add_transaction_details: TemplateChild<Button>,

    #[template_child]
    pub transactions_list: TemplateChild<ListBox>,

    #[template_child]
    pub categories_list: TemplateChild<ListBox>,

    #[template_child]
    pub summary_table: TemplateChild<SummaryTable>,

    #[template_child]
    pub entry_command: TemplateChild<Entry>,

    #[template_child]
    pub toast_overlay: TemplateChild<ToastOverlay>,

    #[template_child]
    pub command_input: TemplateChild<ToggleButton>,

    #[template_child]
    pub prompt_popover: TemplateChild<Popover>,

    pub transactions: RefCell<Option<gio::ListStore>>,
    pub categories: RefCell<Option<gio::ListStore>>,

    pub settings: OnceCell<Settings>,
    pub budgeting: RefCell<Budgeting>,
    pub current_category_id: RefCell<i32>,
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "BudgetAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        // Setup
        let obj = self.obj();
        obj.setup_budget_account();
        obj.update_budget_details();
        obj.setup_categories();
        obj.setup_transactions();
        obj.setup_actions();
        obj.setup_callbacks();

        // Connect to "clicked" signal of `button`
    }
}

impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Inhibit {
        // Save window size
        // self.obj()
        //     .save_all_settings()
        //     .expect("Failed to save settings");
        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for Window {}

// Trait shared by all application
impl AdwApplicationWindowImpl for Window {}
