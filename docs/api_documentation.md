# API documentation

For the API documentation we followed a code-first approach. The documentation
is generated from the source code using [`utoipa`](https://github.com/juhaku/utoipa),
which generates an OpenAPI 3.1 specification file at compile time.

This specification is then consumed and rendered by
[Scalar](https://github.com/scalar/scalar). The final document result is served
by the API itself, and can be publicly accessed at `/docs`.

![API Documentation](./static/api_docs.png)

Live documentation is hosted in <https://api.contigosanmarcos.com/docs>.
