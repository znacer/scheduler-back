CREATE TABLE IF NOT EXISTS labels (
    label_id UUID PRIMARY KEY,
    icon TEXT,
    title VARCHAR(255) NOT NULL,
    subtitle VARCHAR(255) NOT NULL
);

