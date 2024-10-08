-- Create table for storing TaskData
CREATE TABLE IF NOT EXISTS Task (
    task_id UUID PRIMARY KEY,
    start_date BIGINT NOT NULL,
    end_date BIGINT NOT NULL,
    occupancy INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    subtitle VARCHAR(255) NOT NULL,
    description TEXT
);
