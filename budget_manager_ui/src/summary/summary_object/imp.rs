use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use adw::glib::{ParamSpecDouble, ParamSpecFloat, ParamSpecString};
use gtk::glib::once_cell::sync::Lazy;


#[derive(Default)]
pub struct SummaryData {
    pub balance: String,
    pub transfer_in: String,
    pub transfer_out: String,
    pub total_income: String,
    pub total_expense: String,
}

// Object holding the state
#[derive(Default)]
pub struct SummaryObject {
    pub data: Rc<RefCell<SummaryData>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SummaryObject {
    const NAME: &'static str = "SummaryObject";
    type Type = super::SummaryObject;
}

// Trait shared by all GObjects
impl ObjectImpl for SummaryObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("balance").build(),
                ParamSpecString::builder("transfer-in").build(),
                ParamSpecString::builder("transfer-out").build(),
                ParamSpecString::builder("total-income").build(),
                ParamSpecString::builder("total-expense").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "balance" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().balance = input_value;
            },
            "transfer-in" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().transfer_in = input_value;
            },
            "transfer-out" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().transfer_out = input_value;
            },
            "total-income" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().total_income = input_value;
            },
            "total-expense" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().total_expense = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "balance" => self.data.borrow().balance.to_value(),
            "transfer-in" => self.data.borrow().transfer_in.to_value(),
            "transfer-out" => self.data.borrow().transfer_out.to_value(),
            "total-income" => self.data.borrow().total_income.to_value(),
            "total-expense" => self.data.borrow().total_expense.to_value(),
            _ => unimplemented!(),
        }
    }
}
