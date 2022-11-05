use std::cell::RefCell;
use std::rc::Rc;
use adw::ActionRow;
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
#[template(file="../../resources/main_window.ui")]
pub struct Window {
    #[template_child]
    pub budget_balance: TemplateChild<Entry>,
    #[template_child]
    pub transaction_entry: TemplateChild<Entry>,
    #[template_child]
    pub expense_category_entry: TemplateChild<Entry>,
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub list_view: TemplateChild<ListView>,
    #[template_child]
    pub expense_category_list_view: TemplateChild<ListView>,
    pub model: OnceCell<gio::ListStore>,
    pub model_expense_categories: OnceCell<gio::ListStore>
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
        obj.setup_model();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

// Trait shared by all application
impl AdwApplicationWindowImpl for Window {}

