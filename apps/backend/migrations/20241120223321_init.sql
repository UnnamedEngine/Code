--------------------------------------------------------------------------------
-- SCHEMA CREATION
--------------------------------------------------------------------------------
CREATE SCHEMA account;

--------------------------------------------------------------------------------
-- PUBLIC SCHEMA
--------------------------------------------------------------------------------
CREATE TABLE public.i18n (
  id SERIAL PRIMARY KEY,
  written_name VARCHAR(255) NOT NULL UNIQUE,
  language_code VARCHAR(5) NOT NULL UNIQUE
);

--------------------------------------------------------------------------------
-- ACCOUNT SCHEMA
--------------------------------------------------------------------------------
-- Punishments
CREATE TABLE account.punishments (
  id SERIAL PRIMARY KEY,
  slug VARCHAR(64) NOT NULL UNIQUE,
  default_duration BIGINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Punishment Translations
CREATE TABLE account.punishment_translations (
  punishment_id INT REFERENCES account.punishments(id) ON DELETE CASCADE,
  i18n_id INT REFERENCES public.i18n(id) ON DELETE CASCADE,
  title VARCHAR(128) NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (punishment_id, i18n_id)
);

-- Status
CREATE TABLE account.status (
  id SERIAL PRIMARY KEY,
  slug VARCHAR(64) NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Status Translations
CREATE TABLE account.status_translations (
  status_id INT REFERENCES account.status(id) ON DELETE CASCADE,
  i18n_id INT REFERENCES public.i18n(id) ON DELETE CASCADE,
  title VARCHAR(128) NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (status_id, i18n_id)
);

-- Users
CREATE TABLE account.users (
  id SERIAL PRIMARY KEY,
  status_id INT NOT NULL REFERENCES account.status(id),
  email VARCHAR(256) NOT NULL UNIQUE,
  username VARCHAR(64) NOT NULL UNIQUE,
  display_name VARCHAR(128) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_login TIMESTAMP NULL
);

-- User Punishments
CREATE TABLE account.user_punishments (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES account.users(id) ON DELETE CASCADE,
  punishment_id INT NOT NULL REFERENCES account.punishments(id) ON DELETE CASCADE,
  applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  duration BIGINT NOT NULL
);

--------------------------------------------------------------------------------
-- TRIGGER FUNCTIONS
--------------------------------------------------------------------------------
-- Function to update the `updated_at` column
CREATE OR REPLACE FUNCTION account.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

--------------------------------------------------------------------------------
-- ACCOUNT TRIGGERS
--------------------------------------------------------------------------------
-- `updated_at` trigger for punishments
CREATE TRIGGER update_punishments_updated_at
BEFORE UPDATE ON account.punishments
FOR EACH ROW EXECUTE FUNCTION account.update_updated_at_column();

-- `updated_at` trigger for punishment translations
CREATE TRIGGER update_punishments_updated_at
BEFORE UPDATE ON account.punishment_translations
FOR EACH ROW EXECUTE FUNCTION account.update_updated_at_column();

-- `updated_at` trigger for status
CREATE TRIGGER update_status_updated_at
BEFORE UPDATE ON account.status
FOR EACH ROW EXECUTE FUNCTION account.update_updated_at_column();

-- `updated_at` trigger for status translations
CREATE TRIGGER update_status_updated_at
BEFORE UPDATE ON account.status_translations
FOR EACH ROW EXECUTE FUNCTION account.update_updated_at_column();

-- `updated_at` trigger for users
CREATE TRIGGER update_users_updated_at
BEFORE UPDATE ON account.users
FOR EACH ROW EXECUTE FUNCTION account.update_updated_at_column();
