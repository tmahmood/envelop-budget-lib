use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry, Switch, Label};
use std::cell::RefCell;
use std::ptr::NonNull;

use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::{Signal, TypeData};
use adw::glib::Type;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/summary.ui")]
pub struct SummaryTable {

    #[template_child]
    pub balance: TemplateChild<Label>,

    #[template_child]
    pub transfer_in: TemplateChild<Label>,

    #[template_child]
    pub transfer_out: TemplateChild<Label>,

    #[template_child]
    pub total_income: TemplateChild<Label>,

    #[template_child]
    pub total_expense: TemplateChild<Label>,

    // Vector holding the bindings to properties of `Budget Object`
    pub bindings: RefCell<Vec<Binding>>,
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SummaryTable {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SummaryTable";
    type Type = super::SummaryTable;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SummaryTable {}

// Trait shared by all widgets
impl WidgetImpl for SummaryTable {}

// Trait shared by all boxes
impl BoxImpl for SummaryTable {}
