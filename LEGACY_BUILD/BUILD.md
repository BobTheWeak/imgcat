# Build Process

## Software Requirements

* Docker with Docker Compose
* S3 blob storage (Using Cloudflare R2)

## Initial Setup

1. Set up an S3 cloud provider (Cloudflare R2 recommended)
   1. Make your bucket public with a development url
2. Run the bash script `initial_setup`
   1. The script assumes that you have a sed version that has -i. Some BSD kernels do not ship with this feature.
   2. TODO: add Windows CMD script. Windows can run the bash script in WSL for now.
3. Add S3 credentials to `dev.env` file

## Docker build

The docker build is used for developers of any part of the software to not need a set of build tools for every language used to be installed on their computer. For example, frontend developers will not need rust or cargo installed on their computer.

1. Install Docker
   * Windows users want to install WSL ([instructions](https://learn.microsoft.com/en-us/windows/wsl/install)) and [Docker Desktop](https://www.docker.com/products/docker-desktop/)
   * Linux users should install Docker using the package manager of their choice
2. Clone the repository
3. Follow the steps for the Initial Setup found above
4. Start the containers with docker compose
   * `docker compose up -d`
   * Note that if you need to re-run the database init scripts, you simply need to delete the `db_data` folder and re-run the docker container.
