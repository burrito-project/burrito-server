<!-- markdownlint-disable MD033 MD045 MD014 -->

# Despliegue del servidor

El repositorio del servidor contiene todos los componentes necesarios para
configurar fácilmente la integración y el despliegue continuos utilizando
solo un VPS y GitHub Actions.

## Configuración del VPS

La configuración inicial del VPS debería ser sencilla.
Asumiendo que estás en un servidor Debian bookworm con privilegios de root:

### Dependencias

- Instala Docker y Docker Compose siguiendo la guía oficial:
  <https://docs.docker.com/engine/install/debian/>.

### Autenticación SSH

- Crea un nuevo usuario que utilizarás para desplegar el servidor. Nómbralo
  como desees.

  ```console
  # adduser burrito
  # usermod -aG sudo burrito
  ```

- Genera un par de claves SSH (o sube una existente) para el nuevo usuario

  ```console
  # su burrito
  $ ssh-keygen -t ed25519 -C "admin@burrito" -f ~/.ssh/id_ed25519
  ```

- Copia la clave pública en el archivo `authorized_keys` del servidor

  ```console
  $ cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys
  ```

- Prueba la conexión al servidor

  ```console
  $ ssh burrito@localhost
  ```

### Configuración del repositorio

- Ahora clona el repositorio del servidor

  ```console
  $ sudo mkdir -p /opt/burrito/repo
  $ git clone git@github.com:burrito/server.git /opt/burrito/repo
  ```

- Si estás utilizando el pipeline de CI de GitHub, configura las variables
  en los ajustes del repositorio de GitHub y estarás listo. Consulta
  [Subiendo las variables de producción a GitHub]((./env_vars.md#uploading-the-production-variables-to-github)).

  De lo contrario, rellena manualmente un archivo `.env`.

### Certificados TLS

- Lee cuidadosamente el archivo `docker/prod/nginx/nginx.conf`, que contiene la
  configuración del servidor y las rutas a los certificados TLS.

- Sigue las instrucciones de tu CA (por ejemplo, Let's Encrypt) para generar
  los certificados. Probablemente necesitarás crear un nuevo registro DNS
  apuntando a la dirección IP del servidor o un registro TXT de desafío.

- Genera un par de certificados TLS usando Let's Encrypt u otra CA.
  Por ejemplo, utilizando Certbot:

  ```console
  # certbot certonly --standalone -d api.contigosanmarcos.com
  ```

- Cualquiera que sea el dominio que hayas utilizado, asegúrate de actualizar
  la variable `DOMAIN_NAME` en el archivo `docker-compose.prod.yml`.

- Si deseas guardar los certificados en una ruta personalizada
  (como /opt/burrito/certs), asegúrate de actualizar el archivo
  `docker/prod/nginx/nginx.conf`.

- Usa un verificador de [propagación DNS](https://www.whatsmydns.net/) para
  comprobar si tu dominio apunta
  correctamente a la dirección IP del servidor.

### Iniciar el contenedor

Si estás utilizando GH Actions, no necesitas correr estos comandos.
En su lugar, inicia el action de deploy.

- Puedes levantar el servidor utilizando Docker Compose

  ```console
  $ cd /opt/burrito/repo
  $ docker compose -f docker-compose.prod.yml up --build
  ```

- Cada vez que desees actualizar el servidor, simplemente descarga los cambios
  y reinicia el servidor.

  ```console
  $ cd /opt/burrito/repo
  $ git pull origin main
  $ docker compose -f docker-compose.prod.yml up --build
  ```

  Aunque se recomienda usar el pipeline de GitHub Actions en lugar de
  lanzar comandos manuales.

## Production checks

Antes de desplegar en producción, asegúrate de cumplir con las siguientes verificaciones:

- [ ] Has configurado correctamente las [variables de entorno requeridas](./env_vars.md).
- [ ] Has [compilado las consultas para modo offline](./database_management.md#compiling-the-queries-for-offline-mode) ejecutando

  ```console
  cargo sqlx prepare
  ```

- [ ] Las [database migrations](./database_management.md#creating-database-migrations)
  están actualizadas y han sido probadas localmente.
- [ ] Te has asegurado de que el servidor compila y se ejecuta sin errores localmente.
