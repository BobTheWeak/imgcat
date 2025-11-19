SoftMod Actions Microservice
Andy R. Schmitz
2025-11-12

BUILD NOTES:
make dev: Local build (Rust)
make [docker, deploy, clean]: Docker actions
Docker Image: smact - listen :8080
Rust Feature:
* use_mariadb - Compiles in the MariaDB/MySQL libraries (else PostgreSQL)
Required EnvVars:
* IC_DB_HOST
* IC_DB_USER
* IC_DB_PASS
Optional EnvVars:
* IC_HEADER_USER_ID (default: x-ic-user-id)
* IC_HEADER_USER_IP (default: x-ic-user-ip)

PURPOSE:
This is the microservice that supports "soft-moderation" functionality.
Normal users (power users? volunteers?) can do some basic post modding.

BUSINESS FUNCTIONS:
Authenticated users can leverage this to:
* add tags to posts without them
* set the maturity level (ie. this post is NSFW)
* set categories for divisive material (aka: politics)
* flag a post as needing mod review

Anonymous users (general internet folks w/o accounts) can:
* flag a post as needing mod review

As imagined, none of these actions occur automatically, they're simply recording
votes. So if a post accumulates (for example) 5 votes for the "star trek" tag,
then we'll add that tag automatically, or sooner if a mod approves it.

TECHNICAL FUNCTIONS:
These are the middleware functions that the front-end calls.
At the moment, it connects to MariaDB (PostgreSQL TBD), and calls a SP.
It stuffs the request into a queue table, and considers it one vote.
Another webservice (TODO) will read updates, tally votes, & do magic.

FUTURE ENHANCEMENTS
1) Unit tests. This is early & I don't have the luxury of time. Please, help me not suck.
2) Support for PostgreSQL. It needs u64 encode/decode logic before this will work.
3) Consider throwing these votes into a message queue instead of a direct DB call.
