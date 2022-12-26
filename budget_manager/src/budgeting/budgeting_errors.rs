#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BudgetingErrors {
    #[error("Error transferring fund from one category to other")]
    FundTransferError,
    #[error("Category does not exist")]
    CategoryNotFound,
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Category update failed")]
    CategoryUpdateFailed,
    #[error("Budget Account not found: {0}")]
    BudgetAccountNotFound(String),
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
    UnspecifiedDatabaseError,
    #[error("Transaction not found")]
    TransactionNotFound,
}
