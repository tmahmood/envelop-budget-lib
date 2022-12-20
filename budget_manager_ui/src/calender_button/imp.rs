use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use adw::glib::{Date, DateTime};
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use chrono::{NaiveDate, NaiveDateTime, ParseResult};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use std::cell::RefCell;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "../../resources/calendar_button.ui")]
pub struct CalendarButton {
    #[template_child]
    pub toggle: TemplateChild<gtk::ToggleButton>,

    #[template_child]
    pub popover: TemplateChild<gtk::Popover>,

    #[template_child]
    calendar_button_label: TemplateChild<gtk::Label>,

    #[template_child]
    id_calendar: TemplateChild<gtk::Calendar>,

    #[template_child]
    placeholder: TemplateChild<gtk::Label>,
}

impl CalendarButton {
    pub fn set_placeholder(&self, text: &str) {
        self.placeholder.set_label(text);
    }

    pub fn date(&self) -> Option<NaiveDateTime> {
        if self.calendar_button_label.text().is_empty() {
            return None;
        }
        let _d = self.id_calendar.date();
        NaiveDate::from_ymd_opt(_d.year(), _d.month() as u32, _d.day_of_month() as u32)
            .unwrap()
            .and_hms_opt(0, 0, 0)
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
        self.calendar_button_label.set_text(&date);
        self.placeholder.hide();
        self.popover.hide();
        let obj = self.obj();
        obj.emit_by_name::<()>("calendar-button-date-changed", &[&calendar.date()]);
    }

    #[template_callback(name = "popover_closed")]
    fn unset_toggle(&self) {
        self.toggle.set_active(false);
    }
}

impl ObjectImpl for CalendarButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("calendar-button-date-changed")
                .param_types([DateTime::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }

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
