mod imp;

use gtk::{Application, glib, gio, NoSelection, SignalListItemFactory, Entry};
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::transaction_object::TransactionObject;
use crate::transaction_row::TransactionRow;

glib::wrapper! {
pub struct Window(ObjectSubclass<imp::Window>)
    @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

pub fn callback(entry: Entry) {}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create window")
    }

    fn model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(self);
        imp.model.get().expect("Could not get model")
    }

    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TransactionObject::static_type());

        // Get state and set model
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.model()));
        imp.list_view.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        // Get state
        let imp = imp::Window::from_instance(self);
        let model = self.model();
        // Setup callback so that activation
        // creates a new todo object and clears the entry
        imp.entry.connect_activate(clone!(@weak model => move |entry| {
            let buffer = entry.buffer();
            let content = buffer.text();
            let mut splited = str::split(&content, '#');
            let note = splited.next().unwrap().trim().to_string();
            let amount = splited.next().unwrap().trim().parse::<f32>().unwrap();

            let transaction_object = TransactionObject::new(note, amount);
            model.append(&transaction_object);
            buffer.set_text("");
        }));
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TodoRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TodoRow`
            let transaction_row = TransactionRow::new();
            list_item.set_child(Some(&transaction_row));
        });

        // Tell factory how to bind `TodoRow` to a `TodoObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TodoObject` from `ListItem`
            let transaction_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TransactionObject>()
                .expect("The item has to be an `Transaction`.");

            // Get `TransactionRow` from `ListItem`
            let transaction_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TransactionRow>()
                .expect("The child has to be a `TransactionRow`.");

            transaction_row.bind(&transaction_object);
        });

        // Tell factory how to unbind `TodoRow` from `TodoObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TransactionRow>()
                .expect("The child has to be a `TransactionRow`.");

            todo_row.unbind();
        });

        // Set the factory of the list view
        let imp = imp::Window::from_instance(self);
        imp.list_view.set_factory(Some(&factory));
    }
}
