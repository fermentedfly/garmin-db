ALTER TABLE activity_type
    ADD COLUMN elevation_scale FLOAT;   -- elevation gain in meters to equivalent kilometers

UPDATE activity_type as a
SET elevation_scale = c.elevation_scale
FROM (values ('Cycling', 0.025),
             ('Mountain Biking', 0.05),
             ('Running', 0.025),
             ('Walking', 0.0125),
             ('Hiking', 0.025),
             ('Climbing', 0.05),
             ('Mountaineering', 0.05)
     ) as c(name, elevation_scale)
WHERE a.name = c.name