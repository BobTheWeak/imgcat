SoftMod Actions Microservice
Andy R. Schmitz
2025-11-12

BUILD NOTES:
make dev: Local build (Rust)
make [docker, deploy, clean]: Docker actions
Docker Image: smact - listen :8080
Required EnvVars:
* IC_DB_IS_MARIA
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
1) This opens a new DB connection per request. Obviously this is bad/temporary.
2) Unit tests. This is early & I don't have the luxury of time. Please, help me not suck.
3) Support for PostgreSQL. Using this w/o IC_DB_IS_MARIA set will break.
4) Consider throwing these votes into a message queue instead of a direct DB call.
