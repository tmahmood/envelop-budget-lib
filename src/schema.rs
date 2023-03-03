// @generated automatically by Diesel CLI.

diesel::table! {
    budget_accounts (id) {
        id -> Integer,
        filed_as -> Text,
        date_created -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Text,
        allocated -> Double,
    }
}

diesel::table! {
    transaction_types (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        note -> Text,
        payee -> Text,
        date_created -> Timestamp,
        amount -> Double,
        category_id -> Integer,
        income -> Bool,
        transaction_type_id -> Integer,
        transfer_category_id -> Nullable<Integer>,
        budget_account_id -> Integer,
    }
}

diesel::joinable!(transactions -> budget_accounts (budget_account_id));
diesel::joinable!(transactions -> categories (category_id));
diesel::joinable!(transactions -> transaction_types (transaction_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    budget_accounts,
    categories,
    transaction_types,
    transactions,
);
