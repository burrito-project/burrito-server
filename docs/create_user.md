<!-- markdownlint-disable MD033 MD041 -->

# Creando un nuevo usuario de la aplicación

Actualmente no hay forma de crear un nuevo usuario desde la API, necesitarás
acceso directo a la base de datos al esquema `internal`.

<div class="warning">
Todas las funciones y procedimientos mostrados en este capítulo fueron
definidos en la migración "migrations/0006_users.up.sql"

Recordar que la aplicación no depende de usuarios para funcionar. Actualmente
los usuarios son solo usados para propósitos administrativos.
</div>

Existe un **procedure** llamado `internal.create_user` específicamente para este propósito.

```sql
\df internal.create_user
```

Su definición (actual) es la siguiente:

```sql
CREATE OR REPLACE PROCEDURE internal.create_user(
    p_username citext,
    p_display_name varchar(255),
    p_password text,
    p_is_active boolean,
    p_is_staff boolean
)
```

Por ejemplo, en el contenedor de desarrollo local harías lo siguiente:

```console
$ psql 'postgres://admin:dontship@localhost/burrito_app'

burrito_app=# CALL internal.create_user('username', 'Display Name', 'pass123', true, true);
```

## Cambiando la contraseña del usuario

Existe un **procedure** llamado `internal.change_password` para cambiar la
contraseña de un usuario.

```sql
\df internal.change_password
```

Por ejemplo, para cambiar la contraseña del usuario creado en el paso anterior:

```console
$ psql 'postgres://admin:dontship@localhost/burrito_app'

burrito_app=# CALL internal.change_password('username', 'newpass123');
```

## Verifying the user

De manera similar, existe una **function** llamada `internal.get_auth_user`
para consultar un usuario por su nombre de usuario y contraseña.

```sql
\df internal.get_auth_user
```

Por ejemplo, para verificar el usuario creado en el paso anterior:

```console
$ psql 'postgres://admin:dontship@localhost/burrito_app'

burrito_app=# SELECT * FROM internal.get_auth_user('username', 'pass123');
```

Esta función se utiliza internamente en las rutas de la aplicación que
requieren autenticación.

## ¿Por qué hacerlo de esta forma?

De esta forma, Postgres es responsable de hashear la contraseña y almacenarla
de forma segura. El servidor no es responsable de hashear ni verificar nada.

Si se desean exponer endpoints de registro/login de usuarios, se tendrían
que llamar estas funciones desde el backend. Puedes ver un ejemplo de esto en
`src/features/auth/handlers/login.rs`.
