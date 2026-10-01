CREATE SCHEMA IF NOT EXISTS tenant;

CREATE TABLE tenant.tenants
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name       TEXT        NOT NULL,
    phone      TEXT        NOT NULL,
    kra_pin    TEXT,
    tax_mode   TEXT        NOT NULL DEFAULT 'NONE'
        CHECK (tax_mode IN ('VAT', 'TOT', 'NONE')),
    currency   CHAR(3)     NOT NULL DEFAULT 'KES',
    timezone   TEXT        NOT NULL DEFAULT 'Africa/Nairobi',
    status     TEXT        NOT NULL DEFAULT 'ACTIVE'
        CHECK (status IN ('ACTIVE', 'SUSPENDED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX uq_tenants_kra_pin ON tenant.tenants (kra_pin) WHERE kra_pin IS NOT NULL;
CREATE INDEX phone_no_idx ON tenant.tenants (phone);

    
