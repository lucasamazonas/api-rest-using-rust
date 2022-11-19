// @generated automatically by Diesel CLI.

diesel::table! {
    usuarios (id) {
        id -> Int4,
        nome -> Varchar,
        email -> Varchar,
        senha -> Varchar,
    }
}
