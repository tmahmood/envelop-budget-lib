pub mod imp;
use gtk::glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TransactionObject(ObjectSubclass<imp::TransactionObject>);
}

impl TransactionObject {
    pub fn new(note: String, amount: f32) -> Self {
        Object::new(
            &[
                ("note", &note),
                ("amount", &amount)
            ])
            .expect("Failed to create `Transaction Object`.")
    }
}
