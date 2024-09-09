-- Every table should have a `created_at` and `updated_at`. The `created_at`
-- column can begin `default now()`, but `updated_at` requires more work.
--
-- These two functions make it automatic, as long as we call
-- `trigger_updated_at` after creating the table, like so:
--
--     select trigger_updated_at('<table name>');
--
create or replace function set_updated_at() returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(
    tablename regclass
) returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

-- A text collation that sorts text case-insensitively. Helpful for
-- case-insensitive `UNIQUE` indexes on columns for usernames or emails.
create collation case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);
