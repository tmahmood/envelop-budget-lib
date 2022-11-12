use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use adw::glib::ParamSpecFloat;
use gtk::glib::once_cell::sync::Lazy;
use budget_manager::budgeting::transaction_category::TransactionCategory;
use crate::glib::ParamSpecString;

// Object holding the state
#[derive(Default)]
pub struct ExpenseCategoryObject {
    pub data: Rc<RefCell<TransactionCategory>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ExpenseCategoryObject {
    const NAME: &'static str = "ExpenseCategoryObject";
    type Type = super::ExpenseCategoryObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for ExpenseCategoryObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("name").default_value(None).build(),
                ParamSpecFloat::builder("allocated")
                    .minimum(0.0)
                    .maximum(9999999.99)
                    .default_value(0.0)
                    .build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let input_value = value.get().expect("The value needs to be of type `string`.");
                self.data.borrow_mut().set_name(input_value);
            }
            "allocated" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `float`.");
                self.data.borrow_mut().set_allocated(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.data.borrow().name().to_value(),
            "maxbudget" => self.data.borrow().allocated().to_value(),
            _ => unimplemented!(),
        }
    }
}
