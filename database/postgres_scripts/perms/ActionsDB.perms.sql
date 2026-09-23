-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
--    NOTE: Permissions are NOT applied automatically, since that    --
--    process is so dependent on implementation details & secrets    --
-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --

-- Actions is the newer, separated service (from Posts). We want to separate the
-- actions into its own microservice because they're write-heavy, whereas posts
-- are almost entirely read-only. That will create scalability issues later when
-- we do regional, RO-clones, etc. that doesn't work with writes.
-- The Maturity Service is a placeholder until we figure out the design pattern. But
-- it's a database daemon service, grabbing the list of new votes, and rerunning the
-- calculation. It's not an interactive, data-driven REST service like everything else.

CREATE USER ${IC_ACTIONS_SVC_USER} WITH PASSWORD '${IC_ACTIONS_SVC_PASS}';
CREATE USER ${IC_MATURITY_SVC_USER} WITH PASSWORD '${IC_MATURITY_SVC_PASS}';

-- Actions Microservice
GRANT USAGE ON SCHEMA SoftMod TO ${IC_ACTIONS_SVC_USER};
GRANT EXECUTE ON FUNCTION SoftMod.SetMaturityVote TO ${IC_ACTIONS_SVC_USER};
GRANT EXECUTE ON FUNCTION SoftMod.SetCategoryVote TO ${IC_ACTIONS_SVC_USER};
GRANT EXECUTE ON FUNCTION SoftMod.SetTagVote TO ${IC_ACTIONS_SVC_USER};
GRANT EXECUTE ON FUNCTION SoftMod.SetReviewVote TO ${IC_ACTIONS_SVC_USER};
GRANT EXECUTE ON FUNCTION SoftMod.SetReviewVoteAnon TO ${IC_ACTIONS_SVC_USER};
-- TODO: Will need HardMod schema + functions too.

-- Maturity Microservice
GRANT USAGE ON SCHEMA Results TO ${IC_MATURITY_SVC_USER};
GRANT EXECUTE ON FUNCTION Results.CalculateMaturityScore TO ${IC_MATURITY_SVC_USER};
-- TODO: Need something to look at queue depth & do health checks