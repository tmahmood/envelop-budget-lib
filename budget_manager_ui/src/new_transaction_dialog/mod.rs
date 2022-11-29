mod imp;

use glib::{Object};
use gtk::prelude::*;
use gtk::{glib, ResponseType};

use crate::window::Window;


glib::wrapper! {
    pub struct NewTransactionDialog(ObjectSubclass<imp::NewTransactionDialog>)
    @extends gtk::Dialog, gtk::Window, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}


impl NewTransactionDialog {
    pub fn new(parent: &Window) -> Self {
        let d: Self = Object::builder()
            .property("use-header-bar", 1.to_value())
            .property("destroy-with-parent", true.to_value())
            .build();
        d.set_transient_for(Some(parent));
        d.set_default_response(ResponseType::Accept);
        d
    }
}