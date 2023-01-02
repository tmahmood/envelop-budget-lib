use crate::from_gdate_to_naive_date_time;
use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use adw::glib::{Date, DateTime, GString};
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use budget_manager::schema::transactions::date_created;
use chrono::{Datelike, NaiveDate, NaiveDateTime, ParseResult, Timelike};
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
        from_gdate_to_naive_date_time(_d)
    }

    pub fn date_o(&self) -> Option<DateTime> {
        if self.calendar_button_label.text().is_empty() {
            return None;
        }
        Some(self.id_calendar.date())
    }

    pub fn set_date(&self, date: NaiveDate) {
        let g = DateTime::from_local(
            date.year(),
            date.month() as i32,
            date.day() as i32,
            0,
            0,
            0.,
        )
        .unwrap();
        let today = budget_manager::current_date().date();
        if date == today {
            let date = date.format("%Y-%m-%d").to_string();
            self.set_date_field(date);
        }
        self.id_calendar.select_day(&g);
    }

    fn set_date_field(&self, date: String) {
        self.calendar_button_label.set_text(&date);
        self.placeholder.hide();
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
        let date = calendar.date().format("%Y-%m-%d").unwrap().to_string();
        self.set_date_field(date);
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

    fn constructed(&self) {
        let date = self.id_calendar.date().format("%Y-%m-%d").unwrap().to_string();
        self.set_date_field(date);
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
