# Burrito server

The Burrito API serves all the server data (app versions, notifications, flags, sessions)
and is the communication bridge between the bus driver application and user application.

## Running the server

### For development

Because sqlx checks the queries at compile time, the development databse must be running.
The easiest way is to run the development db container with.

```bash
# See docker-compose.yml
docker compose up --build
```

Once you have the database running, then you start the server with

```bash
cargo run
```

### For production

The production container can be started with

```bash
# See docker-compose.prod.yml
docker compose -f docker-compose.prod.yml up --build
```

Please make sure to double check that your .env variables and Docker variables are set
correctly.

## Management

The only thing you'll ever need to manage are database migrations. For this purpose, there
is the [sqlx cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).

```bash
cargo install sqlx-cli

# Run migrations
sqlx migrate run

# Reset the database and migrations
sqlx database reset --force
```

## Mocking routes for showcasing purposes

No bus driver working today? No problem! You can mock the routes by setting `IS_MOCKED=true` in
the env file. The mocked route will be read from `static/mocks/*.json`. See `mock.rs` for more
details.

Once you have set `IS_MOCKED=true`, you can start the server as usual. Mocking works by sending
`POST /driver` requests to ourselves, iterating over the mocked route records.
