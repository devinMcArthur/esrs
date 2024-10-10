-- Add migration script here
CREATE TABLE snapshot_positions (
  key VARCHAR(255) PRIMARY KEY,
  value bigint NOT NULL
);
