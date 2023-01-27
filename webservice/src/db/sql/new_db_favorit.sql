drop table if exists favorite;

create table favorite
(
        id serial primary key,
		article_id INT not null,
		user_id INT not null,
		favorite_status boolean not null
);
