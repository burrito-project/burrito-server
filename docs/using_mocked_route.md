# Usando una ruta de autobús simulada

¿No hay conductor del burrito? ¡No hay problema! Puedes iniciar la aplicación con una ruta simulada
configurando `IS_MOCKED=true` en el archivo .env.

La ruta simulada se leerá desde `static/mocks/*.json`. Consulta `mock.rs` para más
detalles.

Una vez que configures `IS_MOCKED=true`, puedes iniciar el servidor como de costumbre.
La simulación funciona enviando solicitudes `POST /driver` a nosotros mismos,
iterando sobre los registros de la ruta simulada.

## ¿Por qué debería usar esto?

Esta función es útil para mostrar la aplicación sin tener que depender de
un conductor de autobús real, o para fines de prueba y NO DEBE usarse en producción.
