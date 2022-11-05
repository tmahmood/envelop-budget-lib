pub mod imp;
use gtk::glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TransactionObject(ObjectSubclass<imp::TransactionObject>);
}

impl TransactionObject {
    pub fn new(payee: String, note: String, amount: f32) -> Self {
        Object::builder()
            .property("payee", &payee)
            .property("note", &note)
            .property("amount", &amount)
            .build()
    }
}
