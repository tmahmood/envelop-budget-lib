use adw::glib::{ParamSpecDouble, ParamSpecInt, ParamSpecString};
use budget_manager::budgeting::transaction::{Transaction, TransactionModel, TransactionType};
use budget_manager::DEFAULT_CATEGORY;
use chrono::NaiveDateTime;
use glib::{ParamSpec, Value};
use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{date_to_string, fix_float};

#[derive(Default, Debug)]
pub struct TransactionInner {
    pub id: i32,
    pub note: String,
    pub payee: String,
    pub date_created: String,
    pub amount: String,
    pub only_amount: String,
    pub category_name: String,
    pub transaction_type: String,
}
// Object holding the state
#[derive(Default)]
pub struct TransactionObject {
    pub data: Rc<RefCell<TransactionInner>>,
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
                ParamSpecInt::builder("id").build(),
                ParamSpecString::builder("payee").build(),
                ParamSpecString::builder("note").build(),
                ParamSpecString::builder("amount")
                    .build(),
                ParamSpecString::builder("category-name").build(),
                ParamSpecString::builder("only-amount").build(),
                ParamSpecString::builder("date-created").build(),
                ParamSpecString::builder("transaction-type").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "id" => {
                let input_value = value.get().expect("The value needs to be of type `i32`.");
                self.data.borrow_mut().id = input_value;
            }
            "payee" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `string`.");
                self.data.borrow_mut().payee = input_value;
            }
            "note" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `string`.");
                self.data.borrow_mut().note = input_value;
            }
            "amount" => {
                let input_value = value.get().expect("The value needs to be of type `string`.");
                self.data.borrow_mut().amount = input_value;
            }
            "category-name" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `string`.");
                self.data.borrow_mut().category_name = input_value;
            }
            "only-amount" => {
                let input_value = value.get().expect("The value needs to be of type `string`.");
                self.data.borrow_mut().only_amount = input_value;
            }
            "transaction-type" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `string`.");
                self.data.borrow_mut().transaction_type = input_value;
            }
            "date-created" => {
                let input_value = value
                    .get()
                    .expect("the value needs to be of type `string`.");
                self.data.borrow_mut().date_created = input_value;
            }
            _ => {
                println!("{}", pspec.name());
                unimplemented!()
            },
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "id" => self.data.borrow().id.to_value(),
            "note" => self.data.borrow().note.to_value(),
            "amount" => self.data.borrow().amount.to_value(),
            "payee" => self.data.borrow().payee.to_value(),
            "only-amount" => self.data.borrow().only_amount.to_value(),
            "category-name" => self.data.borrow().category_name.to_value(),
            "date-created" => self.data.borrow().date_created.to_value(),
            "transaction-type" => self.data.borrow().transaction_type.to_value(),
            _ => unimplemented!(),
        }
    }
}

pub fn from_transaction_to_transfer_inner(tm: &mut TransactionModel, category_name: String) -> TransactionInner {
    let transfer_type = String::from(TransactionType::from(tm.transaction().transfer_type_id()));
    TransactionInner {
        id: tm.transaction().id(),
        note: tm.transaction().note(),
        payee: tm.transaction().payee(),
        date_created: date_to_string(tm.transaction().date_created()),
        amount: fix_float(tm.transaction().amount()),
        only_amount: fix_float(tm.transaction().only_amount()),
        category_name,
        transaction_type: transfer_type,
    }
}
