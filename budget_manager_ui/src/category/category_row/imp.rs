use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, Button};
use std::cell::RefCell;
use std::rc::Rc;
use serde_json::error::Category;
use budget_manager::budgeting::category::CategoryModel;

// Object holding the state
#[derive(Default, CompositeTemplate, Debug)]
#[template(file = "../../../resources/category_row.ui")]
pub struct CategoryRow {
    #[template_child]
    pub category_id_label: TemplateChild<Button>,
    #[template_child]
    pub allocated_label: TemplateChild<Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CategoryRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "CategoryRow";
    type Type = super::CategoryRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        super::CategoryRow::ensure_type();
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CategoryRow {}

impl WidgetImpl for CategoryRow {}

impl PreferencesRowImpl for CategoryRow {}

impl ListBoxRowImpl for CategoryRow {}

impl ActionRowImpl for CategoryRow {}
