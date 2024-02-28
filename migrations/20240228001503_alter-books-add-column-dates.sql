-- Add migration script here
ALTER TABLE books
    ADD COLUMN creation_time TIMESTAMPTZ NOT NULL DEFAULT now(),
    ADD COLUMN deletion_time TIMESTAMPTZ DEFAULT NULL,
    ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NULL;
    