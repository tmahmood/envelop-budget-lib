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
    pub budget_details_available: String,
    pub budget_unallocated: String,
    pub budget_allocated: String,
    pub budget_total_income: String,
    pub budget_total_expense: String,
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
                ParamSpecString::builder("budget-details-available").build(),
                ParamSpecString::builder("budget-unallocated").build(),
                ParamSpecString::builder("budget-allocated").build(),
                ParamSpecString::builder("budget-total-income").build(),
                ParamSpecString::builder("budget-total-expense").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "budget-details-available" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().budget_details_available = input_value;
            },
            "budget-unallocated" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().budget_unallocated = input_value;
            },
            "budget-allocated" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().budget_allocated = input_value;
            },
            "budget-total-income" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().budget_total_income = input_value;
            },
            "budget-total-expense" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().budget_total_expense = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "budget-details-available" => self.data.borrow().budget_details_available.to_value(),
            "budget-unallocated" => self.data.borrow().budget_unallocated.to_value(),
            "budget-allocated" => self.data.borrow().budget_allocated.to_value(),
            "budget-total-income" => self.data.borrow().budget_total_income.to_value(),
            "budget-total-expense" => self.data.borrow().budget_total_expense.to_value(),
            _ => unimplemented!(),
        }
    }
}
