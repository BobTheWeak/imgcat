-- Auth Microservice
GRANT USAGE ON SCHEMA UserDB TO IC_UDB_USER;
GRANT EXECUTE ON FUNCTION UserDB.GetAccountId TO IC_UDB_USER;
GRANT EXECUTE ON FUNCTION UserDB.GetAccountData TO IC_UDB_USER;
GRANT EXECUTE ON FUNCTION UserDB.CreateAccount TO IC_UDB_USER;
GRANT EXECUTE ON FUNCTION UserDB.IsUsernameFree TO IC_UDB_USER;

GRANT USAGE ON SCHEMA Legal TO IC_UDB_USER;
GRANT EXECUTE ON FUNCTION Legal.IsAgeNeededOnSignup TO IC_UDB_USER;

-- Posts Microservice
-- TODO: It's currently using IC_UDB_USER, not it's own service account

-- Softmod Microservice
-- N/A - All functionality currently uses MariaDB

-- Users Microservice
GRANT USAGE ON SCHEMA UsersSvc TO IC_USERS_SVC_USER;
GRANT EXECUTE ON FUNCTION UsersSvc.GetUserBadges TO IC_USERS_SVC_USER;
GRANT EXECUTE ON FUNCTION UsersSvc.GetAccountPreferences TO IC_USERS_SVC_USER;
GRANT EXECUTE ON FUNCTION UsersSvc.SetAccountPreferences TO IC_USERS_SVC_USER;