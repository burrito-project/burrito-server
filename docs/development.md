<!-- markdownlint-disable MD033 MD045 -->

# Development in the burrito project

## Running the server for development

Because [sqlx](https://github.com/launchbadge/sqlx) checks the queries at
compile time, the development databse must be running.
The easiest way is to run the development db container with.

```bash
# See docker-compose.yml
docker compose up --build
```

Once you have the database running, then you start the server with

```bash
cargo run
```

<div class="warning">
The production workflow is completely different from this. Please refer to the
deployment section for more information.
</div>

Please make sure to double check that your .env variables and Docker variables are set
correctly.

## VSCode configuration

You may want to add the following extensions:

- Rust
- rust-analyzer
- Dependi
- PostgreSQL Language Server

And the following configurations may be useful to add:

```json
{
  "[rust]": {
    "editor.formatOnSave": true,
  },
  "files.associations": {
    "*.sql": "postgres"
  },
  "plpgsqlLanguageServer.definitionFiles": [
    "**/*.sql",
  ],
  // Development postgres variables from your .env
  "plpgsqlLanguageServer.database": "burrito_app",
  "plpgsqlLanguageServer.host": "localhost",
  "plpgsqlLanguageServer.user": "admin",
  "plpgsqlLanguageServer.password": "dontship",
}
```

## Local database management

Read the [database management](./database_management.md) to know how to
locally manage your database, create and revert migrations.

## Mocking routes for showcasing purposes

No bus driver working today? No problem! You can mock the routes by setting `IS_MOCKED=true` in
the env file. The mocked route will be read from `static/mocks/*.json`. See `mock.rs` for more
details.

Once you have set `IS_MOCKED=true`, you can start the server as usual. Mocking works by sending
`POST /driver` requests to ourselves, iterating over the mocked route records.
