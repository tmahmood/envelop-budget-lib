use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry, Switch, ToggleButton, SpinButton};
use std::cell::RefCell;

use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use crate::calender_button::CalendarButton;
use crate::date_selector::DateSelector;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../resources/new_transaction_dialog.ui")]
pub struct NewTransactionDialog {
    #[template_child]
    pub entry_payee: TemplateChild<Entry>,

    #[template_child]
    pub entry_note: TemplateChild<Entry>,

    #[template_child]
    pub entry_amount: TemplateChild<SpinButton>,

    #[template_child]
    pub toggle_income: TemplateChild<ToggleButton>,

    #[template_child]
    pub date_selector_id: TemplateChild<CalendarButton>,

    // Vector holding the bindings to properties of `TransactionObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NewTransactionDialog {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NewTransactionDialog";
    type Type = super::NewTransactionDialog;
    type ParentType = gtk::Dialog;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for NewTransactionDialog {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("budget-updated")
                .param_types([i32::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}

// Trait shared by all widgets
impl WidgetImpl for NewTransactionDialog {}

// Trait shared by all Windows
impl WindowImpl for NewTransactionDialog {}

impl DialogImpl for NewTransactionDialog {}
