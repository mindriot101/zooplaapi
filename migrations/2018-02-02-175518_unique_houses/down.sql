-- This file should undo anything in `up.sql`

ALTER TABLE houses ALTER COLUMN price DROP NOT NULL;
