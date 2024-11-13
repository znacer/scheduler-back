-- Create table for storing TaskData
CREATE TABLE IF NOT EXISTS task (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    start BIGINT,
    duration BIGINT,
    schedule_id BIGSERIAL REFERENCES schedule (id),
    description TEXT,
    category INTEGER
  )
