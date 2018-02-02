-- Your SQL goes here

ALTER TABLE houses DROP COLUMN price;

ALTER TABLE houses ADD COLUMN price INTEGER NOT NULL;

ALTER TABLE houses ADD COLUMN first_published_date VARCHAR(32) NOT NULL;
ALTER TABLE houses ADD COLUMN last_published_date VARCHAR(32) NOT NULL;
