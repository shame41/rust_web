drop table if exists tags_articles;

create table tags_articles
(
        id serial primary key not null,
        tag_id INT not null,
		article_id INT not null
);
