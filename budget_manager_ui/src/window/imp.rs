use adw::gio::Settings;
use adw::glib::signal::Inhibit;
use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::glib::subclass::InitializingObject;
use gtk::{glib, Entry, Popover, ToggleButton};

use adw::{ActionRow, ExpanderRow, Flap, Leaflet, NavigationDirection, ToastOverlay};
use gtk::{gio, Button, Label, ListBox};
use std::cell::RefCell;

use gtk::gio::glib::once_cell::sync::OnceCell;
use gtk::CompositeTemplate;

use crate::summary::summary_table::SummaryTable;
use budget_manager::budgeting::Budgeting;

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
    pub toast_overlay: TemplateChild<ToastOverlay>,

    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,

    #[template_child]
    pub back_button: TemplateChild<Button>,

    #[template_child]
    pub display_budget_accounts: TemplateChild<ToggleButton>,

    #[template_child]
    pub tgl_btn_new_category: TemplateChild<ToggleButton>,

    #[template_child]
    pub transaction_title: TemplateChild<adw::WindowTitle>,

    #[template_child]
    pub summary_table: TemplateChild<SummaryTable>,

    #[template_child]
    pub fund_overspent: TemplateChild<Button>,


    pub transactions: RefCell<Option<gio::ListStore>>,
    pub categories: RefCell<Option<gio::ListStore>>,

    pub settings: OnceCell<Settings>,
    pub budgeting: RefCell<Budgeting>,
    pub current_category_id: RefCell<i32>,
}

#[gtk::template_callbacks]
impl Window {

    #[template_callback]
    fn handle_row_activated(&self, list_box: &super::CategoryRow) {
        let id = list_box.imp().category_id_label.get().label().unwrap();
        self.current_category_id.replace(id.parse().unwrap());
        self.obj().setup_transactions();
        self.obj().setup_budget_details();
        self.leaflet.navigate(NavigationDirection::Forward);
    }
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "BudgetAppWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        Self::bind_template_callbacks(klass);
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
        obj.setup_budget_details();
        obj.setup_categories();
        obj.setup_transactions();
        obj.setup_actions();
        obj.setup_callbacks();
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
