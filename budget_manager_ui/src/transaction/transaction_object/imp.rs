use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use adw::glib::{ParamSpecBoolean, ParamSpecFloat, ParamSpecString};
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
}

// Trait shared by all GObjects
impl ObjectImpl for TransactionObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("payee").build(),
                ParamSpecString::builder("note").build(),
                ParamSpecFloat::builder("amount").default_value(0.0).build(),
                ParamSpecString::builder("category-name").default_value(budget_manager::DEFAULT_CATEGORY).build(),
                ParamSpecFloat::builder("only-amount").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "payee" => {
                let input_value = value.get().expect("The value needs to be of type `string`.");
                self.data.borrow_mut().set_payee(input_value);
            }
            "note" => {
                let input_value = value.get().expect("the value needs to be of type `string`.");
                self.data.borrow_mut().set_note(input_value);
            }
            "amount" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `float`.");
                self.data.borrow_mut().set_amount(input_value);
            },
            "category-name" => {
                let input_value = value.get().expect("the value needs to be of type `string`.");
                self.data.borrow_mut().set_category_name(input_value);
            }
            "only-amount" => {},
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "note" => self.data.borrow().note().to_value(),
            "amount" => self.data.borrow().amount().to_value(),
            "payee" => self.data.borrow().payee().to_value(),
            "only-amount" => self.data.borrow().only_amount().to_value(),
            "category-name" => self.data.borrow().category_name().to_value(),
            _ => unimplemented!(),
        }
    }
}
