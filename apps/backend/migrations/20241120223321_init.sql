--------------------------------------------------------------------------------
-- SCHEMA CREATION
--------------------------------------------------------------------------------
CREATE SCHEMA account;

--------------------------------------------------------------------------------
-- ACCOUNT SCHEMA
--------------------------------------------------------------------------------
-- Status
CREATE TABLE account.status (
  id SERIAL PRIMARY KEY,
  slug VARCHAR(64) NOT NULL UNIQUE,
  created_at TIMESTAMPTZ NOT NULL
);

-- Users
CREATE TABLE account.users (
  id BIGINT PRIMARY KEY,
  status_id INT NOT NULL REFERENCES account.status(id),
  email VARCHAR(256) NOT NULL UNIQUE,
  username VARCHAR(64) NOT NULL UNIQUE,
  display_name VARCHAR(128) NULL,
  created_at TIMESTAMPTZ NOT NULL,
  last_login TIMESTAMPTZ NULL
);

--------------------------------------------------------------------------------
-- DEFAULT ACCOUNT SCHEMA DATA
--------------------------------------------------------------------------------
INSERT INTO account.status (slug, created_at)
VALUES
  ('unverified', NOW()),
  ('verified', NOW());
