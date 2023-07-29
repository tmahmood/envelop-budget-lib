use diesel::result::DatabaseErrorKind;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BudgetingErrors {
    #[error("Error transferring fund from one category to other")]
    FundTransferError,
    #[error("Category does not exist")]
    CategoryNotFound,
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Failed to update category")]
    CategoryUpdateFailed,
    #[error("Failed to delete category")]
    CategoryDeleteFailed,
    #[error("Budget Account not found")]
    BudgetAccountNotFound,
    #[error("Failed to create budget: {0}")]
    FailedToCreateBudget(String),
    #[error("Failed to create category: {0}")]
    FailedToCreateCategory(String),
    #[error("Trying to fund more than what is actually available")]
    OverFundingError,
    #[error("Already Funded")]
    AlreadyFunded,
    #[error("Not all transaction fields are provided")]
    MissingTransactionFields,
    #[error("Unspecified Database Error")]
    UnspecifiedDatabaseError(diesel::result::Error),
    #[error("Transaction not found")]
    TransactionNotFound,
    #[error("Failed to update transaction")]
    TransactionUpdateFailed,
    #[error("Failed to update budget account")]
    BudgetAccountUpdateFailed,
    #[error("Only default category can have income transaction type")]
    OnlyDefaultCategoryCanHaveIncome,
    #[error("You need to select a budget account for this action")]
    BudgetAccountNotSelected,
    #[error("Help: {0}")]
    ReturnWithHelpMessage(String),
}
