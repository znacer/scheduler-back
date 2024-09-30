CREATE TABLE IF NOT EXISTS schedule (
    id UUID PRIMARY KEY,
    tasks UUID[] NOT NULL,
    label_id UUID NOT NULL REFERENCES labels (label_id)
);
