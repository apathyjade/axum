
-- 启用扩展（如未启用）
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "citext"; -- 大小写不敏感的 email/company_name

-- 企业租户主表
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- 基础信息
    company_name CITEXT NOT NULL UNIQUE,        -- 企业全称（唯一）
    short_name TEXT,                            -- 企业简称（可选）
    unified_social_credit_code TEXT UNIQUE,     -- 统一社会信用代码（中国）或 Tax ID（国际）
    industry VARCHAR(100),                      -- 所属行业
    website TEXT CHECK (website ~ '^https://'), -- 官网（可选，需以 http/https 开头）
    
    -- 联系信息
    contact_name TEXT NOT NULL,                 -- 联系人姓名
    contact_phone TEXT NOT NULL,                -- 联系电话
    contact_email CITEXT NOT NULL UNIQUE,       -- 联系邮箱（唯一）
    
    -- 地址信息
    province TEXT,
    city TEXT,
    district TEXT,
    address_detail TEXT,                        -- 详细地址
    
    -- 审核状态
    status INTEGER NOT NULL DEFAULT 0 CHECK (status IN (-1, 0, 1)),
    approved_at TIMESTAMPTZ,                    -- 审核通过时间
    rejected_reason TEXT,                       -- 拒绝原因（仅当 status = -1 时有效）
    
    -- 营业执照（可选但推荐）
    business_license_url TEXT,                  -- 营业执照图片 URL（OSS/S3）
    business_license_verified BOOLEAN DEFAULT false, -- 是否人工验证营业执照
    
    -- 系统字段
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ                      -- 软删除（可选）
);

-- 索引优化
CREATE INDEX idx_tenants_status ON tenants(status);
CREATE INDEX idx_tenants_contact_email ON tenants(contact_email);
CREATE INDEX idx_tenants_credit_code ON tenants(unified_social_credit_code);


CREATE TABLE tenant_audit_logs (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    action VARCHAR(20) NOT NULL CHECK (action IN ('submit', 'approve', 'reject', 'update')),
    operator_id UUID,                           -- 操作人（后台管理员 ID，可为空表示系统自动）
    reason TEXT,                                -- 操作备注（如拒绝原因）
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_tenant ON tenant_audit_logs(tenant_id);
CREATE INDEX idx_audit_logs_created ON tenant_audit_logs(created_at);