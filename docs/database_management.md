# Gestión de la base de datos

Nuestro driver de base de datos, [sqlx](https://github.com/launchbadge/sqlx),
resuelve la mayoría de los problemas relacionados con la gestión de bases de datos.

Si no estás familiarizado con sqlx,
[este video](https://www.youtube.com/watch?v=TCERYbgvbq0)
es un buen punto de partida.

Primero, asegúrate de tener instalado el cliente
[sqlx](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query).

```bash
cargo install sqlx-cli
```

## Creación de migraciones de base de datos

Las migraciones de base de datos se encuentran en el directorio `migrations/`.
Crea una nueva migración para cada cambio que quieras hacer en el esquema de
la base de datos.

Por ejemplo, si necesitas añadir una nueva columna `profile_image` a la tabla
`users`, puedes crear una migración con el siguiente comando:

```bash
sqlx migrate add -rs add_users_profile_image
```

Esto generará dos archivos en el directorio `migrations/`:

- `migrations/000x_add_users_profile_image.up.sql`
- `migrations/000x_add_users_profile_image.down.sql`

Edita el archivo `up.sql` para añadir la nueva columna:

```sql
ALTER TABLE users
ADD COLUMN profile_image TEXT;
```

Luego, edita el archivo `down.sql` para revertir la migración:

```sql
ALTER TABLE users
DROP COLUMN profile_image;
```

La idea es que el archivo `down.sql` revierta los cambios realizados en `up.sql`,
dejando la base de datos exactamente en el mismo estado que antes de aplicar
la migración.

Después de eso, puedes ejecutar o revertir las migraciones con:

```bash
sqlx migrate run
sqlx migrate revert
```

O reiniciar completamente la base de datos con:

```bash
sqlx database reset --force
```

Una vez que hagas commit a tus migraciones, no necesitas hacer nada más
para aplicarlas en producción, ya que siempre se verifican y ejecutan
automáticamente al iniciar el servidor.

## Compilación de consultas en "offline mode"

La idea de sqlx es que las consultas SQL planas se verifican en tiempo de
compilación y se traducen a tipos primitivos de Rust. Sin embargo, esto requiere
una conexión a la base de datos para verificar las consultas.

Por esta razón existe la compilación en "offline mode". Lo único que necesitas
tener en cuenta aquí es que cada vez que hagas un commit ejecutes lo siguiente:

```bash
cargo sqlx prepare
```

Las consultas compiladas se guardarán en el directorio `.sqlx/` y deben incluirse en el repositorio. Para más detalles, consulta la
[documentación sobre el offline mode de sqlx](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query).
