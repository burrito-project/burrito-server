<!-- markdownlint-disable MD033 MD045 -->

# Desarrollo en el proyecto Burrito

## Ejecutando el servidor para desarrollo

Debido a que [sqlx](https://github.com/launchbadge/sqlx) verifica las consultas en tiempo de compilación, la base de datos de desarrollo debe estar en funcionamiento.  
La forma más sencilla es ejecutar el contenedor de la base de datos de desarrollo con:

```bash
# Ver docker-compose.yml
docker compose up --build
```

Una vez que tengas la base de datos en funcionamiento, inicia el servidor con:

```bash
cargo run
```

<div class="warning">
El flujo de trabajo de producción es completamente diferente de este. Por favor, consulta la
sección de despliegue para más información.
</div>

Asegúrate de verificar que las variables de tu archivo `.env` y las variables de Docker estén configuradas correctamente.

## Configuración de VSCode

Es posible que desees agregar las siguientes extensiones:

- Rust
- rust-analyzer
- Dependi
- PostgreSQL Language Server

Y estas configuraciones pueden ser útiles:

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

## Gestión local de la base de datos

Consulta [database management](./database_management.md) para aprender cómo gestionar tu base de datos localmente, crear y revertir migraciones.

## Simulación de rutas para propósitos de demostración

¿No hay conductor trabajando hoy? ¡No hay problema! Puedes simular las rutas configurando `IS_MOCKED=true` en el archivo `.env`. Las rutas simuladas serán leídas desde `static/mocks/*.json`. Consulta `mock.rs` para más detalles.

Una vez que configures `IS_MOCKED=true`, puedes iniciar el servidor como de costumbre. La simulación funciona enviando solicitudes `POST /driver` a nosotros mismos, iterando sobre los registros de rutas simuladas.
