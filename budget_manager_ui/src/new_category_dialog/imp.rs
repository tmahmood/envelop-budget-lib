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

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "../../resources/new_category_dialog.ui")]
pub struct NewCategoryDialog {
    #[template_child]
    pub entry_category_name: TemplateChild<Entry>,

    #[template_child]
    pub entry_amount: TemplateChild<SpinButton>,

    #[template_child]
    pub amount_adjustment: TemplateChild<Adjustment>,

    // Vector holding the bindings to properties of `TransactionObject`
    pub(crate) categories: RefCell<Vec<String>>,
    pub(crate) category: RefCell<Option<Category>>,
}

impl NewCategoryDialog {
    pub fn set_fields(&self) {
        let c = self.category.borrow();
        if c.is_none() {
            return;
        }
        let category = c.as_ref().unwrap();
        self.entry_category_name.set_text(&category.name());
        self.entry_amount.set_value(category.allocated());
    }
}
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NewCategoryDialog {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NewCategoryDialog";
    type Type = super::NewCategoryDialog;
    type ParentType = gtk::Dialog;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for NewCategoryDialog {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            // get calls after
            vec![Signal::builder("valid-category-entered")
                .param_types([
                    Variant::static_type(),
                    Variant::static_type(),
                    Variant::static_type(),
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

        self.entry_category_name.connect_changed(
            clone!(@weak self as dialog, @weak dialog_button => move|entry| if !entry.text().is_empty() || !dialog.categories.borrow().contains(&entry.text().to_string()) {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );

        self.entry_amount.connect_changed(
            clone!(@weak dialog_button => move|entry| if !entry.value().is_nan() {
                entry.remove_css_class("error");
                dialog_button.set_sensitive(true) }),
        );
    }
}

// Trait shared by all widgets
impl WidgetImpl for NewCategoryDialog {}

// Trait shared by all Windows
impl WindowImpl for NewCategoryDialog {}

impl DialogImpl for NewCategoryDialog {
    fn response(&self, response: ResponseType) {
        if response != ResponseType::Accept {
            self.obj().destroy();
            return;
        }
        // let's assume all is good
        let mut no_error = true;
        if self.entry_category_name.text().is_empty() {
            self.entry_category_name.add_css_class("error");
            no_error = false;
        }

        if self.entry_amount.value().is_nan() || self.entry_amount.value() == 0. {
            self.entry_amount.add_css_class("error");
            no_error = false;
        }

        if no_error {
            let name = self.entry_category_name.text();
            let amount = self.entry_amount.value();
            let category_opt = self.category.borrow().clone();

            if category_opt.is_none() {
                self.obj().emit_by_name::<()>(
                    "valid-category-entered",
                    &[
                        &name.to_variant(),
                        &amount.to_variant(),
                        &None::<i32>.to_variant(),
                    ],
                );
            } else {
                let category = category_opt.unwrap();
                let name = if name != category.name() {
                    name.to_variant()
                } else {
                    None::<String>.to_variant()
                };
                let amount = if amount != category.allocated() {
                    amount.to_variant()
                } else {
                    None::<f64>.to_variant()
                };
                self.obj().emit_by_name::<()>(
                    "valid-category-entered",
                    &[&name, &amount, &category.id().to_variant()],
                );
            }
        }
    }
}
