pub mod imp;
use gtk::glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ExpenseCategoryObject(ObjectSubclass<imp::ExpenseCategoryObject>);
}

impl ExpenseCategoryObject {
    pub fn new(name: String, allocated: f64) -> Self {
        Object::builder()
            .property("name", &name)
            .property("allocated", &allocated)
            .build()
    }
}
