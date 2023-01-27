drop table if exists users;

create table users
(
	id serial primary key,
	username VARCHAR(25) not null,
	email VARCHAR(25) not null,
	password VARCHAR(25) not null,
	avatar VARCHAR(25) not null,
	account_type boolean not null
);
