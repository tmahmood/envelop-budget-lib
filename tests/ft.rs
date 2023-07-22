use diesel::{Connection, RunQueryDsl, SqliteConnection, TextExpressionMethods};
use rand::Rng;
use envelop_budget_lib::budgeting::Budgeting;
use envelop_budget_lib::run_migrations;
use crate::tests::{BILLS, TRAVEL};

pub fn generate_random_str(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn new_budget_using_budgeting(budgeting: &mut Budgeting) {
    budgeting.new_budget("wallet", 5000.).expect("Error creating new budget");
    budgeting.new_budget("main", 10000.).expect("Error creating new budget");
    budgeting
        .create_category("Bills", BILLS, true)
        .unwrap();
    budgeting
        .create_category("Travel", TRAVEL, true)
        .unwrap();
}

pub fn memory_db() -> SqliteConnection {
    let mut sqlite = SqliteConnection::establish(":memory:")
        .unwrap_or_else(|_| panic!("Failed to load memory db"));
    run_migrations(&mut sqlite).expect("Failed to run migrations");
    sqlite

}
#[cfg(test)]
mod tests {
    use envelop_budget_lib::budgeting::Budgeting;
    use envelop_budget_lib::DEFAULT_CATEGORY;
    use crate::{memory_db, new_budget_using_budgeting};

    // test all the possible things!
    pub const DEFAULT_ID: i32 = 1;
    pub const BILL_ID: i32 = 2;
    pub const TRAVEL_ID: i32 = 3;
    pub const BILLS: f64 = 2000.;
    pub const TRAVEL: f64 = 3000.;
    pub const UNUSED: f64 = 10000.;
    pub const INITIAL: f64 = 15000.;

    #[test]
    fn new_behavior_test() {
        let mut db = memory_db();
        let mut budgeting = Budgeting::new(db);

        new_budget_using_budgeting(&mut budgeting);

        let result = budgeting.category_balance("Bills");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BILLS);

        budgeting.switch_budget_account("wallet").unwrap();

        budgeting.new_transaction_to_category("Bills")
            .expense(400.)
            .payee("Someone")
            .note("Paid for something from wallet")
            .done().unwrap();

        budgeting.switch_budget_account("main").unwrap();

        budgeting.new_transaction_to_category("Bills")
            .expense(600.)
            .payee("Someone")
            .note("Paid for something from main")
            .done().unwrap();

        let result = budgeting.category_balance("Bills");
        assert_eq!(result.unwrap(), BILLS - 1000.);
    }

    #[test]
    fn transfer_should_not_be_counted_as_income_or_expense() {
        let mut db = memory_db();
        let mut blib = Budgeting::new(db);
        new_budget_using_budgeting(&mut blib);
        assert_eq!(blib.total_income(Some("Bills")).unwrap(), 0.);
        assert_eq!(blib.total_expense(Some(DEFAULT_CATEGORY)).unwrap(), 0.);
    }

    #[test]
    fn creating_budget_and_adding_transaction() {
        let mut db = memory_db();
        let mut blib = Budgeting::new(db);
        new_budget_using_budgeting(&mut blib);
        // initial + allocation to bills + allocation to travel
        assert_eq!(blib.transactions(None).len(), 5);
        assert_eq!(blib.actual_total_balance(), INITIAL);
        assert_eq!(blib.uncategorized_balance(), INITIAL - (BILLS + TRAVEL));
        // now let's do some transactions
        {
            let mut travel = blib.new_transaction_to_category("Travel");
            assert!(travel
                .expense(1000.)
                .payee("Some")
                .note("Other")
                .done()
                .is_ok());
            assert!(travel
                .expense(1300.)
                .payee("Uber")
                .note("Other")
                .done()
                .is_ok());
            // this will be automatically added to default category, not income
            assert!(travel
                .income(400.)
                .payee("Other")
                .note("Other")
                .done()
                .is_ok());
        }
        {
            let mut bills = blib.new_transaction_to_category("Bills");
            assert!(bills
                .expense(300.)
                .payee("Some")
                .note("Other")
                .done()
                .is_ok());
        }
        {
            let mut default = blib.new_transaction_to_category(DEFAULT_CATEGORY);
            assert!(default
                .expense(1000.)
                .payee("Other")
                .note("Other")
                .done()
                .is_ok());
            assert!(default
                .income(5000.)
                .payee("Other")
                .note("Other")
                .done()
                .is_ok());
        }
        // check total balance
        assert_eq!(blib.actual_total_balance(), INITIAL - 3600. + 5400.);
        assert_eq!(
            blib.category_balance("Travel").unwrap(),
            TRAVEL - 1000. - 1300.
        );
        assert_eq!(blib.category_balance("Bills").unwrap(), BILLS - 300.);
        assert_eq!(
            blib.category_balance(DEFAULT_CATEGORY).unwrap(),
            INITIAL + 5000. - 1000. - 3000. - 2000. + 400.
        );
    }


    #[test]
    fn transfer_fund_from_balance() {
        let mut db = memory_db();
        let mut blib = Budgeting::new(db);
        new_budget_using_budgeting(&mut blib);
        assert!(blib.transfer_fund("Bills", "Travel", BILLS).is_ok());
        //
        assert_eq!(blib.category_balance("Bills").unwrap(), 0.);
        assert_eq!(blib.category_balance("Travel").unwrap(), BILLS + TRAVEL);
        //
        assert_eq!(blib.total_expense(Some("Bills")).unwrap(), 0.);
        assert_eq!(blib.total_income(Some("Bills")).unwrap(), 0.);
    }
}
