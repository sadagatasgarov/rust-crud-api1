// @generated automatically by Diesel CLI.

diesel::table! {
    baza (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
