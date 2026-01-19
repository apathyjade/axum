// @generated automatically by Diesel CLI.

diesel::table! {
    tenant_audit_logs (id) {
        id -> Int8,
        tenant_id -> Uuid,
        #[max_length = 20]
        action -> Varchar,
        operator_id -> Nullable<Uuid>,
        reason -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tenants (id) {
        id -> Uuid,
        company_name -> Citext,
        short_name -> Nullable<Text>,
        unified_social_credit_code -> Nullable<Text>,
        #[max_length = 100]
        industry -> Nullable<Varchar>,
        website -> Nullable<Text>,
        contact_name -> Text,
        contact_phone -> Text,
        contact_email -> Citext,
        province -> Nullable<Text>,
        city -> Nullable<Text>,
        district -> Nullable<Text>,
        address_detail -> Nullable<Text>,
        status -> Int4,
        approved_at -> Nullable<Timestamptz>,
        rejected_reason -> Nullable<Text>,
        business_license_url -> Nullable<Text>,
        business_license_verified -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
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

diesel::joinable!(tenant_audit_logs -> tenants (tenant_id));

diesel::allow_tables_to_appear_in_same_query!(tenant_audit_logs, tenants, users,);
