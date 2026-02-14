MariaDB Server
Andy R. Schmitz
2025-11-17

BUILD NOTES:
make [docker, deploy, clean]: Docker actions
Docker Image: msql - listen :3306
Required EnvVars:
* IC_DB_MARIA_ROOT_PASS

PURPOSE:
This is the docker container that runs/rebuilds an instance of MariaDB.
It will start from a blank slate, and backups or data loads will need to be 

BUSINESS FUNCTIONS:
Runs a modern version of MariaDB so devs can develop on their local machine
without having to install a database

TECHNICAL FUNCTIONS:
We use the latest LTS version, which is: 11.8.4 (as of 2025-11-17)

FUTURE ENHANCEMENTS
1) Develop a bunch of test cases, and create a backup file