SELECT view_num, category, favorite_num, comment_num, title, tagname, article_time, articles.id
from articles
join tags_articles
on articles.id = tags_articles.article_id
join tags
on tags.id = tags_articles.tag_id
where category = '222';
