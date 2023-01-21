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
#[template(file = "../../../resources/budget_account_row.ui")]
pub struct BudgetAccountRow {
    #[template_child]
    pub budget_account_id_label: TemplateChild<Button>,
    #[template_child]
    pub btn_edit_budget_account: TemplateChild<Button>,
    pub budget_account_id: RefCell<i32>
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for BudgetAccountRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "BudgetAccountRow";
    type Type = super::BudgetAccountRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        super::BudgetAccountRow::ensure_type();
        Self::bind_template(klass);
        Self::bind_template_callbacks(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


#[gtk::template_callbacks]
impl BudgetAccountRow {

    #[template_callback]
    fn handle_edit_button_clicked(&self, btn: &Button) {
        let budget_account_id = self.budget_account_id.borrow().clone();
        self.obj()
            .emit_by_name::<()>("budget-account-selected-for-edit", &[&budget_account_id]);
    }

}

impl ObjectImpl for BudgetAccountRow {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            // get calls after
            vec![
                Signal::builder("budget-account-selected-for-edit")
                .param_types([i32::static_type()]).build(),
            ]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for BudgetAccountRow {}

impl PreferencesRowImpl for BudgetAccountRow {}

impl ListBoxRowImpl for BudgetAccountRow {}

impl ActionRowImpl for BudgetAccountRow {}
