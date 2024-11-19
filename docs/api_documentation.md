# Documentación de la API

Para la documentación de la API seguimos un enfoque de código primero. La documentación
se genera a partir del código fuente utilizando [`utoipa`](https://github.com/juhaku/utoipa),
que genera un archivo de especificación OpenAPI 3.1 en tiempo de compilación.

Esta especificación es luego consumida y renderizada por
[Scalar](https://github.com/scalar/scalar). El resultado final del documento es servido
por la propia API y puede ser accedido públicamente en `/docs`.

![Documentación de la API](./static/api_docs.png)

Puedes consultar la documentación online en <https://api.contigosanmarcos.com/docs>.

## Documentar los endpoints de la API

Para documentar una ruta se utiliza la macro `path` del atributo de `utoipa`.
Esta macro toma varios argumentos para documentar el endpoint. Los más
importantes se muestran en este ejemplo de `PATCH /flag/id`:

```rust,no_run,noplayground
#[utoipa::path(
    tag = docs::tags::APP_VERSIONS_TAG,
    description = "Edits an existing app version. All columns are optional.",
    request_body(content = schemas::AppVersionPatchPayload),
    responses(
        (status = 200, body = schemas::AppVersion),
        (status = 400),
        (status = 401),
    ),
    security(("staff_user_auth" = [])),
)]
#[patch("/<id>", format = "json", data = "<payload>")]
async fn patch_app_version(
    id: i32,
    _user: StaffUser,
    payload: JsonResult<'_, schemas::AppVersionPatchPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::AppVersion>> {
    // ...
}
```

Consulta la documentación de [utoipa::path](https://docs.rs/utoipa/latest/utoipa/attr.path.html)
para más detalles.
