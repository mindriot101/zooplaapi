-- Your SQL goes here

CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    longitude REAL NOT NULL,
    latitude REAL NOT NULL,
    street_name VARCHAR(255) NOT NULL,
    displayable_address VARCHAR(255) NOT NULL,
    house_id INTEGER REFERENCES houses(id)
)
