use super::*;
use crate::test_helpers::{new_budget_using_budgeting, memory_db};
use crate::tests::{BILLS, INITIAL, TRAVEL, UNUSED};
use diesel::prelude::*;

#[test]
fn managing_multiple_budget_accounts() {
    // let mut dd = DbDropper::new();
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);

    blib.new_budget("savings", 10000.).unwrap();
    blib.new_budget("wallet", 5000.).unwrap();

    assert_eq!(blib.uncategorized_balance(), 5000.);
    assert_eq!(blib.actual_total_balance(), 5000.);

    assert!(blib.create_category("Bills", 2000., true).is_ok());
    assert!(blib.create_category("Travel", 3000., true).is_ok());

    assert_eq!(blib.uncategorized_balance(), 0.);
    assert_eq!(blib.actual_total_balance(), 5000.);

    assert!(blib
        .new_transaction_to_category("Bills")
        .expense(2000.)
        .payee("NO")
        .note("Internet")
        .done()
        .is_ok());

    assert_eq!(blib.actual_total_balance(), 3000.);
    assert_eq!(blib.total_expense(None).unwrap(), -2000.);
    blib.switch_budget_account("savings").unwrap();

    assert_eq!(blib.uncategorized_balance(), 10000.);
    assert_eq!(blib.actual_total_balance(), 10000.);
}

#[test]
fn allocating_money_behaviour() {
    let mut db = memory_db();
    let mut budgeting = Budgeting::new(db);
    assert!(budgeting.new_budget("main", 10000.).is_ok());
    assert!(budgeting.new_budget("wallet", 7000.).is_ok());
    assert!(budgeting
        .create_category("Bills", BILLS - 1000., true)
        .is_ok());
    assert!(budgeting
        .create_category("Travel", TRAVEL - 1000., true)
        .is_ok());
    assert_eq!(budgeting.uncategorized_balance(), 4000.);
    assert!(budgeting.switch_budget_account("main").is_ok());
    assert!(budgeting
        .transfer_fund(DEFAULT_CATEGORY, "Bills", 1000.)
        .is_ok());
    assert!(budgeting
        .transfer_fund(DEFAULT_CATEGORY, "Travel", 1000.)
        .is_ok());
    assert_eq!(budgeting.uncategorized_balance(), 8000.);
    assert!(budgeting.switch_budget_account("wallet").is_ok());
    assert_eq!(budgeting.uncategorized_balance(), 4000.);
}

#[test]
fn total_allocation_check() {
    let mut db = memory_db();
    let mut budgeting = Budgeting::new(db);
    new_budget_using_budgeting(&mut budgeting);
    assert_eq!(budgeting.total_allocated(), 5000.);
}

#[test]
fn total_balance_is_actual_money() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    // a transaction without any category
    let mut tc = blib
        .new_transaction_to_category(DEFAULT_CATEGORY)
        .expense(1000.)
        .payee("Some")
        .note("Other")
        .done();
    assert_eq!(blib.uncategorized_balance(), 4000.);
}

#[test]
fn transactions_in_default_category_should_change_balance() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let mut def = blib.new_transaction_to_category(DEFAULT_CATEGORY);
    def.expense(1000.)
        .payee("Some")
        .note("Other")
        .done()
        .unwrap();
    def.income(5000.)
        .payee("Some")
        .note("Other")
        .done()
        .unwrap();
    assert_eq!(blib.category_balance(DEFAULT_CATEGORY).unwrap(), -1000. + 10000.);
    assert_eq!(blib.category_balance("Bills").unwrap(), 2000.);
}

#[test]
pub fn total_balance_should_be_sum_of_all_categories_balance() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let mut travel = blib.new_transaction_to_category("Travel");
    let mut tb = travel.expense(1000.).payee("Some").note("Other").done();
    let mut tb = travel.income(500.).payee("Some").note("Other").done();
    assert_eq!(blib.actual_total_balance(), 5000. + BILLS + TRAVEL - 500.);
}

