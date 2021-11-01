use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label, Entry};
use std::cell::RefCell;
use adw::ActionRow;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/transaction_row.ui")]
pub struct TransactionRow {
    #[template_child]
    pub data_row: TemplateChild<ActionRow>,
    #[template_child]
    pub note_label: TemplateChild<Label>,
    #[template_child]
    pub amount_label: TemplateChild<Label>,
    // Vector holding the bindings to properties of `TodoObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TransactionRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TransactionRow";
    type Type = super::TransactionRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for TransactionRow {}

// Trait shared by all widgets
impl WidgetImpl for TransactionRow {}

// Trait shared by all boxes
impl BoxImpl for TransactionRow {}
