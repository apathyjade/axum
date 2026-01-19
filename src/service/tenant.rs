use chrono::Utc;
use uuid::Uuid;

use diesel::prelude::*;

use crate::{
    model::{
        pager::Pager,
        tenant::{NewTenant, Status, Tenant},
    },
    schema,
    utils::db::DbConn,
};

pub fn get_list(conn: &mut DbConn, pager: Option<Pager>) -> Result<Vec<Tenant>, String> {
    let result = tokio::task::block_in_place(|| {
        let table = schema::tenants::table;
        let pager = pager.unwrap_or(Pager {
            page: Some(1),
            page_size: Some(10),
        });
        table
            .select(schema::tenants::all_columns)
            .offset(pager.get_offset())
            .limit(pager.get_limit())
            .load::<Tenant>(conn)
    });
    match result {
        Ok(tenants) => Ok(tenants),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn create_tenant(conn: &mut DbConn, new_tenant: NewTenant) -> Result<Tenant, String> {
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
    let result = tokio::task::block_in_place(|| {
        diesel::insert_into(schema::tenants::table)
            .values(&new_tenant_instance)
            .get_result::<Tenant>(conn)
    });
    match result {
        Ok(tenant) => Ok(tenant),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn patch_status(
    conn: &mut DbConn,
    tenant_id: Uuid,
    new_status: Status,
) -> Result<(), String> {
    let target = schema::tenants::table.filter(schema::tenants::id.eq(tenant_id));
    let result = tokio::task::block_in_place(|| {
        diesel::update(target)
            .set(schema::tenants::status.eq(new_status as i32))
            .execute(conn)
    });
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
