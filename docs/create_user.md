<!-- markdownlint-disable MD041 -->

## Creating a new app user

Currently there is no way to create a new user from the API, you'll need direct database access to
the `internal` schema.

![Alt text](https://cdn.discordapp.com/emojis/1184175117840961606.webp?size=96&quality=lossless)

There is a DB **procedure** `internal.create_user` exactly for this purpose.

```sql
\df internal.create_user
```

Its (current) definition is as follows:

```sql
CREATE OR REPLACE PROCEDURE internal.create_user(
    p_username citext,
    p_display_name varchar(255),
    p_password text,
    p_is_active boolean,
    p_is_staff boolean
)
```

For example, in the local dev container you would do the following:

```console
$ psql 'postgres://admin:dontship@localhost/burrito_app'

burrito_app=# CALL internal.create_user('username', 'Display Name', 'pass123', true, true);
```

### Changing the user password

There is a DB **procedure** `internal.change_password` for changing a user's password.

```sql
\df internal.change_password
```

For example, to change the password of the user created in the previous step:

```console
$ psql 'postgres://admin:dontship@localhost/burrito_app'

burrito_app=# CALL internal.change_password('username', 'newpass123');
```

### Verifying the user

Similarly, there is a DB **function** `internal.get_auth_user` for querying a user by its username and password.

```sql
\df internal.get_auth_user
```

For example, to verify the user created in the previous step:

```console
$ psql 'postgres://admin:dontship@localhost/burrito_app'

burrito_app=# SELECT * FROM internal.get_auth_user('username', 'pass123');
```

This functions is used under the hood on app routes that require authentication.

### Why this even exists?

So Postgres is responsible of hashing the password and storing it securely.
