# Burrito server

```bash
cargo run
```

## Mocking routes for showcasing purposes

No bus driver working today? No problem! You can mock the routes by setting `IS_MOCKED=true` in
the env file. The mocked route will be read from `static/mocks/*.json`. See `mock.rs` for more
details.

Once you have set `IS_MOCKED=true`, you can start the server as usual. Mocking works by sending
`POST /status` requests to ourselves, iterating over the mocked route records.
