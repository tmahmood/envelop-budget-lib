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
    categories (id) {
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
        category_id -> Integer,
    }
}

diesel::joinable!(categories -> budget_accounts (budget_account_id));
diesel::joinable!(transactions -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    budget_accounts,
    categories,
    transactions,
);
