-- add column
ALTER TABLE post ADD COLUMN slug TEXT NOT NULL DEFAULT "NO_SLUG";
-- add dynamic default values (using first word of title as temporary slug)
UPDATE post SET slug = substr(title, 1, instr(title || ' ', ' ') - 1);

-- TODO: fix this later. There's no alter column in sqlite
-- set column not null
--ALTER TABLE post ALTER COLUMN slug TEXT NOT NULL;

