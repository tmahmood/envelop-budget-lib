use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use super::*;
use crate::test_helpers::{memory_db, new_budget_using_budgeting, BILLS, INITIAL, TRAVEL, UNUSED};
use diesel::prelude::*;
use crate::parse_date;

#[test]
fn date_parsing_tests() {
    let nd = NaiveDate::from_str("2021-05-17").unwrap();
    let nt = NaiveTime::from_str("00:00:00").unwrap();
    println!("{} {}", nd, nt);
    assert_eq!(parse_date("2021-05-17"), NaiveDateTime::new(
        nd, nt
    ))
}
#[test]
fn managing_multiple_budget_accounts() {
    // let mut dd = DbDropper::new();
    let db = memory_db();
    let mut blib = Budgeting::new(db);

    blib.new_budget("savings", 10000.).unwrap();
    blib.new_budget("wallet", 5000.).unwrap();

    assert_eq!(blib.uncategorized_balance(), 15000.);
    assert_eq!(blib.actual_total_balance(), 15000.);

    assert!(blib.create_category("Bills", 7500., true).is_ok());
    assert!(blib.create_category("Travel", 7500., true).is_ok());

    assert_eq!(blib.uncategorized_balance(), 0.);
    assert_eq!(blib.actual_total_balance(), 15000.);

    assert!(blib
        .new_transaction_to_category("Bills").unwrap()
        .expense(2000.)
        .payee("NO")
        .note("Internet")
        .done()
        .is_ok());

    assert_eq!(blib.actual_total_balance(), 13000.);
    assert_eq!(blib.total_expense(None).unwrap(), -2000.);
    // this does not matter when counting category balance
    blib.switch_budget_account("savings").unwrap();

    assert_eq!(blib.uncategorized_balance(), 0.);
    assert_eq!(blib.actual_total_balance(), 13000.);
}

#[test]
fn allocating_money_behaviour() {
    let db = memory_db();
    let mut budgeting = Budgeting::new(db);
    let to_wallet = 7000.;
    let to_main = 10000.;
    let total_available = to_main + to_wallet;
    let to_bill = BILLS - 1000.;
    let to_travel = TRAVEL - 1000.;
    let after_allocation = total_available - to_bill - to_travel;

    budgeting.new_budget("main", to_main).unwrap();
    budgeting.new_budget("wallet", to_wallet).unwrap();
    budgeting.create_category("Bills", to_bill, true).unwrap();
    budgeting
        .create_category("Travel", to_travel, true)
        .unwrap();

    assert_eq!(budgeting.uncategorized_balance(), after_allocation);

    let after_transfer_to_category = after_allocation - 2000.;
    budgeting
        .transfer_fund(DEFAULT_CATEGORY, "Bills", 1000.)
        .unwrap();
    budgeting
        .transfer_fund(DEFAULT_CATEGORY, "Travel", 1000.)
        .unwrap();

    assert_eq!(
        budgeting.uncategorized_balance(),
        after_transfer_to_category
    );
}

#[test]
fn total_allocation_check() {
    let db = memory_db();
    let mut budgeting = Budgeting::new(db);
    new_budget_using_budgeting(&mut budgeting);
    assert_eq!(budgeting.total_allocated(), 5000.);
}

#[test]
fn total_balance_is_actual_money() {
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    // a transaction without any category
    blib.new_transaction_to_category(DEFAULT_CATEGORY).unwrap()
        .expense(1000.)
        .payee("Some")
        .note("Other")
        .done()
        .unwrap();
    assert_eq!(
        blib.uncategorized_balance(),
        15000. - BILLS - TRAVEL - 1000.
    );
}

#[test]
fn transactions_in_default_category_should_change_balance() {
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let mut def = blib.new_transaction_to_category(DEFAULT_CATEGORY).unwrap();
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
    assert_eq!(blib.actual_total_balance(), INITIAL - 1000. + 5000.);
    assert_eq!(blib.category_balance("Bills").unwrap(), 2000.);
}

#[test]
pub fn total_balance_should_be_sum_of_all_categories_balance() {
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let mut travel = blib.new_transaction_to_category("Travel").unwrap();
    travel
        .expense(1000.)
        .payee("Some")
        .note("Other")
        .done()
        .unwrap();
    travel
        .income(500.)
        .payee("Some")
        .note("Other")
        .done()
        .unwrap();
    assert_eq!(blib.actual_total_balance(), INITIAL - 1000. + 500.);
}

#[test]
fn finding_category_by_name_in_budget_account() {
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    blib.current_budget().unwrap().id();
    {
        let category = blib.find_category("Bills").unwrap();
        let mut bills = CategoryModel::new(Rc::clone(&blib.conn), category);
        assert_eq!(bills.allocated(), BILLS);
        assert_eq!(bills.balance(), BILLS);
    }
}

#[test]
fn creating_category_and_do_transactions() {
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let _home = {
        let home = { blib.create_category("Home", 3000., true).unwrap() };
        assert_eq!(home.allocated(), 3000.0);
        assert_eq!(blib.category_balance("Home").unwrap(), 3000.0);
        home
    };
    let mut home_ops = blib.new_transaction_to_category("Home").unwrap();
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
    assert_eq!(blib.category_balance("Home").unwrap(), 1000.0);
    let mut cm = blib.get_category_model("Home");

    assert_eq!(cm.allocated(), 3000.);
    assert_eq!(blib.total_expense(Some("Home")).unwrap(), -2000.);
}

#[test]
pub fn spending_from_category() {
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    let bills_available = blib.category_balance("Bills").unwrap();
    assert_eq!(bills_available, BILLS);
    assert_eq!(blib.actual_total_balance(), BILLS + TRAVEL + UNUSED);
    blib.new_transaction_to_category("Bills").unwrap()
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
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    blib.new_transaction_to_category("Bills").unwrap()
        .expense(14000.)
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
    let db = memory_db();
    let mut blib = Budgeting::new(db);
    new_budget_using_budgeting(&mut blib);
    blib.new_transaction_to_category("Bills").unwrap()
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
    let db = memory_db();
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
        .new_transaction_to_category("Bills").unwrap()
        .expense(600.)
        .payee("someone")
        .note("test")
        .done()
        .expect("Error occurred");
    budgeting
        .new_transaction_to_category(DEFAULT_CATEGORY).unwrap()
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
