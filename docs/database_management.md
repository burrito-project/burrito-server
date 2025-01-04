# Database managment

Our database driver, [sqlx](https://github.com/launchbadge/sqlx), already solves
the majority of the database management issues.

If you are not familiar with sqlx,
[this video](https://www.youtube.com/watch?v=TCERYbgvbq0)
is a good starting point:

First, make sure you have the
[sqlx client](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query)
installed.

```bash
cargo install sqlx-cli
```

## Creating database migrations

Database migrations live in the `migrations/` directory. Create one for each
change you want to make to the database schema.

For example, let's say you want to add a new `profile_image` column to
the `users` table, you can create a new migration like this:

```bash
sqlx migrate add -rs add_users_profile_image
```

Two files will be created in the `migrations/` directory:

- `migrations/000x_add_users_profile_image.up.sql`
- `migrations/000x_add_users_profile_image.down.sql`

Edit the `up.sql` file to add the new column:

```sql
ALTER TABLE users
ADD COLUMN profile_image TEXT;
```

Edit the `down.sql` file to remove the column:

```sql
ALTER TABLE users
DROP COLUMN profile_image;
```

The idea is that the `down.sql` file should revert the changes made in the
`up.sql`, letting the database in exactly the same state as before the
migration.

Then you can run or revert the migrations with:

```bash
sqlx migrate run
sqlx migrate revert
```

Or completely reset the database with:

```bash
sqlx database reset --force
```

Once you commit your migrations, you do not need to do anything more to
apply them to production, because they are always checked and executed on
each server start.

## Compiling the queries for offline mode

The idea of sqlx is that plain SQL queries are checked at compile time and
translated into Rust primitives. However, this requires a connection to the
database to check the queries.

This is why offline mode compilation exists. The only thing you need to make
sure is to always run the following before committing your changes:

```bash
cargo sqlx prepare
```

The compiled queries will be saved in the `.sqlx/` directory and should be
committed to the repository. For more details, check the
[Offline mode docs](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query).
