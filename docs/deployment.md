<!-- markdownlint-disable MD033 MD045 MD014 -->

# Server deployment

The server repository contains all the necessary components to easily set up
continuous integration and continuous deployment with just a VPS and GitHub
Actions.

## Setting up the VPS

The initial VPS setup should be straightforward to follow.
Assuming you are in a Debian bookworm server with root privileges:

### Dependencies

- Install Docker and Docker Compose following the official guide <https://docs.docker.com/engine/install/debian/>.

### SSH authentication

- Create a new user that you would use to deploy the server. Name it however you like

  ```console
  # adduser burrito
  # usermod -aG sudo burrito
  ```

- Generate a SSH key pair (or upload an existing one) for the new user

  ```console
  # su burrito
  $ ssh-keygen -t ed25519 -C "admin@burrito" -f ~/.ssh/id_ed25519
  ```

- Copy the public key to the server's `authorized_keys` file

  ```console
  $ cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys
  ```

- Test the connection to the server

  ```console
  $ ssh burrito@localhost
  ```

### Repository setup

- Now clone the server repository

  ```console
  $ sudo mkdir -p /opt/burrito/repo
  $ git clone git@github.com:burrito/server.git /opt/burrito/repo
  ```

- If you are using the GitHub CI pipeline, set the variables in the GitHub
  repository settings and you are ready to go. See
  [Uploading the production variables to GitHub](./env_vars.md#uploading-the-production-variables-to-github).

  Otherwise, manually populate an `.env` file.

### TLS certificates

- Carefully read the `docker/prod/nginx/nginx.conf`. It contains the server
  configuration and the paths to the TLS certificates.

- Follow the instructions of your CA (Let's Encrypt, for example) to generate
  the certificates. You'll probably need to create a new DNS record pointing to
  the server's IP address or a challenge TXT record.

- Generate a pair of TLS certificates using Let's Encrypt or any other CA.
  For example, using Certbot:

  ```console
  # certbot certonly --standalone -d api.contigosanmarcos.com
  ```

- Wherever domain you have used, make sure to update the `DOMAIN_NAME`
  variable in the `docker-compose.prod.yml` file.

- If you want to save the certs in a custom path (such as /opt/burrito/certs)
  make sure to update the `docker/prod/nginx/nginx.conf` file.

- Use a [DNS propagation checker](https://www.whatsmydns.net/) to check if
  your domain is correctly pointing to the server's IP address.

### Starting the container

- Start the server using Docker Compose

  ```console
  $ cd /opt/burrito/repo
  $ docker compose -f docker-compose.prod.yml up --build
  ```

- Every time you want to update the server, just pull the changes and restart
  the server

  ```console
  $ cd /opt/burrito/repo
  $ git pull origin main
  $ docker compose -f docker-compose.prod.yml up --build
  ```

  Altough, it is **strongly** recommended to use GitHub Actions pipeline instead.

## Production checks

Before deploying to production, make sure you have the following checks in place:

- [ ] You have properly set up the required [environment variables](./env_vars.md).
- [ ] You have [compiled the queries for offline mode](./database_management.md#compiling-the-queries-for-offline-mode) by running

  ```console
  cargo sqlx prepare
  ```

- [ ] The [database migrations](./database_management.md#creating-database-migrations)
  are up-to-date and have been tested locally
- [ ] You have make sure the server compiles and runs without any errors
