// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Nullable<Integer>,
        text -> Text,
        completed -> Bool,
    }
}
