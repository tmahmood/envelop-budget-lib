use adw::gio;
use adw::glib::{clone, closure_local, GStr, GString, Type, Variant};
use glib::Binding;
use gtk::glib::DateTime;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib, Adjustment, CompositeTemplate, DropDown, Entry, Label, ResponseType, SpinButton,
    StringList, Switch, ToggleButton,
};
use std::cell::RefCell;

use crate::calender_button::CalendarButton;
use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use budget_manager::budgeting::category::Category;
use budget_manager::budgeting::transaction::{Transaction, TransactionType};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../resources/new_transaction_dialog.ui")]
pub struct NewTransactionDialog {
    #[template_child]
    pub entry_payee: TemplateChild<Entry>,

    #[template_child]
    pub entry_note: TemplateChild<Entry>,

    #[template_child]
    pub toggle_income: TemplateChild<ToggleButton>,

    #[template_child]
    pub transaction_date: TemplateChild<CalendarButton>,

    #[template_child]
    pub entry_amount: TemplateChild<SpinButton>,

    #[template_child]
    pub amount_adjustment: TemplateChild<Adjustment>,

    #[template_child]
    category_list: TemplateChild<DropDown>,

    // Vector holding the bindings to properties of `TransactionObject`
    categories: RefCell<Vec<Category>>,

    pub category_selected: RefCell<String>,
    pub(crate) transaction: RefCell<Option<Transaction>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NewTransactionDialog {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NewTransactionDialog";
    type Type = super::NewTransactionDialog;
    type ParentType = gtk::Dialog;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for NewTransactionDialog {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            // get calls after
            vec![Signal::builder("valid-transaction-entered")
                .param_types([
                    Variant::static_type(),  // payee
                    Variant::static_type(),  // note
                    Variant::static_type(),  // amount
                    Variant::static_type(),  // is_income
                    DateTime::static_type(), // date
                    Variant::static_type(),  // category name
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();

        let dialog_button = self
            .obj()
            .widget_for_response(ResponseType::Accept)
            .expect("The dialog needs to have a widget for response type `Accept`.");
        dialog_button.set_sensitive(false);

        self.entry_payee.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.text().is_empty() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        self.entry_note.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.text().is_empty() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        self.entry_amount.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.value().is_nan() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        self.transaction_date.connect_closure(
            "calendar-button-date-changed",
            false,
            closure_local!(move |_b: CalendarButton, date: DateTime| {
                _b.remove_css_class("error");
                dialog_button.set_sensitive(true)
            }),
        );

        self.toggle_income
            .connect_toggled(clone!(@weak self as dialog => move |btn| {
                if btn.is_active() {
                    dialog.category_list.set_sensitive(false);
                }else {
                    dialog.category_list.set_sensitive(true);
                }
            }));

        self.category_list
            .connect_selected_notify(clone!(@weak self as dialog => move |d| {
                let selected = d.selected();
                let c = dialog.categories.borrow();
                let name = c.get(selected as usize).unwrap().name().to_string();
                dialog.category_selected.replace(name);
            }));
    }
}

// Trait shared by all widgets
impl WidgetImpl for NewTransactionDialog {}

// Trait shared by all Windows
impl WindowImpl for NewTransactionDialog {}

impl DialogImpl for NewTransactionDialog {
    fn response(&self, response: ResponseType) {
        if response != ResponseType::Accept {
            self.obj().destroy();
            return;
        }
        // let's assume all is good
        let mut no_error = true;
        if self.entry_payee.text().is_empty() {
            self.entry_payee.add_css_class("error");
            no_error = false;
        }

        if self.entry_note.text().is_empty() {
            self.entry_note.add_css_class("error");
            no_error = false;
        }

        if self.entry_amount.value().is_nan() || self.entry_amount.value() == 0. {
            self.entry_amount.add_css_class("error");
            no_error = false;
        }

        if self.transaction_date.imp().date().is_none() {
            self.transaction_date.add_css_class("error");
            no_error = false;
        }

        if no_error {
            let payee = self.entry_payee.get().text();
            let note = self.entry_note.get().text();
            let amount = self.entry_amount.get().value();
            let is_income = self.toggle_income.get().is_active();
            let date = self.transaction_date.get().imp().date_o().unwrap();
            let category_name = self.category_selected.borrow();

            self.obj().emit_by_name::<()>(
                "valid-transaction-entered",
                &[
                    &payee.to_variant(),
                    &note.to_variant(),
                    &amount.to_variant(),
                    &is_income.to_variant(),
                    &date,
                    &category_name.to_variant(),
                ],
            );
        }
    }
}

impl NewTransactionDialog {
    pub(crate) fn set_transaction_and_categories(
        &self,
        transaction: Option<Transaction>,
        categories: Vec<Category>,
        _category_id: i32,
    ) {
        self.transaction.replace(transaction);
        let t = self.transaction.borrow();
        let category_id = if t.is_some() {
            t.as_ref().and_then(|v| Some(v.category_id())).unwrap()
        } else {
            _category_id
        };
        let mut selected_category_index = 0;
        for (ii, category) in categories.iter().enumerate() {
            if category.id() == category_id {
                selected_category_index = ii as u32;
                break;
            }
        }
        self.categories.replace(categories);
        let store = self.categories.borrow();
        let c: StringList = store.iter().map(|v| v.name()).collect();
        self.category_list.get().set_model(Some(&c));
        self.category_list.set_selected(selected_category_index);
        let t = self.transaction.borrow();
        if t.is_none() {
            return;
        }
        let transaction = t.as_ref().unwrap();
        self.entry_payee.set_text(&transaction.payee());
        let v = transaction.amount();
        self.entry_amount
            .set_value(if v < 0. { v * -1. } else { v });
        self.entry_note.set_text(&transaction.note());
        self.transaction_date
            .imp()
            .set_date(transaction.date_created().date());
        match TransactionType::from(transaction.transfer_type_id()) {
            TransactionType::Income => {
                self.toggle_income.set_active(true);
                self.category_list.set_sensitive(false);
            }
            TransactionType::Expense => {
                self.toggle_income.set_active(false);
            }
            TransactionType::TransferIn | TransactionType::TransferOut => {}
        };
    }
}
