use std::cell::RefCell;
use std::rc::Rc;
use adw::ActionRow;
use adw::gio::Settings;
use adw::glib::signal::Inhibit;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{Button, Entry, gio, ListBox, ListView};
use gtk::cairo::glib::subclass::TypeData;
use gtk::glib::{Type, Value, ParamSpec, ParamFlags};
use gtk::glib::subclass::InitializingObject;
use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::gio::glib::once_cell::sync::OnceCell;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use budget_manager::budgeting::transaction::Transaction;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/main_window.ui")]
pub struct Window {
    #[template_child]
    pub add_transaction_details: TemplateChild<Button>,

    #[template_child]
    pub transactions_list: TemplateChild<ListBox>,

    #[template_child]
    pub expense_category_list_view: TemplateChild<ListBox>,

    pub transactions: RefCell<Option<gio::ListStore>>,
    pub expense_categories: RefCell<Option<gio::ListStore>>,

    pub settings: OnceCell<Settings>,
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
        self.obj()
            .save_all_settings()
            .expect("Failed to save settings");
        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for Window {}

// Trait shared by all application
impl AdwApplicationWindowImpl for Window {}

