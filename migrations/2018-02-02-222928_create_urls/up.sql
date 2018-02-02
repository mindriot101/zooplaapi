-- Your SQL goes here

CREATE TABLE houses_urls (
    id SERIAL PRIMARY KEY,
    details VARCHAR(255) NOT NULL,
    property_report VARCHAR(255) NOT NULL,
    house_id INTEGER REFERENCES houses(id)
)
