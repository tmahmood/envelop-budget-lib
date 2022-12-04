pub mod imp;
use gtk::glib::Object;
use gtk::glib;
use budget_manager::budgeting::category::CategoryModel;

glib::wrapper! {
    pub struct CategoryObject(ObjectSubclass<imp::CategoryObject>);
}

impl CategoryObject {
    pub fn new(cm: &mut CategoryModel) -> Self {
        Object::builder()
            .property("id", &cm.category().id())
            .property("name", &cm.category().name())
            .property("allocated", &cm.allocated())
            .build()
    }
}
