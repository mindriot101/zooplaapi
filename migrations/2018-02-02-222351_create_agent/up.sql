-- Your SQL goes here

CREATE TABLE houses_agents (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    house_id INTEGER REFERENCES houses(id)
)
