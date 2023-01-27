drop table if exists tags;

create table tags
(
        id serial primary key,
        tagname VARCHAR(25) not null,
		tag_time TIMESTAMP default now() not null
);


