-- Add down migration script here

DROP INDEX IF EXISTS posts_user_id_idx;

DROP TABLE IF EXISTS posts;
