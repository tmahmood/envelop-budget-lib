use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, Image, Button};
use std::cell::RefCell;
use adw::ActionRow;
use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use adw::subclass::action_row::ActionRowImpl;
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ExpanderRowImpl;


// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/transaction_row.ui")]
pub struct TransactionRow {
    #[template_child]
    pub data_action: TemplateChild<Label>,

    #[template_child]
    pub data_inside_row_prefix: TemplateChild<Label>,

    #[template_child]
    pub data_inside_row_suffix: TemplateChild<Label>,

    #[template_child]
    pub transaction_id_btn: TemplateChild<Button>,

    #[template_child]
    pub transaction_type: TemplateChild<Image>,

    #[template_child]
    pub data_inside_row: TemplateChild<ActionRow>,

    pub transaction_id: RefCell<i32>

}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TransactionRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TransactionRow";
    type Type = super::TransactionRow;
    type ParentType = adw::ExpanderRow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        Self::bind_template_callbacks(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl TransactionRow {

    #[template_callback]
    fn handle_edit_button_clicked(&self, btn: &Button) {
        let transaction_id = self.transaction_id.borrow().clone();
        self.obj()
            .emit_by_name::<()>("transaction-selected-for-edit", &[&transaction_id]);
    }

}


// Trait shared by all GObjects
impl ObjectImpl for TransactionRow {

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            // get calls after
            vec![
                Signal::builder("transaction-selected-for-edit")
                    .param_types([i32::static_type()]).build(),
            ]
        });
        SIGNALS.as_ref()
    }

}

// Trait shared by all widgets
impl WidgetImpl for TransactionRow {}

// Trait shared by all boxes
impl PreferencesRowImpl for TransactionRow {}

impl ListBoxRowImpl for TransactionRow {}

impl ExpanderRowImpl for TransactionRow {}
