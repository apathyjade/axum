// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int8,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 128]
        email -> Nullable<Varchar>,
        #[max_length = 16]
        phone -> Nullable<Varchar>,
        #[max_length = 64]
        real_name -> Nullable<Varchar>,
        status -> Int4,
        created_time -> Timestamp,
        updated_time -> Timestamp,
    }
}
