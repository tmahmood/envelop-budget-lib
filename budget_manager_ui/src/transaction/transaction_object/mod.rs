pub mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::Object;
use gtk::glib;
use budget_manager::budgeting::transaction::Transaction;

glib::wrapper! {
    pub struct TransactionObject(ObjectSubclass<imp::TransactionObject>);
}

impl TransactionObject {
    pub fn new(payee: String, note: String, amount: f32) -> Self {
        let only_amount = if amount > 0. { amount } else { -1. * amount };
        Object::builder()
            .property("payee", &payee)
            .property("note", &note)
            .property("amount", &amount)
            .property("only-amount", &only_amount)
            .build()
    }

    pub fn payee(&self) -> String {
        self.imp().data.borrow().get_payee()
    }

    pub fn note(&self) -> String {
        self.imp().data.borrow().get_note()
    }

    pub fn amount(&self) -> f32 {
        self.imp().data.borrow().get_amount()
    }

    pub fn only_amount(&self) -> f32 {
        self.imp().data.borrow().get_only_amount()
    }

    pub fn is_income(&self) -> bool {
        self.imp().data.borrow().is_income()
    }

    pub fn from_transaction_data(transaction_data: Transaction ) -> Self {
        Self::new(
            transaction_data.get_payee(),
            transaction_data.get_note(),
            transaction_data.get_amount()
        )
    }
}
