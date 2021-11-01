pub mod imp;
use gtk::glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ExpenseCategoryObject(ObjectSubclass<imp::ExpenseCategoryObject>);
}

impl ExpenseCategoryObject {
    pub fn new(name: String, max_budget: f32) -> Self {
        Object::new(
            &[
                ("name", &name),
                ("maxbudget", &max_budget)
            ])
            .expect("Failed to create `Expense Category Object`.")
    }
}
