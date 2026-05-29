-- TODO: Be more selective. This is fine, as intended, but still...
GRANT USAGE ON SCHEMA UserDB TO imgcat_jwt;
GRANT EXECUTE ON ALL ROUTINES IN SCHEMA UserDB TO imgcat_jwt;

GRANT USAGE ON SCHEMA Legal TO imgcat_jwt;
GRANT EXECUTE ON ALL ROUTINES IN SCHEMA Legal TO imgcat_jwt;