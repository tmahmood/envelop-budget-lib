use crate::budgeting::budget_account::{
    BudgetAccount, BudgetAccountBuilder, BudgetAccountModel, NewBudgetAccount,
};
use crate::budgeting::category::{Category, CategoryBuilder, CategoryModel};
use crate::budgeting::transaction::{Transaction, TransactionBuilder, TransactionForm, TransactionModel, TransactionType};
use crate::{establish_connection, DEFAULT_CATEGORY};
use budgeting_errors::BudgetingErrors;
use budgeting_errors::BudgetingErrors::{
    BudgetAccountNotFound, CategoryNotFound, FailedToCreateBudget,
};
use diesel::connection::BoxableConnection;
use diesel::dsl::sum;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::env;
use std::ops::DerefMut;
use dotenvy::dotenv;

pub mod budget_account;
pub mod budgeting_errors;
pub mod category;
pub mod transaction;

mod builder {
    pub trait Builder {
        fn build(&self) -> Self;
    }
}

pub struct Budgeting {
    conn: RefCell<SqliteConnection>,
    budget: Option<BudgetAccount>,
}

impl Budgeting {
    // get all budget accounts
    pub fn budget_accounts(&mut self) -> Result<Vec<BudgetAccount>, BudgetingErrors> {
        BudgetAccountModel::find_all(self.conn_mut())
    }

    pub fn new(conn: SqliteConnection) -> Self {
        let mut r = RefCell::new(conn);
        Budgeting {
            conn: r,
            budget: None,
        }
    }

    /// creates a new budget and set as current budget
    pub fn new_budget(
        &mut self,
        filed_as: &str,
        amount: f64,
    ) -> Result<BudgetAccount, BudgetingErrors> {
        let budget_account = self.find_budget(filed_as);
        if budget_account.is_ok() {
            return Err(FailedToCreateBudget(filed_as.to_string()));
        }
        let b = BudgetAccountBuilder::new(self.conn_mut(), filed_as).build();
        self.budget = Some(b.clone());
        self.new_transaction_to_category(DEFAULT_CATEGORY)
            .income(amount)
            .payee("Self")
            .note("Initial Balance")
            .done()?;
        Ok(b)
    }

