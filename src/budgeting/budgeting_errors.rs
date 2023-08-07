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

impl From<BudgetingErrors> for String {
    fn from(value: BudgetingErrors) -> Self {
        match value {
            BudgetingErrors::BudgetAccountNotFound => "E0001".to_string(),
            BudgetingErrors::CategoryNotFound => "E0002".to_string(),
            BudgetingErrors::FailedToCreateBudget(_) => "E0003".to_string(),
            BudgetingErrors::BudgetAccountNotSelected => "E0004".to_string(),
            BudgetingErrors::CategoryAlreadyExists => "E0005".to_string(),
            BudgetingErrors::FundTransferError => "E0006".to_string(),
            BudgetingErrors::CategoryUpdateFailed => "E0007".to_string(),
            BudgetingErrors::CategoryDeleteFailed => "E0008".to_string(),
            BudgetingErrors::FailedToCreateCategory(_) => "E0009".to_string(),
            BudgetingErrors::OverFundingError => "E0010".to_string(),
            BudgetingErrors::AlreadyFunded => "E0011".to_string(),
            BudgetingErrors::MissingTransactionFields => "E0012".to_string(),
            BudgetingErrors::UnspecifiedDatabaseError(_) => "E0013".to_string(),
            BudgetingErrors::TransactionNotFound => "E0014".to_string(),
            BudgetingErrors::TransactionUpdateFailed => "E0015".to_string(),
            BudgetingErrors::BudgetAccountUpdateFailed => "E0016".to_string(),
            BudgetingErrors::OnlyDefaultCategoryCanHaveIncome => "E0017".to_string(),
            BudgetingErrors::ReturnWithHelpMessage(_) => "E0018".to_string(),
        }
    }
}
