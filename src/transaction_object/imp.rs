use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::glib::once_cell::sync::Lazy;
use budget_manager::budgeting::transaction::Transaction;

// Object holding the state
#[derive(Default)]
pub struct TransactionObject {
    pub data: Rc<RefCell<Transaction>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TransactionObject {
    const NAME: &'static str = "TransactionObject";
    type Type = super::TransactionObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for TransactionObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_string(
                    "note",
                    "note",
                    "note",
                    None,
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_float(
                    "amount",
                    "amount",
                    "amount",
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
            "note" => {
                let input_value = value.get().expect("The value needs to be of type `string`.");
                self.data.borrow_mut().set_note(input_value);
            }
            "amount" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `float`.");
                self.data.borrow_mut().set_amount(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "note" => self.data.borrow().get_note().to_value(),
            "amount" => self.data.borrow().get_amount().to_value(),
            _ => unimplemented!(),
        }
    }
}