    /// switch to given budget account
    pub(crate) fn switch_budget_account(
        &mut self,
        budget_account: &str,
    ) -> Result<(), BudgetingErrors> {
        match BudgetAccountModel::load_by_name(self.conn_mut(), budget_account) {
            Ok(e) => {
                self.budget = Some(e);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Starts a new transaction belonging to given category.
    /// it's not completed until `done` method is called
    pub fn new_transaction_to_category(&mut self, category: &str) -> TransactionBuilder {
        let b = self.current_budget();
        let mut _category = self.find_category(category).unwrap();
        TransactionBuilder::new(self.conn_mut(), b.id(), _category.id())
    }

    /// Transfers fund from one category to another
    pub fn transfer_fund(
        &mut self,
        src: &str,
        dest: &str,
        amount: f64,
    ) -> Result<(), BudgetingErrors> {
        let k = self.new_transaction_to_category(src)
            .transfer_from(amount)
            .payee(dest)
            .note(&format!("Funded"))
            .done()?;
        self.new_transaction_to_category(dest)
            .transfer_to(amount)
            .transfer_category_id(k.category_id())
            .payee(src)
            .note(&format!("Received"))
            .done()?;
        Ok(())
    }

    pub fn update_budget_account(
        &mut self,
        budget_account_id: i32,
        filed_as: Option<String>,
    ) -> Result<usize, BudgetingErrors> {
        BudgetAccountModel::update(self.conn_mut(), budget_account_id, filed_as)
    }

    pub fn update_category(
        &mut self,
        category_id: i32,
        name: Option<String>,
        amount: Option<f64>,
    ) -> Result<usize, BudgetingErrors> {
        CategoryModel::update(self.conn_mut(), category_id, name, amount)
    }

    pub fn delete_category(&mut self, category_id: i32) -> Result<(usize), BudgetingErrors> {
        CategoryModel::delete(self.conn_mut(), category_id)
    }

    pub fn update_transaction(
        &mut self,
        transaction_id: i32,
        change_set: TransactionForm,
    ) -> Result<usize, BudgetingErrors> {
        TransactionModel::update(self.conn_mut(), transaction_id, change_set)
    }
    /// calculates the amount required to fully fund the category from unallocated balance.
    pub fn calculate_amount_to_fund(
        &mut self,
        src_category: &str,
        dest_category: &str,
        as_much_possible: bool,
    ) -> Result<f64, BudgetingErrors> {
        let bid = self.current_budget().id();
        let mut cm = self.get_category_model(dest_category);
        let balance = cm.balance(bid);
        let allocated = cm.allocated();
        if balance >= allocated {
            return Err(BudgetingErrors::AlreadyFunded);
        }
        let src_balance = self.category_balance(src_category)?;
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
        let src_balance = self.category_balance(src_category)?;
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
    ) -> Result<Category, BudgetingErrors> {
        let c = self.category_builder(category).allocated(allocate).done()?;
        if transfer {
            self.transfer_fund(DEFAULT_CATEGORY, category, allocate)?;
        }
        Ok(c)
    }

    pub fn category_model(&mut self, category: Category) -> CategoryModel {
        CategoryModel::new(self.conn_mut(), category)
    }

    pub fn transaction_model(&mut self, transaction: Transaction) -> TransactionModel {
        TransactionModel::new(self.conn_mut(), transaction)
    }

    pub fn budget_account_model(&mut self, budget_account: BudgetAccount) -> BudgetAccountModel {
        BudgetAccountModel::new(self.conn_mut(), budget_account)
    }

    pub fn get_category_model(&mut self, category_name: &str) -> CategoryModel {
        let c = self.find_category(category_name).unwrap();
        self.category_model(c)
    }

    pub fn get_category_model_by_id(
        &mut self,
        category_id: i32,
    ) -> Result<CategoryModel, BudgetingErrors> {
        CategoryModel::load(self.conn_mut(), category_id)
    }

    pub fn get_transaction_model_by_id(
        &mut self,
        transaction_id: i32,
    ) -> Result<TransactionModel, BudgetingErrors> {
        TransactionModel::load(self.conn_mut(), transaction_id)
    }

    pub fn find_category(&self, category_name: &str) -> Result<Category, BudgetingErrors> {
        imp_db!(categories);
        let mut conn = self.conn.borrow_mut();
        let result: QueryResult<Category> = categories.filter(name.eq(category_name)).first(conn.deref_mut());
        result.map_err(|e| CategoryNotFound)
    }

    pub fn category_balance(&self, category: &str) -> Result<f64, BudgetingErrors> {
        let bid = self.current_budget().id();
        let mut c = self.conn.borrow_mut();
        let conn = c.deref_mut();
        CategoryModel::c_balance(conn, bid, category)
    }

    pub fn find_budget(&mut self, _filed_as: &str) -> QueryResult<BudgetAccount> {
        imp_db!(budget_accounts);
        let budget_account: QueryResult<BudgetAccount> = budget_accounts
            .filter(filed_as.eq(_filed_as))
            .first(self.conn_mut());
        budget_account
    }

    pub fn current_budget(&self) -> BudgetAccount {
        let b = self.budget.as_ref().unwrap();
        b.clone()
    }

    pub fn set_current_budget(&mut self, filed_as: &str) -> Result<BudgetAccount, BudgetingErrors> {
        let b = self.find_budget(filed_as);
        if b.is_err() {
            return Err(BudgetAccountNotFound);
        }
        let b = b.unwrap();
        self.budget = Some(b.clone());
        Ok(b)
    }

    pub fn category_builder(&mut self, category_name: &str) -> CategoryBuilder {
        CategoryBuilder::new(self.conn_mut(), category_name)
    }

    /// returns all the category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn all_categories(&mut self) -> Vec<Category> {
        imp_db!(categories);
        categories.load::<Category>(self.conn_mut()).unwrap()
    }

    /// returns all the category except the unallocated category. To get the unallocated category
    /// `uncategorized` method can be used
    pub fn categories(&mut self) -> Vec<Category> {
        imp_db!(categories);
        categories
            .filter(name.ne(DEFAULT_CATEGORY))
            .load::<Category>(self.conn_mut())
            .unwrap()
    }

    pub fn total_allocated(&mut self) -> f64 {
        imp_db!(categories);
        let result_option: QueryResult<Option<f64>> = categories
            .select(sum(allocated))
            .filter(name.ne(DEFAULT_CATEGORY))
            .first::<Option<f64>>(self.conn_mut());
        return_sum!(result_option)
    }

    pub fn default_category(&mut self) -> Category {
        imp_db!(categories);
        categories
            .filter(name.eq(DEFAULT_CATEGORY))
            .first::<Category>(self.conn_mut())
            .unwrap()
    }

    /// actual total balance? it is the real money available
    /// sum of all the category balances + unallocated balance
    /// unallocated balance would be balance unused + all the
    /// transactions in unallocated category
    pub fn actual_total_balance(&mut self) -> f64 {
        let bid = self.current_budget().id();
        TransactionModel::total(self.conn_mut(), None, None, Some(bid))
    }

    /// returns the total unallocated balance
    pub fn uncategorized_balance(&mut self) -> f64 {
        let c = self.default_category();
        let bid = self.current_budget().id();
        imp_db!(transactions);
        let result_option = transactions
            .select(sum(amount))
            .filter(category_id.eq(c.id()))
            .filter(budget_account_id.eq(bid))
            .first::<Option<f64>>(self.conn_mut());
        return_sum!(result_option)
    }

    pub fn total_income(&mut self, category: Option<&str>) -> Result<f64, BudgetingErrors> {
        self.total_of(TransactionType::Income, category)
    }

    pub fn total_expense(&mut self, category: Option<&str>) -> Result<f64, BudgetingErrors> {
        self.total_of(TransactionType::Expense, category)
    }

    fn total_of(
        &mut self,
        filter_opt: TransactionType,
        category: Option<&str>,
    ) -> Result<f64, BudgetingErrors> {
        let cid = if let Some(c) = category {
            Some(self.find_category(c)?.id())
        } else {
            None
        };
        let bid = self.current_budget().id();
        Ok(TransactionModel::total(
            self.conn_mut(),
            Some(filter_opt),
            cid,
            Some(bid),
        ))
    }

    pub fn transactions(&mut self, _category_id: Option<i32>) -> Vec<Transaction> {
        let bid = Some(self.current_budget().id());
        TransactionModel::find_all(self.conn_mut(), _category_id, bid)
    }


    pub(crate) fn conn_mut(&mut self) -> &mut SqliteConnection {
        self.conn.get_mut()
    }
}

impl Default for Budgeting {
    fn default() -> Self {
        let conn = establish_connection();
        Budgeting::new(conn)
    }
}

#[cfg(test)]
pub mod tests;