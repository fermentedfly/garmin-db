CREATE TABLE activity_type
(
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    scale FLOAT NOT NULL
);

INSERT INTO activity_type (name, scale)
VALUES ('Cycling', 1),
       ('Swimming', 18),
       ('Open Water Swimming', 18),
       ('Running', 4),
       ('Hiking', 3),
       ('Climbing', 5),
       ('Walking', 1.5),
       ('Rowing', 2.5),
       ('Mountain Biking', 1),
       ('Mountaineering', 5),
       ('Snowshoeing', 2);

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