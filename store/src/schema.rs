// @generated automatically by Diesel CLI.

diesel::table! {
    bookmark (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        title -> Text,
        url -> Text,
        description -> Nullable<Text>,
        is_favorite -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(bookmark -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookmark,
    users,
);
