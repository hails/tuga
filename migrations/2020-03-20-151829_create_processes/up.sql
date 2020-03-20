-- Your SQL goes here

CREATE TABLE processes (
  id serial PRIMARY KEY,
  code text NOT NULL,
  telegram_user_id TEXT NOT NULL,
  status text NOT NULL,
  created_at timestamp  NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),

  UNIQUE(telegram_user_Id, code)
);

