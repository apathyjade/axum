use chrono::Utc;
use uuid::Uuid;

use diesel::prelude::*;

use crate::{
    model::{
        pager::Pager, tenant::{NewTenant, Status, Tenant}
    },
    schema,
    utils::db::DbConn,
};

pub fn get_list(conn: &mut DbConn, pager: Option<Pager>) -> Result<Vec<Tenant>, diesel::result::Error> {
    let table = schema::tenants::table;
    let pager = pager.unwrap_or(Pager {
        page: Some(1),
        page_size: Some(10),
    });
    table.select(schema::tenants::all_columns)
        .offset(pager.get_offset())
        .limit(pager.get_limit())
        .load::<Tenant>(conn)
}

pub fn create_tenant(conn: &mut DbConn, new_tenant: NewTenant) -> Result<Tenant, diesel::result::Error> {
    let new_tenant_instance = Tenant {
        id: Uuid::new_v4(),
        company_name: new_tenant.company_name,
        short_name: new_tenant.short_name,
        unified_social_credit_code: new_tenant.unified_social_credit_code,
        industry: new_tenant.industry,
        website: new_tenant.website,
        contact_name: new_tenant.contact_name,
        contact_phone: new_tenant.contact_phone,
        contact_email: new_tenant.contact_email,
        province: new_tenant.province,
        city: new_tenant.city,
        district: new_tenant.district,
        address_detail: new_tenant.address_detail,
        status: Status::PENDING as i32,
        approved_at: None,
        rejected_reason: None,
        business_license_url: new_tenant.business_license_url,
        business_license_verified: None,
        updated_at: Utc::now(),
        created_at: Utc::now(),
        deleted_at: None,
    };
    diesel::insert_into(schema::tenants::table)
        .values(&new_tenant_instance)
        .get_result::<Tenant>(conn)
}

pub async fn patch_status(
    conn: &mut DbConn,
    tenant_id: Uuid,
    new_status: Status,
) -> Result<usize, diesel::result::Error> {
    let target = schema::tenants::table.filter(schema::tenants::id.eq(tenant_id));
    diesel::update(target)
        .set(schema::tenants::status.eq(new_status as i32))
        .execute(conn)
}

pub fn is_tenant_exists(
    conn: &mut PgConnection,
    company_name: String,
) -> Result<bool, diesel::result::Error> {
    schema::tenants::table
        .filter(schema::tenants::company_name.eq(company_name))
        .select(schema::tenants::id)
        .first::<Uuid>(conn)
        .map(|_| true)
        .or_else(|_| Ok(false))
}
