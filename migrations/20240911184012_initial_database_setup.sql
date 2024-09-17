-- Add migration script here
CREATE TABLE jobsites (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);
