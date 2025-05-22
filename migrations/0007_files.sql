CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS directories (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER REFERENCES directories(id) ON DELETE CASCADE,
    dir_name TEXT NOT NULL,
    dir_path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    directory_id INTEGER REFERENCES directories(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    uploaded_at TIMESTAMP NOT NULL DEFAULT NOW()
);
