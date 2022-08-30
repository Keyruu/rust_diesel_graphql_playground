ALTER TABLE posts DROP CONSTRAINT fk_posts_authors;

ALTER TABLE posts
  DROP COLUMN author_id;

DROP TABLE authors;
