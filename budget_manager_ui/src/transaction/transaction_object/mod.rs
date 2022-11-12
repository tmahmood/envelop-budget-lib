pub mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::Object;
use gtk::glib;
use budget_manager::budgeting::transaction::Transaction;

glib::wrapper! {
    pub struct TransactionObject(ObjectSubclass<imp::TransactionObject>);
}

impl TransactionObject {
    pub fn new(payee: String, note: String, amount: f32, category: &str) -> Self {
        let only_amount = if amount > 0. { amount } else { -1. * amount };
        Object::builder()
            .property("payee", &payee)
            .property("note", &note)
            .property("amount", &amount)
            .property("only-amount", &only_amount)
            .property("category-name", &category)
            .build()
    }

    pub fn category_name(&self) -> String {
        self.imp().data.borrow().category_name().to_string()
    }

    pub fn payee(&self) -> String {
        self.imp().data.borrow().payee()
    }

    pub fn note(&self) -> String {
        self.imp().data.borrow().note()
    }

    pub fn amount(&self) -> f32 {
        self.imp().data.borrow().amount()
    }

    pub fn only_amount(&self) -> f32 {
        self.imp().data.borrow().only_amount()
    }

    pub fn is_income(&self) -> bool {
        self.imp().data.borrow().is_income()
    }

    pub fn from_transaction_data(transaction_data: &Transaction) -> Self {
        Self::new(
            transaction_data.payee(),
            transaction_data.note(),
            transaction_data.amount(),
            transaction_data.category_name(),
        )
    }
}
