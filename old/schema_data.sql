DROP TABLE IF EXISTS data;

CREATE TABLE data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    'type' INTEGER NOT NULL,
    date TEXT NOT NULL,
    time TEXT NOT NULL,
    distance NUMERIC NOT NULL,
    elevation NUMERIC,
    title TEXT,
    FOREIGN KEY ('type') REFERENCES scale (id),
    UNIQUE (date)
);