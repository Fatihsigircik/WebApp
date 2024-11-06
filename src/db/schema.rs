// @generated automatically by Diesel CLI.

diesel::table! {
    menus (id) {
        id -> Int4,
        parent_menu_id -> Nullable<Int4>,
        #[max_length = 500]
        title -> Varchar,
        link_type -> Nullable<Int2>,
        #[max_length = 1000]
        link -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        #[max_length = 500]
        title -> Varchar,
        body -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 15]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    menus,
    pages,
    users,
);
