use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, Button};
use std::cell::RefCell;
use std::rc::Rc;
use adw::glib::once_cell::sync::Lazy;
use adw::glib::subclass::Signal;
use serde_json::error::Category;
use budget_manager::budgeting::category::CategoryModel;

// Object holding the state
#[derive(Default, CompositeTemplate, Debug)]
#[template(file = "../../../resources/category_row.ui")]
pub struct CategoryRow {
    #[template_child]
    pub category_id_label: TemplateChild<Button>,
    #[template_child]
    pub btn_edit_category: TemplateChild<Button>,

    pub category_id: RefCell<i32>
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CategoryRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "CategoryRow";
    type Type = super::CategoryRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        super::CategoryRow::ensure_type();
        Self::bind_template(klass);
        Self::bind_template_callbacks(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


#[gtk::template_callbacks]
impl CategoryRow {

    #[template_callback]
    fn handle_edit_button_clicked(&self, btn: &Button) {
        let category_id = self.category_id.borrow().clone();
        self.obj()
            .emit_by_name::<()>("category-selected-for-edit", &[&category_id]);
    }

}

impl ObjectImpl for CategoryRow {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            // get calls after
            vec![
                Signal::builder("category-selected-for-edit")
                .param_types([i32::static_type()]).build(),
            ]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for CategoryRow {}

impl PreferencesRowImpl for CategoryRow {}

impl ListBoxRowImpl for CategoryRow {}

impl ActionRowImpl for CategoryRow {}
