ALTER TABLE directories ADD UNIQUE (dir_path);
ALTER TABLE files ADD UNIQUE (file_path);
ALTER TABLE sandbox ADD UNIQUE (slug);