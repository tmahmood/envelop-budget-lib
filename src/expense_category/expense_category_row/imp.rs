use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label, Entry};
use std::cell::RefCell;
use adw::ActionRow;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/expense_category_row.ui")]
pub struct ExpenseCategoryRow {
    #[template_child]
    pub data_row: TemplateChild<ActionRow>,
    #[template_child]
    pub name_label: TemplateChild<Label>,
    #[template_child]
    pub max_budget_label: TemplateChild<Label>,
    // Vector holding the bindings to properties of `TodoObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ExpenseCategoryRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "ExpenseCategoryRow";
    type Type = super::ExpenseCategoryRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for ExpenseCategoryRow {}

// Trait shared by all widgets
impl WidgetImpl for ExpenseCategoryRow {}

// Trait shared by all boxes
impl BoxImpl for ExpenseCategoryRow {}
