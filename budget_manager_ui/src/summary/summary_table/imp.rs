use std::cell::RefCell;
use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};
use gtk::ResponseType::No;
use budget_manager::budgeting::transaction::TransactionType;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/summary.ui")]
pub struct SummaryTable {
    #[template_child]
    pub transfer_in: TemplateChild<adw::ActionRow>,

    #[template_child]
    pub transfer_out: TemplateChild<adw::ActionRow>,

    #[template_child]
    pub total_income: TemplateChild<adw::ActionRow>,

    #[template_child]
    pub total_expense: TemplateChild<adw::ActionRow>,

    #[template_child]
    pub toggle: TemplateChild<gtk::ToggleButton>,

    #[template_child]
    pub popover: TemplateChild<gtk::Popover>,

    #[template_child]
    pub fund_overspent: TemplateChild<gtk::Button>,

    #[template_child]
    pub overspent_by: TemplateChild<adw::ActionRow>,

    #[template_child]
    pub fund_transfer_sb: TemplateChild<gtk::SpinButton>,

    #[template_child]
    pub fund_transfer_adjustment: TemplateChild<gtk::Adjustment>,

    #[template_child]
    pub allocation_adjustment_sb: TemplateChild<gtk::SpinButton>,

    #[template_child]
    pub allocation_adjustment: TemplateChild<gtk::Adjustment>,

    pub filter_by: RefCell<Option<TransactionType>>,

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
        Self::bind_template_callbacks(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SummaryTable {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("transaction-filter-changed")
                .build()]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.filter_by.replace(None);
    }
}

// Trait shared by all widgets
impl WidgetImpl for SummaryTable {}

// Trait shared by all boxes
impl BoxImpl for SummaryTable {}

#[gtk::template_callbacks]
impl SummaryTable {
    #[template_callback]
    fn toggle_toggled(&self, toggle: &gtk::ToggleButton) {
        if toggle.is_active() {
            self.popover.popup();
        }
    }

    #[template_callback]
    fn handle_filter_button_clicked(&self, btn: &gtk::CheckButton) {
        if !btn.is_active() { return }
        let _id = btn.buildable_id().unwrap();
        let id = _id.as_str();
        if id == "btn_all_transactions" {
            self.filter_by.replace(None);
        } else if id == "btn_funded" {
            self.filter_by.replace(Some(TransactionType::TransferIn));
        } else if id == "btn_transfer_out" {
            self.filter_by.replace(Some(TransactionType::TransferOut));
        } else if id == "btn_total_income" {
            self.filter_by.replace(Some(TransactionType::Income));
        } else {
            self.filter_by.replace(Some(TransactionType::Expense));
        }
        self.obj().emit_by_name::<()>("transaction-filter-changed", &[]);
    }

    #[template_callback(name = "popover_closed")]
    fn unset_toggle(&self) {
        self.toggle.set_active(false);
    }
}
