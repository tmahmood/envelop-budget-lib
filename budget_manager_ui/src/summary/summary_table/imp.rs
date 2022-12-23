use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};

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
impl ObjectImpl for SummaryTable {}

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

    #[template_callback(name = "popover_closed")]
    fn unset_toggle(&self) {
        self.toggle.set_active(false);
    }
}
