use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use adw::glib::{ParamSpecDouble, ParamSpecFloat, ParamSpecInt, ParamSpecString};
use gtk::glib::once_cell::sync::Lazy;
use budget_manager::budgeting::category::Category;

#[derive(Default)]
pub struct CategoryInner {
    pub id: i32,
    pub name: String,
    pub balance: String,
    pub allocated: String,
}

// Object holding the state
#[derive(Default)]
pub struct CategoryObject {
    pub data: Rc<RefCell<CategoryInner>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CategoryObject {
    const NAME: &'static str = "CategoryObject";
    type Type = super::CategoryObject;
}

// Trait shared by all GObjects
impl ObjectImpl for CategoryObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecInt::builder("id").build(),
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("allocated").build(),
                ParamSpecString::builder("balance").build(),
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
            "name" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `string`.");
                self.data.borrow_mut().name = input_value
            }
            "allocated" => {
                let input_value = value.get().expect("The value needs to be of type `String`.");
                self.data.borrow_mut().allocated = input_value
            }
            "balance" => {
                let input_value = value.get().expect("The value needs to be of type `String`.");
                self.data.borrow_mut().balance = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "id" => self.data.borrow().id.to_value(),
            "name" => self.data.borrow().name.to_value(),
            "allocated" => self.data.borrow().allocated.to_value(),
            "balance" => self.data.borrow().balance.to_value(),
            _ => unimplemented!(),
        }
    }
}
