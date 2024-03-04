-- create table users
create table users
(
    id         varchar(13)  not null primary key,
    email      varchar(255) not null,
    password   varchar(60)  not null,
    is_banned  boolean      not null default false,
    updated_at timestamp    not null default current_timestamp,
    created_at timestamp    not null default current_timestamp,
    deleted_at timestamp -- soft delete
);

create trigger trigger_users_gen_id
    before insert
    on users
    for each row
    execute procedure unique_short_id();

create index users_email_idx
    on users (email);

create unique index users_email_uq
    on users (email);

-- create table profile
create table profile
(
    id           varchar(13) primary key,
    first_name   text,
    last_name    text,
    birth_date   date,
    phone_number text,
    user_id      varchar(13)                         not null,
    updated_at   timestamp default current_timestamp not null,
    created_at   timestamp default current_timestamp not null
);

create trigger trigger_profiles_gen_id
    before insert
    on profile
    for each row
    execute procedure unique_short_id();

alter table profile
    add constraint profile_user_id_f_key
        foreign key (user_id) references users (id);

-- create table sessions
create table sessions
(
    id         varchar(13) not null primary key,
    token      varchar(60) not null,
    is_valid   boolean     not null default true,
    ip_address text,
    user_agent text,
    user_id    varchar(13) not null,
    created_at timestamp   not null default current_timestamp,
    updated_at timestamp   not null default current_timestamp,
    deleted_at timestamp -- soft delete
);

create trigger trigger_sessions_gen_id
    before insert
    on sessions
    for each row
    execute procedure unique_short_id();

alter table sessions
    add constraint sessions_user_id_f_key
        foreign key (user_id) references users (id);

-- create table services
create table services
(
    id         varchar(13) not null primary key,
    title      text        not null,
    number     integer     not null,
    user_id    varchar(13) not null,
    created_at timestamp   not null default current_timestamp,
    updated_at timestamp   not null default current_timestamp,
    deleted_at timestamp -- soft delete
);

create trigger trigger_services_gen_id
    before insert
    on services
    for each row
    execute procedure unique_short_id();

alter table services
    add constraint services_user_id_f_key
        foreign key (user_id) references users (id);

-- create table invoices
create table invoices
(
    id             varchar(13) not null primary key,
    number         text        not null,
    type           text        not null,
    client_name    text        not null,
    client_address text        not null,
    client_vat     text        not null,
    payment_type   text        not null,
    due_date       date        not null,
    invoice_date   timestamp   not null,
    invoice_sent   date,
    notes          text,
    admin_notes    text,
    repeat         integer,
    language       text        not null,
    is_invoice     boolean     not null default true,
    user_id        varchar(13) not null,
    created_at     timestamp   not null default current_timestamp,
    updated_at     timestamp   not null default current_timestamp,
    deleted_at     timestamp -- soft delete
);

create trigger trigger_invoices_gen_id
    before insert
    on invoices
    for each row
    execute procedure unique_short_id();

alter table invoices
    add constraint invoices_user_id_f_key
        foreign key (user_id) references users (id);

-- create table invoice_items
create table invoice_items
(
    id          varchar(13)      not null primary key,
    description text             not null default '',
    unit        text             not null,
    quantity    integer          not null,
    price       double precision not null,
    discount    integer          not null default 0,
    tax         integer          not null default 0,
    invoice_id  varchar(13)      not null,
    created_at  timestamp        not null default current_timestamp,
    updated_at  timestamp        not null default current_timestamp,
    deleted_at  timestamp -- soft delete
);

create trigger trigger_invoice_items_gen_id
    before insert
    on invoice_items
    for each row
    execute procedure unique_short_id();

alter table invoice_items
    add constraint invoice_items_invoice_id_f_key
        foreign key (invoice_id) references invoices (id);

-- create table invoice_notes
create table invoice_notes
(
    id         varchar(13) not null primary key,
    note       text        not null,
    user_id    varchar(13) not null,
    created_at timestamp   not null default current_timestamp,
    updated_at timestamp   not null default current_timestamp,
    deleted_at timestamp -- soft delete
);

create trigger trigger_invoice_notes_gen_id
    before insert
    on invoice_notes
    for each row
    execute procedure unique_short_id();


alter table invoice_notes
    add constraint invoice_notes_user_id_f_key
        foreign key (user_id) references users (id);

-- create table clients
create table clients
(
    id                    varchar(13) not null primary key,
    name                  text        not null,
    vat                   text        not null,
    address               text        not null,
    city                  text        not null,
    zip                   text        not null,
    phone                 text,
    email                 text,
    iban                  text,
    notes                 text,
    admin_notes           text,
    tax                   integer     not null,
    liquidator_first_name text,
    liquidator_last_name  text,
    mobile                text,
    url                   text,
    nkd                   text,
    swift                 text,
    eu_vat                text,
    office                text,
    tax_notes             text,
    tax_details           text,
    is_client             boolean     not null default false,
    country               text,
    type                  text,
    due_date              date,
    user_id               varchar(13) not null,
    created_at            timestamp   not null default current_timestamp,
    updated_at            timestamp   not null default current_timestamp,
    deleted_at            timestamp -- soft delete
);

create trigger trigger_clients_gen_id
    before insert
    on clients
    for each row
    execute procedure unique_short_id();

alter table clients
    add constraint clients_user_id_f_key
        foreign key (user_id) references users (id);

-- create table certificates
create table certificates
(
    id                  varchar(13) not null primary key,
    object_key          text        not null,
    bucket_name         text        not null,
    password            text,
    location_code       text,
    operator_code       text,
    operator_first_name text,
    operator_last_name  text,
    operator_vat        text,
    user_id             varchar(13) not null,
    created_at          timestamp   not null default current_timestamp,
    updated_at          timestamp   not null default current_timestamp,
    deleted_at          timestamp -- soft delete
);

create trigger trigger_certificates_gen_id
    before insert
    on certificates
    for each row
    execute procedure unique_short_id();

alter table certificates
    add constraint certificates_user_id_f_key
        foreign key (user_id) references users (id);


-- create table catalogs
create table catalogs
(
    id         varchar(13) not null primary key,
    sku        text,
    title      text,
    unit       text,
    quantity   integer,
    price      double precision,
    discount   integer,
    tax        integer,
    user_id    varchar(13) not null,
    created_at timestamp   not null default current_timestamp,
    updated_at timestamp   not null default current_timestamp,
    deleted_at timestamp -- soft delete
);

create trigger trigger_catalogs_gen_id
    before insert
    on catalogs
    for each row
    execute procedure unique_short_id();

alter table catalogs
    add constraint catalog_user_id_f_key
        foreign key (user_id) references users (id);

-- create table roles
create table roles
(
    id         varchar(13) not null,
    name       text        not null,
    model      text        not null,
    created_at timestamp   not null default current_timestamp,
    updated_at timestamp   not null default current_timestamp,
    deleted_at timestamp -- soft delete
);

create trigger trigger_roles_gen_id
    before insert
    on roles
    for each row
    execute procedure unique_short_id();

alter table roles
    add constraint roles_name_uq
        unique (name);

alter table roles
    add constraint roles_roles_pk
        primary key (id);

-- create table permissions
create table permissions
(
    id         varchar(13)                         not null,
    name       text                                not null,
    role_id    varchar(13)                         not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    deleted_at timestamp -- soft delete
);

create trigger trigger_permissions_gen_id
    before insert
    on permissions
    for each row
    execute procedure unique_short_id();

alter table permissions
    add constraint permissions_pk
        primary key (id);

alter table permissions
    add constraint permissions_roles_id_f_key
        foreign key (role_id) references roles (id);
