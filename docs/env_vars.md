<!-- markdownlint-disable MD033 MD045 -->

# Environment variables

The `.env.example` shows an up-to-date list of all environment variables that
the server uses. Start by copying the example file to `.env`:

```bash
cp .env.example .env
```

All the variables defined here directly map an static, global Rust variable
defined in the `src/env.rs` file, so you can use them in your code like this:

```rust
fn main() {
  let port: u16 = *crate::env::PORT;
  println!("Server running on port {}", port);
}
```

## Examples

A ready-to-go .env file for development would look like this:

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

A .env file for production would look like this:

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
The above files does not contain any valid credentials and just
demonstrate how a real .env file would look like in both cases.
</div>

## Uploading the production variables to GitHub

If you are using the GitHub CI pipeline, make sure to set the variables in the
GitHub repository settings. You can find them in the `Settings` tab, under
`Secrets and Variables > Actions` section.

<img src="./static/repo_secrets.png"></img>

<img src="./static/repo_variables.png"></img>

Secrets description:

- `ENV_FILE`: An exact copy of the production .env file content.
- `SSH_KEY`: The private SSH key that will be used to connect to the server.
  This assumes the public key is already in the server's `authorized_keys` file.

Variables description:

- `SSH_HOST`: The host where the server is running.
- `SSH_USERNAME`: The user that will be used to connect to the server.
- `SSH_REPO_PATH`: The path where the repository is already cloned in the server.
