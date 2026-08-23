-- -- -- DATABASE PURPOSE -- -- --
-- The ActionsDB contains the list of all content voting actions by any user
-- The three main schemas are dumb CRUD storage:
--   Softmod contains user-driven actions, such as reporting a post for bad content
--   Hardmod is mostly a clone, but with customizations designed for moderator actions
--   Automod (not implemented yet) contains sytems for automated content detection, etc
-- Then the ContentManager service runs, consolidates the opinions from every source
--   Results stores that analyis & all the mathy, consolidated things needed for that

-- -- -- DATABASE INFRASTRUCTURE -- -- --
-- From a scalability perspective, ActionsDB should be write-centric.
-- This means we should NOT plop a read-API onto Results, feeding the data to the web
-- Once we finish calculating the data for a post, those results should get copied to
-- a read-centric database (like PostsDB), which can be replicated out to RO clones.

CREATE SCHEMA IF NOT EXISTS Softmod;
CREATE SCHEMA IF NOT EXISTS Hardmod;
-- CREATE SCHEMA IF NOT EXISTS Automod;
CREATE SCHEMA IF NOT EXISTS Results;

-- Our security model doesn't let users run arbitrary SQL,
-- so any default permissions are unnecessary
REVOKE ALL PRIVILEGES ON SCHEMA public FROM PUBLIC;
