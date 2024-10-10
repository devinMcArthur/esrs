-- Add up migration script here
CREATE TABLE employee_hours (
  id SERIAL PRIMARY KEY,
  employee_namel VARCHAR(255) NOT NULL,
  jobsite_id INTEGER NOT NULL,
  hours_worked DECIMAL NOT NULL
);
