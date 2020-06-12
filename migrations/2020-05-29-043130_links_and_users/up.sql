CREATE TABLE links(
  short TEXT PRIMARY KEY,
  long  TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  created_by INTEGER NOT NULL
);

CREATE TABLE users(
  id       SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  pw_hash  TEXT NOT NULL
);
