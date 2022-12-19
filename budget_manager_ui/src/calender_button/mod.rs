mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CalendarButton(ObjectSubclass<imp::CalendarButton>) @extends gtk::Widget;
}
