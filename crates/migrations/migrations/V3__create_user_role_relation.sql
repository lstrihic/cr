create table user_roles (
    user_id varchar(13),
    role_id varchar(13),
    primary key (user_id, role_id),
    foreign key (user_id) references users (id),
    foreign key (role_id) references roles (id)
);
