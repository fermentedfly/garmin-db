DROP TABLE IF EXISTS scale;

CREATE TABLE scale (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    scale NUMERIC NOT NULL
);

INSERT INTO scale (name, scale) VALUES ('Cycling', 1);
INSERT INTO scale (name, scale) VALUES ('Swimming', 18);
INSERT INTO scale (name, scale) VALUES ('Running', 4);
INSERT INTO scale (name, scale) VALUES ('Hiking', 3);
INSERT INTO scale (name, scale) VALUES ('Climbing', 3);
INSERT INTO scale (name, scale) VALUES ('Walking', 1.5);
INSERT INTO scale (name, scale) VALUES ('Rowing', 2.5);