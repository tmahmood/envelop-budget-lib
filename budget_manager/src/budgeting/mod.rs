use crate::budgeting::budget_account::{BudgetAccount, BudgetAccountBuilder, NewBudgetAccount};
use crate::budgeting::category::{Category, CategoryBuilder, CategoryModel};
use crate::budgeting::transaction::{
    Transaction, TransactionBuilder, TransactionModel, TransactionType,
};
use crate::{establish_connection, DEFAULT_CATEGORY};
use budgeting_errors::BudgetingErrors;
use budgeting_errors::BudgetingErrors::{
    BudgetAccountNotFound, CategoryNotFound, FailedToCreateBudget,
};
use diesel::connection::BoxableConnection;
use diesel::dsl::sum;
use diesel::{QueryResult, SqliteConnection};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub mod budget_account;
pub mod budgeting_errors;
pub mod category;
pub mod storage;
pub mod transaction;

mod builder {
    pub trait Builder {
        fn build(&self) -> Self;
    }
}

pub mod prelude {
    use super::budget_account::*;
    use super::category::*;
    use super::transaction::*;
}

pub struct Budgeting {
    conn: Rc<SqliteConnection>,
    budget: Option<BudgetAccount>,
}

impl Default for Budgeting {
    fn default() -> Self {
        Budgeting::new()
    }
}

