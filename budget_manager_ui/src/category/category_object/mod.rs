pub mod imp;

use std::borrow::BorrowMut;
use gtk::glib::Object;
use gtk::glib;
use budget_manager::budgeting::category::CategoryModel;
use crate::fix_float;

glib::wrapper! {
    pub struct CategoryObject(ObjectSubclass<imp::CategoryObject>);
}

impl CategoryObject {
    pub fn new(cm: &mut CategoryModel) -> Self {
        let allocated = fix_float(cm.allocated());
        let balance = fix_float(cm.allocated());
        Object::builder()
            .property("id", &cm.category().id())
            .property("name", &cm.category().name())
            .property("allocated", allocated)
            .property("balance", balance)
            .build()
    }
}
