CREATE SCHEMA IF NOT EXISTS product;

-- KRA reference data (global)
CREATE TABLE product.kra_tax_types
(
    code CHAR(1) PRIMARY KEY,
    name TEXT          NOT NULL,
    rate NUMERIC(5, 2) NOT NULL
);

CREATE TABLE product.kra_item_classes
(
    code      CHAR(10) PRIMARY KEY,
    name      TEXT        NOT NULL,
    tax_type  CHAR(1) REFERENCES product.kra_tax_types (code),
    active    BOOLEAN     NOT NULL DEFAULT true,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Products
CREATE TABLE product.products
(
    id           UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    tenant_id    UUID        NOT NULL,
    name         TEXT        NOT NULL,
    sku          TEXT,
    plu_code     TEXT,                 -- scale labels (weighed items)
    measure_type TEXT        NOT NULL DEFAULT 'COUNT'
        CHECK (measure_type IN ('COUNT', 'WEIGHT', 'VOLUME')),
    base_unit    TEXT        NOT NULL, -- 'piece', 'g', 'ml'
    item_cls_cd  CHAR(10) REFERENCES product.kra_item_classes (code),
    tax_type_cd  CHAR(1)     NOT NULL DEFAULT 'B'
        REFERENCES product.kra_tax_types (code),
    active       BOOLEAN     NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (id, tenant_id)
);

CREATE INDEX idx_products_tenant ON product.products (tenant_id, active);
CREATE UNIQUE INDEX uq_products_sku ON product.products (tenant_id, sku)
    WHERE sku IS NOT NULL;
CREATE UNIQUE INDEX uq_products_plu ON product.products (tenant_id, plu_code)
    WHERE plu_code IS NOT NULL;

-- Units: how a product is bought/sold, and at what price
CREATE TABLE product.product_units
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id       UUID    NOT NULL,
    product_id      UUID    NOT NULL,
    name            TEXT    NOT NULL,                    -- 'kg', 'crate', 'bottle'
    factor          BIGINT  NOT NULL CHECK (factor > 0), -- base units per this unit
    price_cents     BIGINT CHECK (price_cents >= 0),     -- NULL = not sold in this unit
    is_base         BOOLEAN NOT NULL DEFAULT false,
    kra_qty_unit_cd TEXT,
    FOREIGN KEY (product_id, tenant_id)
        REFERENCES product.products (id, tenant_id) ON DELETE CASCADE,
    CHECK (NOT is_base OR factor = 1),
    UNIQUE (id, tenant_id)
);

CREATE UNIQUE INDEX uq_unit_name ON product.product_units (product_id, lower(name));
CREATE UNIQUE INDEX uq_one_base_unit ON product.product_units (product_id)
    WHERE is_base;

-- Barcodes per unit
CREATE TABLE product.unit_barcodes
(
    tenant_id UUID NOT NULL,
    barcode   TEXT NOT NULL,
    unit_id   UUID NOT NULL,
    PRIMARY KEY (tenant_id, barcode),
    FOREIGN KEY (unit_id, tenant_id)
        REFERENCES product.product_units (id, tenant_id) ON DELETE CASCADE
);