CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    email VARCHAR(128) NOT NULL UNIQUE,
    passwordhash VARCHAR(256) NOT NULL,
    admin BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- default admin user
INSERT INTO users (name, email, passwordhash, admin)
VALUES ('Admin User', 'nicolas.theo.frey@gmail.com', '$2b$12$iZvb3cEc70hC5lYtmRCLh.Yb7vdY2D2jnEDdW8GrVquDWNiMvnxJm', true);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author INTEGER REFERENCES users(id) ON DELETE SET NULL,
    title VARCHAR(256) NOT NULL,
    markdown_content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

-- for videos, code snippets, videos or gifs
CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    static_path VARCHAR(256) NOT NULL,
    media_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
