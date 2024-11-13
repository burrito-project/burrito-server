# Using a mocked bus route

No bus driver? No problem! You can start the app with a mocked route
by setting `IS_MOCKED=true` in the env file.

The mocked route will be read from `static/mocks/*.json`. See `mock.rs` for more
details.

Once you have set `IS_MOCKED=true`, you can start the server as usual.
Mocking works by sending `POST /driver` requests to ourselves,
iterating over the mocked route records.

## Why would I use this?

This feature is useful for showcasing the app without having to rely on
a real bus driver, or testing purposes.
