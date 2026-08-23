-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
--    NOTE: Permissions are NOT applied automatically, since that    --
--    process is so dependent on implementation details & secrets    --
-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --

-- Posts is the older, all-in one, MariaDB-based service. Actions is the newer, separated service.
--   We want to separate the actions into its own microservice because they're write-heavy, whereas posts are almost entirely read-only
--   That will create scalability issues later when we do regional, RO-clones, etc. that doesn't work with writes.
-- The Maturity Service is a placeholder until we figure out the design pattern. But it's a database daemon service, grabbing
-- the list of new votes, and rerunning the calculation. It's not an interactive, data-driven REST service like everything else.
-- 
CREATE USER ${IC_POSTS_SVC_USER} WITH PASSWORD '${IC_POSTS_SVC_PASS}';
CREATE USER ${IC_ACTIONS_SVC_USER} WITH PASSWORD '${IC_ACTIONS_SVC_PASS}';
CREATE USER ${IC_MATURITY_SVC_USER} WITH PASSWORD '${IC_MATURITY_SVC_PASS}';

-- Actions Microservice
-- TBD - Right now this is MariaDB & needs to get migrated
-- These are all the writer functions, handling inserts into the right tables

-- Maturity Microservice
GRANT USAGE ON SCHEMA Results TO ${IC_MATURITY_SVC_USER};
GRANT EXECUTE ON FUNCTION Results.CalculateMaturityScore TO ${IC_MATURITY_SVC_USER};
-- TODO: Need something to look at queue depth & do health checks