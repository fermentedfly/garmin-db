CREATE TABLE activity_type
(
    id              SERIAL PRIMARY KEY,
    name            TEXT  NOT NULL, -- name of the activity type
    scale           FLOAT NOT NULL, -- scale to convert to equivalent kilometers
    elevation_scale FLOAT NOT NULL  -- elevation gain in meters to equivalent kilometers
);

INSERT INTO activity_type (name, scale, elevation_scale)
VALUES ('Cycling', 1, 0.025),
       ('Swimming', 18, 0),
       ('Running', 4, 0.025),
       ('Hiking', 3, 0.025),
       ('Walking', 1.5, 0.0125),
       ('Rowing', 2.5, 0),
       ('Mountain Biking', 1, 0.05),
       ('Mountaineering', 5, 0.05),
       ('Snowshoeing', 2, 0);

CREATE TABLE activities
(
    id               SERIAL PRIMARY KEY,
    title            TEXT      NOT NULL,
    activity_type_id SERIAL    NOT NULL,
    CONSTRAINT fk_activity_id FOREIGN KEY (activity_type_id) REFERENCES activity_type (id) ON DELETE RESTRICT,
    date             TIMESTAMP NOT NULL UNIQUE,
    time             INTERVAL  NOT NULL,
    distance         FLOAT     NOT NULL,
    elevation        FLOAT     NOT NULL
);