drop table if exists comments;
drop table if exists users;
drop table if exists likes;
drop table if exists categories;
drop table if exists articles;
drop table if exists tags;
drop table if exists article_tag;


create table comments 
(
  id serial primary key NOT NULL,
  content varchar(50) NOT NULL,
  user_id int NOT NULL,
  article_id int NOT NULL,
  create_at timestamp default now() NOT NULL
);

create table users 
(
  id serial primary key NOT NULL,
  email varchar(30) NOT NULL,
  name varchar(30) NOT NULL,
  password varchar(30) NOT NULL,
  avator varchar(50) DEFAULT NULL,
  type varchar(16) DEFAULT 'normal',
  token varchar(50) DEFAULT NULL
);

create table likes 
(
  user_id int NOT NULL,
  article_id int NOT NULL,
  create_at timestamp default now() NOT NULL
); 

create table categories 
(
  id serial primary key NOT NULL,
  name varchar(30) NOT NULL,
  create_at timestamp default now() NOT NULL
);

create table articles 
(
  id serial primary key NOT NULL,
  author_id int NOT NULL,
  content text NOT NULL,
  category_id int NOT NULL,
  read_num int DEFAULT 0,
  is_public boolean DEFAULT false,
  create_at timestamp default now() NOT NULL
);

create table tags 
(
  id serial primary key NOT NULL,
  name varchar(30) NOT NULL,
  create_at timestamp default now() NOT NULL
);

create table article_tag 
(
  article_id int NOT NULL,
  tag_id int NOT NULL
);
