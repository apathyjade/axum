use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{model::pager::Pager, schema};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Copy)]
#[repr(i32)]
pub enum Status {
    PENDING = 0,
    APPROVED = 1,
    REJECTED = -1,
}

#[derive(
    Debug,
    Deserialize,
    Serialize,
    ToSchema,
    Queryable,
    Identifiable,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = schema::tenants)]
#[diesel(check_for_backend(Pg))]
pub struct Tenant {
    #[schema(value_type = String, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "中华科技有限公司")]
    pub company_name: String,
    #[schema(example = "中科")]
    pub short_name: Option<String>,
    #[schema(example = "91310000786784005J")]
    pub unified_social_credit_code: Option<String>,
    #[schema(example = "IT")]
    pub industry: Option<String>,
    #[schema(example = "https://www.china.com")]
    pub website: Option<String>,
    #[schema(example = "张三")]
    pub contact_name: String,
    #[schema(example = "13800138000")]
    pub contact_phone: String,
    #[schema(example = "1234567890@email.com")]
    pub contact_email: String,
    #[schema(example = "北京市")]
    pub province: Option<String>,
    #[schema(example = "北京市")]
    pub city: Option<String>,
    #[schema(example = "东城区")]
    pub district: Option<String>,
    #[schema(example = "西二旗")]
    pub address_detail: Option<String>,
    #[schema(example = "0")]
    pub status: i32,
    #[schema(example = "2023-07-01 12:00:00 UTC")]
    pub approved_at: Option<DateTime<Utc>>,
    #[schema(example = "拒绝理由")]
    pub rejected_reason: Option<String>,
    #[schema(example = "https://www.china.com/xxxx.png")]
    pub business_license_url: Option<String>,
    #[schema(example = "true")]
    pub business_license_verified: Option<bool>,
    #[schema(example = "2023-07-01 12:00:00 UTC")]
    pub updated_at: DateTime<Utc>,
    #[schema(example = "2023-07-01 12:00:00 UTC")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2023-07-01 12:00:00 UTC")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NewTenant {
    pub company_name: String,
    pub short_name: Option<String>,
    pub unified_social_credit_code: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address_detail: Option<String>,
    pub business_license_url: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ListQueryParams {
    pub pager: Option<Pager>,
    pub search: Option<String>,
}
