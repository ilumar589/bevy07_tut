-- SCHEMA: master_catalog

-- drop schema master_catalog ;

create schema master_catalog
    authorization postgres;

create table master_catalog.product_detail_source
(
    id   bigserial(20) not null,
    uuid varchar(36) not null,
    code varchar(10) not null,
    primary key (id)
);

create unique index concurrently "uk_uuid" on master_catalog.product_detail_source
    using btree(digest("body_markdown", 'sha512'::text), "uuid");

create unique index concurrently "uk_code" on master_catalog.product_detail_source
    using btree(digest("body_markdown", 'sha512'::text), "code");

create table master_catalog.product
(
    id                    bigserial(20) not null,
    uuid                  varchar(36)  not null,
    vendor_id             varchar(36)  not null,
    vendor_product_number varchar(190) not null,
    primary key (id)
);

create unique index concurrently "uk_uuid" on master_catalog.product
    using btree(digest("body_markdown", 'sha512'::text), "uuid");

create unique index concurrently "uk_vendor_id_vendor_product_number" on master_catalog.product
    using btree(digest("body_markdown", 'sha512'::text), "vendor_id", "vendor_product_number");

create table master_catalog.product_detail
(
    id                       bigserial(20) not null,
    uuid                     varchar(36) not null,
    product_id               bigserial(20) not null,
    product_detail_source_id bigserial(20) not null,
    name mediumtext not null,
    description              mediumtext,
    primary key (id),
    constraint "fk_product_product_id" foreign key (product_id) references master_catalog.product (id),
    constraint "fk_product_detail_source_product_detail_source_id" foreign key (product_detail_source_id) references master_catalog.product_detail_source (id),
);

create unique index concurrently "uk_uuid" on master_catalog.product_detail
    using btree(digest("body_markdown", 'sha512'::text), "uuid");

create table master_catalog.bundle_packaging_unit
(
    id   bigserial(20) not null,
    uuid varchar(36) not null,
    code varchar(10) not null,
    primary key (id)
);

create unique index concurrently "uk_uuid" on master_catalog.bundle_packaging_unit
    using btree(digest("body_markdown", 'sha512'::text), "uuid");

create unique index concurrently "uk_code" on master_catalog.bundle_packaging_unit
    using btree(digest("body_markdown", 'sha512'::text), "code");

create table master_catalog.bundle
(
    id                  bigserial(20) not null,
    uuid                varchar(36)    not null,
    base_unit_amount    decimal(19, 4) not null,
    base_unit_id        bigserial(20) not null,
    sales_unit_id       bigserial(20) not null,
    imported_base_unit  varchar(10)    not null,
    imported_sales_unit varchar(10)    not null,
    product_id          bigserial(20) not null,
    primary key (id),
    constraint "fk_packaging_unit_base_unit_id" foreign key (base_unit_id) references master_catalog.bundle_packaging_unit (id),
    constraint "fk_packaging_unit_sales_unit_id" foreign key (sales_unit_id) references master_catalog.bundle_packaging_unit (id),
    constraint "fk_product_product_id" foreign key (product_id) references master_catalog.product (id),
);

create unique index concurrently "uk_uuid" on master_catalog.bundle
    using btree(digest("body_markdown", 'sha512'::text), "uuid");