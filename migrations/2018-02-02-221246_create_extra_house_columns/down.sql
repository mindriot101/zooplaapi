-- This file should undo anything in `up.sql`

ALTER TABLE houses DROP COLUMN price;
ALTER TABLE houses ADD COLUMN price SERIAL NOT NULL;

ALTER TABLE houses DROP COLUMN first_published_date;
ALTER TABLE houses DROP COLUMN last_published_date;
