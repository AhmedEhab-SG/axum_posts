-- Add down migration script here

-- Drop users table
DROP TABLE IF EXISTS users;

-- Drop custom enum type
DROP TYPE IF EXISTS user_role;

-- Drop custom enum type
DROP EXTENSION IF EXISTS pgcrypto;

