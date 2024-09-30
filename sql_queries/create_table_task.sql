-- Create table for storing TaskData
CREATE TABLE IF NOT EXISTS Task (
    taks_id UUID PRIMARY KEY,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    occupancy INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    subtitle VARCHAR(255) NOT NULL,
    description TEXT
);
