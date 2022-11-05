mod imp;

use adw::Application;
use gtk::{glib, gio, NoSelection, SignalListItemFactory, Entry, ListItemFactory, ListView};
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::expense_category::expense_category_object::ExpenseCategoryObject;
use crate::expense_category::expense_category_row::ExpenseCategoryRow;
use crate::transaction::transaction_object::TransactionObject;
use crate::transaction::transaction_row::TransactionRow;

glib::wrapper! {
pub struct Window(ObjectSubclass<imp::Window>)
    @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

pub fn callback(entry: Entry) {}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(self);
        imp.model.get().expect("Could not get model")
    }

    fn expense_category_model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(self);
        imp.model_expense_categories.get().expect("Could not get model")
    }

    fn setup_model(&self) {
        let model = gio::ListStore::new(TransactionObject::static_type());
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");
        let selection_model = NoSelection::new(Some(self.model()));
        imp.list_view.set_model(Some(&selection_model));
        let model_ec = gio::ListStore::new(ExpenseCategoryObject::static_type());
        imp.model_expense_categories.set(model_ec).expect("Could not set model");
        let selection_model = NoSelection::new(Some(self.expense_category_model()));
        imp.expense_category_list_view.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        let imp = imp::Window::from_instance(self);
        let model = self.model();
        imp.transaction_entry.connect_activate(clone!(@weak model => move |entry| {
            let buffer = entry.buffer();
            let content = buffer.text();
            let mut splited = str::split(&content, '#');
            let payee = splited.next().unwrap().trim().to_string();
            let note = splited.next().unwrap().trim().to_string();
            let amount = splited.next().unwrap().trim().parse::<f32>().unwrap();

            let transaction_object = TransactionObject::new(payee, note, amount);
            model.append(&transaction_object);
            buffer.set_text("");
        }));
        let model = self.expense_category_model();
        imp.expense_category_entry.connect_activate(clone!(@weak model => move |entry| {
            let buffer = entry.buffer();
            let content = buffer.text();
            let mut splited = str::split(&content, '#');
            let name = splited.next().unwrap().trim().to_string();
            let max_budget = splited.next().unwrap().trim().parse::<f32>().unwrap();

            let expense_category_object = ExpenseCategoryObject::new(name, max_budget);
            model.append(&expense_category_object);
            buffer.set_text("");
        }));
    }

    fn setup_factory(&self) {
        self.setup_expense_category_factory();
        self.setup_transaction_list_factory();
    }

    fn setup_expense_category_factory(&self) {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let expense_category_row = ExpenseCategoryRow::new();
            list_item.set_child(Some(&expense_category_row));
        });
        factory.connect_bind(move |_, list_item| {
            let expense_category_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<ExpenseCategoryObject>()
                .expect("The item has to be an `ExpenseCategoryObject`.");
            let expense_category_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<ExpenseCategoryRow>()
                .expect("The child has to be a `ExpenseCategoryRow`.");
            expense_category_row.bind(&expense_category_object);
        });
        factory.connect_unbind(move |_, list_item| {
            let expense_category_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<ExpenseCategoryRow>()
                .expect("The child has to be a `ExpenseCategoryRow`.");
            expense_category_row.unbind();
        });
        let imp = imp::Window::from_instance(self);
        imp.expense_category_list_view.set_factory(Some(&factory));
    }

    fn setup_transaction_list_factory(&self) {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let transaction_row = TransactionRow::new();
            list_item.set_child(Some(&transaction_row));
        });
        factory.connect_bind(move |_, list_item| {
            let transaction_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TransactionObject>()
                .expect("The item has to be an `TransactionObject`.");
            let transaction_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TransactionRow>()
                .expect("The child has to be a `TransactionRow`.");

            transaction_row.bind(&transaction_object);
        });
        factory.connect_unbind(move |_, list_item| {
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TransactionRow>()
                .expect("The child has to be a `TransactionRow`.");
            todo_row.unbind();
        });
        let imp = imp::Window::from_instance(self);
        imp.list_view.set_factory(Some(&factory));
    }
}
