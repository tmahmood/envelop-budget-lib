use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::glib::once_cell::sync::Lazy;
use budget_manager::budgeting::expense_category::ExpenseCategory;

// Object holding the state
#[derive(Default)]
pub struct ExpenseCategoryObject {
    pub data: Rc<RefCell<ExpenseCategory>>,
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
                ParamSpec::new_string(
                    "name",
                    "name",
                    "name",
                    None,
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_float(
                    "maxbudget",
                    "maxbudget",
                    "maxbudget",
                    0.0,
                    99999999.99,
                    0.0,
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let input_value = value.get().expect("The value needs to be of type `string`.");
                self.data.borrow_mut().set_name(input_value);
            }
            "maxbudget" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `float`.");
                self.data.borrow_mut().set_max_budget(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.data.borrow().get_name().to_value(),
            "maxbudget" => self.data.borrow().get_max_budget().to_value(),
            _ => unimplemented!(),
        }
    }
}
