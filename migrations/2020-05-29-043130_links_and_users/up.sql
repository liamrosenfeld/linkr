CREATE TABLE links(
  short TEXT PRIMARY KEY,
  long  TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  created_by INTEGER NOT NULL
);

CREATE TABLE users(
  id       INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  pw_hash  TEXT NOT NULL,
  manage_links BOOLEAN NOT NULL,
  manage_users BOOLEAN NOT NULL
);
