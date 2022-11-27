-- create a users table
CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    username TEXT NOT NULL,
    firstname TEXT NOT NULL,
    lastname TEXT,
    email TEXT NOT NULL UNIQUE,
    location TEXT,
    created_at timestamptz NOT NULL,
    modified_at timestamptz NOT NULL
)