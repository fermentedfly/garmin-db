CREATE TABLE valid_activity_types(
    type TEXT PRIMARY KEY
);

INSERT INTO valid_activity_types (type) VALUES
                                               ('Cycling'),
                                               ('Swimming'),
                                               ('Running'),
                                               ('Hiking'),
                                               ('Climbing'),
                                               ('Walking'),
                                               ('Rowing'),
                                               ('Mountain Biking'),
                                               ('Mountaineering'),
                                               ('Snowshoeing');

CREATE TABLE activities(
    id SERIAL PRIMARY KEY,
    type TEXT REFERENCES valid_activity_types (type) NOT NULL,
    date TIMESTAMP NOT NULL UNIQUE,
    time INTERVAL NOT NULL,
    distance NUMERIC NOT NULL,
    elevation NUMERIC,
    title TEXT
);