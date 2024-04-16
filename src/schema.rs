// @generated automatically by Diesel CLI.

diesel::table! {
    baza (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
