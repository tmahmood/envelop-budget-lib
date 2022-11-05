pub mod imp;
use gtk::glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ExpenseCategoryObject(ObjectSubclass<imp::ExpenseCategoryObject>);
}

impl ExpenseCategoryObject {
    pub fn new(name: String, max_budget: f32) -> Self {
        Object::builder()
            .property("name", &name)
            .property("maxbudget", &max_budget)
            .build()
    }
}
