drop table if exists comments;

create table comments
(
        id serial primary key,
        content TEXT not null,
        comment_time TIMESTAMP default now() not null,
		comment_status boolean not null,
		article_id INT not null,
		user_id INT not null
)