impl Budgeting {
    /// Starts a new transaction belonging to given category.
    /// it's not completed until `done` method is called
    pub fn new_transaction_to_category<'b>(&'b mut self, category: &'b str) -> TransactionBuilder {
        let b = self.current_budget();
        let mut _category = self.find_category(category).unwrap();
        TransactionBuilder::new(self.conn(), _category.id())
    }

    /// Transfers fund from one category to another
    pub fn transfer_fund(
        &mut self,
        src: &str,
        dest: &str,
        amount: f64,
    ) -> Result<(), BudgetingErrors> {
        self.new_transaction_to_category(src)
            .transfer_from(amount)
            .payee(dest)
            .note(&format!("Funded {}", dest))
            .done()?;
        self.new_transaction_to_category(dest)
            .transfer_to(amount)
            .payee(src)
            .note(&format!("Received {}", src))
            .done()?;
        Ok(())
    }

    pub fn update_category(
        &mut self,
        category_id: i32,
        name: Option<String>,
        amount: Option<f64>,
    ) -> Result<i32, BudgetingErrors> {
        CategoryModel::update(self.conn(), category_id, name, amount)
    }

    pub fn delete_category(&mut self, category_id: i32) -> Result<(usize), BudgetingErrors> {
        CategoryModel::delete(self.conn(), category_id)
    }

    /// calculates the amount required to fully fund the category from unallocated balance.
    pub fn calculate_amount_to_fund(
        &mut self,
        src_category: &str,
        dest_category: &str,
        as_much_possible: bool,
    ) -> Result<f64, BudgetingErrors> {
        let mut cm = self.get_category_model(dest_category);
        let balance = cm.balance();
        let allocated = cm.allocated();
        if balance >= allocated {
            return Err(BudgetingErrors::AlreadyFunded);
        }
        let src_balance = self.category_balance(src_category);
        let to_fund = if balance > 0. {
            allocated - balance
        } else {
            balance.abs() + allocated
        };
        let diff_src_to_fund = src_balance - to_fund;
        if diff_src_to_fund < 0. {
            if !as_much_possible {
                return Err(BudgetingErrors::OverFundingError);
            }
            return Ok(to_fund + diff_src_to_fund);
        }
        Ok(to_fund)
    }

    pub fn check_if_funding_possible(
        &mut self,
        src_category: &str,
        fund: f64,
        as_much_possible: bool,
    ) -> Result<f64, BudgetingErrors> {
        let src_balance = self.category_balance(src_category);
        let diff_src_to_fund = src_balance - fund;
        if diff_src_to_fund <= 0. {
            if as_much_possible && fund <= src_balance && src_balance != 0. {
                return Ok(fund);
            }
            return Err(BudgetingErrors::OverFundingError);
        }
        Ok(fund)
    }

    pub fn fund_all_from_unallocated(
        &mut self,
        category: &str,
        as_much_possible: bool,
    ) -> Result<(), BudgetingErrors> {
        let to_fund =
            self.calculate_amount_to_fund(DEFAULT_CATEGORY, category, as_much_possible)?;
        self.transfer_fund(DEFAULT_CATEGORY, category, to_fund)
    }

    // creates a new category and allocates the budget
    pub fn create_category(
        &mut self,
        category: &str,
        allocate: f64,
        transfer: bool,
    ) -> Result<i32, BudgetingErrors> {
        let c = self.category_builder(category).allocated(allocate).done()?;
        if transfer {
            self.transfer_fund(DEFAULT_CATEGORY, category, allocate)?;
        }
        Ok(c.id())
    }

    pub fn category_model(&mut self, category: Category) -> CategoryModel {
        CategoryModel::new(self.conn(), category)
    }

    pub fn transaction_model(&mut self, transaction: Transaction) -> TransactionModel {
        TransactionModel::new(self.conn(), transaction)
    }

    pub fn get_category_model(&mut self, category_name: &str) -> CategoryModel {
        let c = self.find_category(category_name).unwrap();
        self.category_model(c)
    }

    pub fn get_category_model_by_id(
        &mut self,
        category_id: i32,
    ) -> Result<CategoryModel, BudgetingErrors> {
        CategoryModel::load(self.conn(), category_id)
    }

    pub fn get_transaction_model_by_id(
        &mut self,
        transaction_id: i32,
    ) -> Result<TransactionModel, BudgetingErrors> {
        TransactionModel::load(self.conn(), transaction_id)
    }

    pub fn find_category(&mut self, category_name: &str) -> Result<Category, BudgetingErrors> {
        imp_db!(categories);
        imp_db!(budget_accounts);
        let budget = { self.current_budget() };
        let conn = self.conn();
        let result: QueryResult<Category> = Category::belonging_to(&budget)
            .filter(name.eq(category_name))
            .first(conn);
        result.map_err(|e| CategoryNotFound)
    }

    pub fn category_balance(&mut self, category: &str) -> f64 {
        imp_db!(categories);
        imp_db!(budget_accounts);
        self.get_category_model(category).balance()
    }

    pub fn find_budget(&mut self, filed_as: &str) -> QueryResult<BudgetAccount> {
        imp_db!(budget_accounts);
        let budget_account: QueryResult<BudgetAccount> = budget_accounts
            .filter(filed_as.eq(filed_as))
            .first(self.conn());
        budget_account
    }

    pub fn current_budget(&mut self) -> BudgetAccount {
        let b = self.budget.as_ref().unwrap();
        b.clone()
    }

    pub fn set_current_budget(&mut self, filed_as: &str) -> Result<BudgetAccount, BudgetingErrors> {
        let b = self.find_budget(filed_as);
        if b.is_err() {
            return Err(BudgetAccountNotFound(filed_as.to_string()));
        }
        let b = b.unwrap();
        self.budget = Some(b.clone());
        Ok(b)
    }

    pub fn category_builder(&mut self, category_name: &str) -> CategoryBuilder {
        let id = self.current_budget().id();
        CategoryBuilder::new(self.conn(), id, category_name)
    }

    pub fn new_budget(
        &mut self,
        filed_as: &str,
        amount: f64,
    ) -> Result<BudgetAccount, BudgetingErrors> {
        let budget_account = self.find_budget(filed_as);
        if budget_account.is_ok() {
            return Err(FailedToCreateBudget(filed_as.to_string()));
        }
        let b = BudgetAccountBuilder::new(self.conn(), "main").build();
        self.budget = Some(b.clone());
        // create the default category
        self.category_builder(DEFAULT_CATEGORY)
            .allocated(0.)
            .done()?;
        self.new_transaction_to_category(DEFAULT_CATEGORY)
            .income(amount)
            .payee("Self")
            .note("Initial Balance")
            .done()?;
        Ok(b)
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn all_categories(&mut self) -> Vec<Category> {
        imp_db!(categories);
        Category::belonging_to(&self.current_budget())
            .load::<Category>(self.conn())
            .unwrap()
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn categories(&mut self) -> Vec<Category> {
        imp_db!(categories);
        Category::belonging_to(&self.current_budget())
            .filter(name.ne(DEFAULT_CATEGORY))
            .load::<Category>(self.conn())
            .unwrap()
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn total_category_balance(&mut self) -> f64 {
        imp_db!(transactions);
        self.categories()
            .iter()
            .map(|v| CategoryModel::new(self.conn(), v.clone()).balance())
            .sum::<f64>()
    }

    pub fn total_allocated(&mut self) -> f64 {
        imp_db!(categories);
        let result_option: QueryResult<Option<f64>> =
            Category::belonging_to(&self.current_budget())
                .select(sum(allocated))
                .filter(name.ne(DEFAULT_CATEGORY))
                .first::<Option<f64>>(self.conn());
        return_sum!(result_option)
    }

    pub fn default_category(&mut self) -> Category {
        imp_db!(categories);
        Category::belonging_to(&self.current_budget())
            .filter(name.eq(DEFAULT_CATEGORY))
            .first::<Category>(self.conn())
            .unwrap()
    }

    /// actual total balance? it is the real money available
    /// sum of all the category balances + unallocated balance
    /// unallocated balance would be balance unused + all the transactions in unallocated category
    pub fn actual_total_balance(&mut self) -> f64 {
        imp_db!(transactions);
        let result = transactions
            .select(sum(amount))
            .first::<Option<f64>>(self.conn());
        return_sum!(result)
    }

    /// returns the total unallocated balance
    pub fn uncategorized_balance(&mut self) -> f64 {
        let c = self.default_category();
        imp_db!(transactions);
        let result_option = Transaction::belonging_to(&c)
            .select(sum(amount))
            .first::<Option<f64>>(self.conn());
        return_sum!(result_option)
    }

    pub fn total_income(&mut self) -> f64 {
        self.total_of(TransactionType::Income)
    }

    pub fn total_expense(&mut self) -> f64 {
        self.total_of(TransactionType::Expense)
    }

    fn total_of(&mut self, filter_opt: TransactionType) -> f64 {
        imp_db!(transactions);
        let f = transaction_type_id.eq(i32::from(filter_opt));
        let result_option = transactions
            .select(sum(amount))
            .filter(f)
            .first::<Option<f64>>(self.conn());
        return_sum!(result_option)
    }

    pub fn transactions(&mut self) -> Vec<Transaction> {
        imp_db!(transactions);
        transactions.load::<Transaction>(self.conn()).unwrap()
    }

    pub(crate) fn conn(&mut self) -> &mut SqliteConnection {
        Rc::get_mut(&mut self.conn).unwrap()
    }

    pub fn new() -> Self {
        let r = Rc::new(establish_connection());
        Budgeting {
            conn: r,
            budget: None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::budgeting::budgeting_errors::BudgetingErrors::FundTransferError;
    use crate::budgeting::transaction::Transaction;
    use crate::test_helpers::{new_budget_using_budgeting, DbDropper};
    use crate::tests::{BILLS, DEFAULT_ID, INITIAL, TRAVEL, UNUSED};
    use diesel::prelude::*;

    #[test]
    fn initial_budget_account_details() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        blib.new_budget("main", 10000.).unwrap();
        assert_eq!(blib.uncategorized_balance(), INITIAL);
        assert_eq!(blib.actual_total_balance(), INITIAL);
    }

    #[test]
    fn allocating_money_behaviour() {
        let mut _dd = DbDropper::new();
        let mut blib = Budgeting::new();
        blib.new_budget("main", 10000.)
            .expect("Failed to create new budget");
        blib.create_category("Bills", BILLS, true)
            .expect("Failed to create category");
        blib.create_category("Travel", TRAVEL, true)
            .expect("Failed to create category");
        let v = blib.get_category_model(DEFAULT_CATEGORY).transactions();
        println!("{:#?}", v);
        assert_eq!(blib.uncategorized_balance(), 5000.);
    }

    #[test]
    fn total_allocation_check() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        assert_eq!(blib.total_allocated(), 5000.);
    }

    #[test]
    fn total_balance_is_actual_money() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
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
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let mut def = blib.new_transaction_to_category(DEFAULT_CATEGORY);
        def.expense(1000.).payee("Some").note("Other").done();
        def.income(5000.).payee("Some").note("Other").done();
        imp_db!(budget_accounts);
        assert_eq!(blib.category_balance(DEFAULT_CATEGORY), -1000. + 10000.)
    }

    #[test]
    pub fn total_balance_should_be_sum_of_all_categories_balance() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let mut travel = blib.new_transaction_to_category("Travel");
        let mut tb = travel.expense(1000.).payee("Some").note("Other").done();
        let mut tb = travel.income(500.).payee("Some").note("Other").done();
        assert_eq!(blib.actual_total_balance(), 5000. + BILLS + TRAVEL - 500.);
    }

    #[test]
    fn finding_category_by_name_in_budget_account() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        {
            let category = blib.find_category("Bills").unwrap();
            let mut bills = CategoryModel::new(blib.conn(), category);
            assert_eq!(bills.allocated(), BILLS);
            assert_eq!(bills.balance(), BILLS);
        }
        let mut bills = blib.new_transaction_to_category("Bills");
        let mut tb = bills.income(500.).payee("Some").note("Other").done();
        {
            let category = blib.find_category("Bills").unwrap();
            let mut bills = CategoryModel::new(blib.conn(), category);
            assert_eq!(bills.allocated(), BILLS);
            assert_eq!(bills.balance(), BILLS + 500.);
        }
    }

    #[test]
    fn creating_category_and_do_transactions() {
        let _dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let _home = {
            let home_id = blib.create_category("Home", 3000., true).unwrap();
            let mut cm = CategoryModel::load(blib.conn(), home_id).unwrap();
            let home = cm.category();
            assert_eq!(home.allocated(), 3000.0);
            assert_eq!(blib.category_balance("Home"), 3000.0);
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
        assert_eq!(blib.category_balance("Home"), 2000.0);
        let mut cm = blib.get_category_model("Home");
        assert_eq!(cm.allocated(), 3000.);
        assert_eq!(cm.expense(), -2000.);
        assert_eq!(cm.income(), 1000.);
    }

    #[test]
    pub fn spending_from_category() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        let bills_available = blib.category_balance("Bills");
        assert_eq!(bills_available, BILLS);
        assert_eq!(blib.actual_total_balance(), BILLS + TRAVEL + UNUSED);
        blib.new_transaction_to_category("Bills")
            .expense(BILLS)
            .payee("someone")
            .note("test")
            .done()
            .expect("Error occurred");
        let bills_available = blib.category_balance("Bills");
        assert_eq!(bills_available, 0.0);
        assert_eq!(blib.actual_total_balance(), TRAVEL + UNUSED);
    }

    #[test]
    pub fn funding_category_over_funded() {
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
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
        let mut dd = DbDropper::new();
        let mut blib = Budgeting::new();
        new_budget_using_budgeting(&mut blib);
        blib.new_transaction_to_category("Bills")
            .expense(600.)
            .payee("someone")
            .note("test")
            .done()
            .expect("Error occurred");
        assert_eq!(blib.fund_all_from_unallocated("Bills", false), Ok(()));
        assert_eq!(blib.category_balance("Bills"), BILLS);
    }

    #[test]
    pub fn funding_category_as_much_as_possible() {
        let mut dd = DbDropper::new();
        let mut budgeting = Budgeting::new();
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
}
