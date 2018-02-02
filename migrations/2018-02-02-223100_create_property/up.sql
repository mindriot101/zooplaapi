-- Your SQL goes here

CREATE TABLE houses_properties (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    num_bathrooms INTEGER NOT NULL,
    num_bedrooms INTEGER NOT NULL,
    num_floors INTEGER NOT NULL,
    house_id INTEGER REFERENCES houses(id)
)
