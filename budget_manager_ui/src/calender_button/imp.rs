use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "../../resources/calendar_button.ui")]
pub struct CalendarButton {
    #[template_child]
    pub toggle: TemplateChild<gtk::ToggleButton>,

    #[template_child]
    pub popover: TemplateChild<gtk::Popover>,

    #[template_child]
    calendar_button_label: TemplateChild<gtk::Label>,
}

impl CalendarButton {
    fn set_label(&self, label: String) {
        self.calendar_button_label.set_label(&label);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CalendarButton {
    const NAME: &'static str = "CalendarButton";
    type Type = super::CalendarButton;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl CalendarButton {
    #[template_callback]
    fn toggle_toggled(&self, toggle: &gtk::ToggleButton) {
        if toggle.is_active() {
            self.popover.popup();
        }
    }

    #[template_callback]
    fn day_selected(&self, calendar: &gtk::Calendar) {
        let date = calendar.date().format("%Y-%m-%d").unwrap();
        self.calendar_button_label.set_label(&date);
        self.popover.hide();
    }
    #[template_callback(name = "popover_closed")]
    fn unset_toggle(&self) {
        self.toggle.set_active(false);
    }
}

impl ObjectImpl for CalendarButton {
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CalendarButton {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
        self.popover.present();
    }
}

impl BoxImpl for CalendarButton {}