#[test]
fn finding_category_by_name_in_budget_account() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let bid = blib.current_budget().id();
    {
        let category = blib.find_category("Bills").unwrap();
        let mut bills = CategoryModel::new(blib.conn_mut(), category);
        assert_eq!(bills.allocated(), BILLS);
        assert_eq!(bills.balance(bid), BILLS);
    }
    let mut bills = blib.new_transaction_to_category("Bills");
    let mut tb = bills.income(500.).payee("Some").note("Other").done();
    {
        let category = blib.find_category("Bills").unwrap();
        let mut bills = CategoryModel::new(blib.conn_mut(), category);
        assert_eq!(bills.allocated(), BILLS);
        assert_eq!(bills.balance(bid), BILLS + 500.);
    }
}

#[test]
fn creating_category_and_do_transactions() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let _home = {
        let home = { blib.create_category("Home", 3000., true).unwrap() };
        assert_eq!(home.allocated(), 3000.0);
        assert_eq!(blib.category_balance("Home").unwrap(), 3000.0);
        home
    };
    let mut home_ops = blib.new_transaction_to_category("Home");
    home_ops
        .expense(2000.)
        .payee("someone")
        .note("test")
        .done()
        .unwrap();
    home_ops
        .income(1000.)
        .payee("another someone")
        .note("test some")
        .done()
        .expect("Error occurred");
    assert_eq!(blib.category_balance("Home").unwrap(), 2000.0);
    let mut cm = blib.get_category_model("Home");

    assert_eq!(cm.allocated(), 3000.);
    assert_eq!(blib.total_expense(Some("Home")).unwrap(), -2000.);
    assert_eq!(blib.total_income(Some("Home")).unwrap(), 1000.);
}

#[test]
pub fn spending_from_category() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let bills_available = blib.category_balance("Bills").unwrap();
    assert_eq!(bills_available, BILLS);
    assert_eq!(blib.actual_total_balance(), BILLS + TRAVEL + UNUSED);
    blib.new_transaction_to_category("Bills")
        .expense(BILLS)
        .payee("someone")
        .note("test")
        .done()
        .expect("Error occurred");
    let bills_available = blib.category_balance("Bills").unwrap();
    assert_eq!(bills_available, 0.0);
    assert_eq!(blib.actual_total_balance(), TRAVEL + UNUSED);
}

#[test]
pub fn funding_category_over_funded() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    blib.new_transaction_to_category("Bills")
        .expense(9000.)
        .payee("someone")
        .note("test")
        .done()
        .expect("Error occurred");
    assert_eq!(
        blib.fund_all_from_unallocated("Bills", false),
        Err(BudgetingErrors::OverFundingError)
    );
}

#[test]
pub fn funding_category_good() {
    let mut db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    blib.new_transaction_to_category("Bills")
        .expense(600.)
        .payee("someone")
        .note("test")
        .done()
        .expect("Error occurred");
    assert_eq!(blib.fund_all_from_unallocated("Bills", false), Ok(()));
    assert_eq!(blib.category_balance("Bills").unwrap(), BILLS);
}

#[test]
pub fn funding_category_as_much_as_possible() {
    let mut db = memory_db();
    let mut budgeting = Budgeting::new(db);
    budgeting
        .new_budget("main", 3000.)
        .expect("Error creating new budget");
    budgeting.create_category("Bills", 3100., false).unwrap();
    assert_eq!(
        budgeting.calculate_amount_to_fund(DEFAULT_CATEGORY, "Bills", false),
        Err(BudgetingErrors::OverFundingError)
    );
    assert_eq!(
        budgeting.calculate_amount_to_fund(DEFAULT_CATEGORY, "Bills", true),
        Ok(3000.)
    );
    budgeting
        .new_transaction_to_category("Bills")
        .expense(600.)
        .payee("someone")
        .note("test")
        .done()
        .expect("Error occurred");
    budgeting
        .new_transaction_to_category(DEFAULT_CATEGORY)
        .income(3000.)
        .payee("someone")
        .note("test")
        .done()
        .expect("Error occurred");
    assert_eq!(
        budgeting.calculate_amount_to_fund(DEFAULT_CATEGORY, "Bills", true),
        Ok(3700.)
    );
}
