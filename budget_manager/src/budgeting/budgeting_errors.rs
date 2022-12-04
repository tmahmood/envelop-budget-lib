#[derive(thiserror::Error, Debug, Clone)]
pub enum BudgetingErrors {
    #[error("Error transferring fund from one category to other")]
    FundTransferError,
    #[error("Category does not exist")]
    CategoryNotFound,
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Category update failed")]
    CategoryUpdateFailed,
    #[error("Budget Account not found")]
    BudgetAccountNotFound(String),
    #[error("Failed to create budget")]
    FailedToCreateBudget(String),
    #[error("Failed to create category")]
    FailedToCreateCategory(String),
}
