<!-- markdownlint-disable MD033 MD045 -->

# Variables de entorno

El archivo `.env.example` muestra una lista actualizada de todas las variables de entorno que utiliza el servidor. Comienza copiando el archivo de ejemplo a `.env`:

```bash
cp .env.example .env
```

Todas las variables definidas aquí se corresponden directamente con una variable global estática de Rust definida en el archivo `src/env.rs`, por lo que puedes utilizarlas en tu código de esta forma:

```rust
fn main() {
  let port: u16 = *crate::env::PORT;
  println!("Server running on port {}", port);
}
```

## Ejemplos

Un archivo .env listo para usar en desarrollo se vería así:

```bash
IS_MOCKED=false

ROOT_SECRET="root_secret"
AUTH_DRIVER_PASSPHRASE="driver_pass"
AUTH_WHATSAPP_ACCESS_TOKEN="none"

POSTGRES_PASSWORD="dontship"
DATABASE_URL="postgres://admin:${POSTGRES_PASSWORD}@localhost/burrito_app"

CLOUDINARY_API_KEY="438453545385499"
CLOUDINARY_CLOUD_NAME="sea1jk51z"
CLOUDINARY_API_SECRET="mJd3bbkWa5mPVKuNBgCLxjY5FSM"
```

Un archivo .env para producción se vería así:

```bash
IS_MOCKED=false

ROOT_SECRET="burrito_prod_6z3g5z2t5z2g5Z2t5g3X"
AUTH_DRIVER_PASSPHRASE="burrito_prod_K4ZVf3g1zS6x2TcjdyDztkbvh4CQHrF6"

# Leave it empty ("") if you don't plan to use the WhatsApp API
AUTH_WHATSAPP_ACCESS_TOKEN="EAAjnKUIiz4ABOzMXloXZCVvifdfFHJGHvlFFWENYE1zFyfg0Ikh0ExDWnkTO1q9CllVXQgKZBvrD3XUucr6Bxk9RIZAITIvzAxWZB2KbZApppIbSwsk2Ozu54emMqb6QlpBRrUM7WAvrRWa8ZApj5p4ZBY9ROIcHKI6CXujoAg1Q1jnv7pJCnVeLDUblAND97J7Q5LliGPZCdiZAHKI16boABdPHo6p2mm8lFCIYZD"

POSTGRES_PASSWORD="MM3ky4RhgpFSbfoXmUh42r0REZzCYXyu"
DATABASE_URL="postgres://admin:${POSTGRES_PASSWORD}@burrito_db/burrito_app"

CLOUDINARY_API_KEY="438453545385499"
CLOUDINARY_CLOUD_NAME="sea1jk51z"
CLOUDINARY_API_SECRET="mJd3bbkWa5mPVKuNBgCLxjY5FSM"
```

<div class="warning">
Los archivos anteriores no contienen credenciales válidas y solo
demuestran cómo se vería un archivo .env real en ambos casos.
</div>

## Subiendo las variables de producción a GitHub

Si estás utilizando el pipeline de GitHub CI, asegúrate de configurar las variables en
la configuración del repositorio de GitHub. Puedes encontrarlas en la pestaña `Settings`, bajo
`Secrets and Variables > Actions`.

<img src="./static/repo_secrets.png"></img>

<img src="./static/repo_variables.png"></img>

Descripción de los secretos:

- `ENV_FILE`: Una copia exacta del contenido del archivo .env de producción.
- `SSH_KEY`: La clave SSH privada que se utilizará para conectarse al servidor.
  Esto asume que la clave pública ya está en el archivo `authorized_keys` del servidor.

Descripción de las variables:

- `SSH_HOST`: El host donde se ejecuta el servidor.
- `SSH_USERNAME`: El usuario que se utilizará para conectarse al servidor.
- `SSH_REPO_PATH`: La ruta donde ya está clonado el repositorio en el servidor.
