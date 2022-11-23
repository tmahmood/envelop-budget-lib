// @generated automatically by Diesel CLI.

diesel::table! {
    budget_accounts (id) {
        id -> Integer,
        filed_as -> Text,
        date_created -> Timestamp,
        balance -> Double,
    }
}

diesel::table! {
    transaction_categories (id) {
        id -> Integer,
        name -> Text,
        allocated -> Double,
        budget_account_id -> Integer,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        note -> Text,
        payee -> Text,
        date_created -> Timestamp,
        income -> Bool,
        amount -> Double,
        transaction_category_id -> Integer,
    }
}

diesel::joinable!(transaction_categories -> budget_accounts (budget_account_id));
diesel::joinable!(transactions -> transaction_categories (transaction_category_id));

diesel::allow_tables_to_appear_in_same_query!(
    budget_accounts,
    transaction_categories,
    transactions,
);
