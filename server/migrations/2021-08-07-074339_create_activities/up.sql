CREATE TABLE valid_activity_types(
    activity_type TEXT PRIMARY KEY
);

INSERT INTO valid_activity_types (activity_type) VALUES
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
    activity_type TEXT REFERENCES valid_activity_types (activity_type) NOT NULL,
    date TIMESTAMP NOT NULL UNIQUE,
    time TIME NOT NULL,
    distance FLOAT NOT NULL,
    elevation FLOAT NOT NULL,
    title TEXT
);