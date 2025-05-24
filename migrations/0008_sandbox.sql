CREATE TABLE IF NOT EXISTS sandbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    directory_id INTEGER REFERENCES directories(id) ON DELETE CASCADE NOT NULL,
    slug TEXT NOT NULL
);