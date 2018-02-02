-- Your SQL goes here

CREATE TABLE house_categories (
    id SERIAL PRIMARY KEY,
    property_type VARCHAR(255) NOT NULL,
    category VARCHAR(255) NOT NULL,
    price_modifier VARCHAR(255),
    house_id INTEGER REFERENCES houses(id)
)
