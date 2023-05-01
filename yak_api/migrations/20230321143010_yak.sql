-- Create the YAK table
CREATE TABLE IF NOT EXISTS yak (
    id serial PRIMARY KEY,
    name varchar(255) NOT NULL,
    age real NOT NULL,
    age_last_shaved real DEFAULT 0 -- Age since last shaved in days
);

-- Insert initial data into the YAK table
INSERT INTO yak (name, age, age_last_shaved)
VALUES
    ('Betty', 5, 0),
    ('Betsy', 2, 0),
    ('Tommy', 3, 0),
    ('YakYak-binks', 6, 0),
    ('SOmething', 2, 0),
    ('SomethingElse', 1, 0);

-- Set default value for AGE_LAST_SHAVED column
ALTER TABLE yak
ALTER COLUMN age_last_shaved SET DEFAULT 0;
