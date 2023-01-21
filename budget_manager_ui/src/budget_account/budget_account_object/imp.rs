use glib::{ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use adw::glib::{ParamSpecInt, ParamSpecString};
use gtk::glib::once_cell::sync::Lazy;

#[derive(Default)]
pub struct BudgetAccountInner {
    pub id: i32,
    pub filed_as: String,
    pub date_created: String,
}

// Object holding the state
#[derive(Default)]
pub struct BudgetAccountObject {
    pub data: Rc<RefCell<BudgetAccountInner>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for BudgetAccountObject {
    const NAME: &'static str = "BudgetAccountObject";
    type Type = super::BudgetAccountObject;
}

// Trait shared by all GObjects
impl ObjectImpl for BudgetAccountObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecInt::builder("id").build(),
                ParamSpecString::builder("filed-as").build(),
                ParamSpecString::builder("date-created").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "id" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `int`.");
                self.data.borrow_mut().id = input_value
            }
            "filed-as" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `string`.");
                self.data.borrow_mut().filed_as = input_value
            }
            "date-created" => {
                let input_value = value.get().expect("The value needs to be of type `String`.");
                self.data.borrow_mut().date_created = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "id" => self.data.borrow().id.to_value(),
            "filed-as" => self.data.borrow().filed_as.to_value(),
            "date-created" => self.data.borrow().date_created.to_value(),
            _ => unimplemented!(),
        }
    }
}
