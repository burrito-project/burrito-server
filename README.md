# Burrito server

```bash
cargo run
```

## Management

```bash
cargo install sqlx-cli

# Reset the database and migrations
sqlx database reset --force
```

## App updating protocol

The GET `/pending_updates?version=1.0.0` endpoint will return a list of application versions that
are newer than the one provided in the query parameter.

The client MUST NOT let the user proceed with the application if some version is marked as
`is_mandatory`. If the client decides to, it can show a dialog to the user with the changelog and
the option to update, storing the acknowledgement in the local storage.

An example of the workflow would be:

```txt
Act 1: the first time

>client fetches /pending_updates?version=1.0.0
>server returns two pending version, where neither is mandatory
>two options, [Update now] and [Later] are shown to the user along with the changelogs
>user acknowledges
>client stores the highest one as "latest_acknowledged_version" in local storage
>user decides not to update

Act 2: next day, next update

>client fetches /pending_updates?version=1.0.0
>now server returns three versions, still none mandatory
>since one of them is newer than "latest_acknowledged_version", client shows the dialog
>user acknowledges
>client stores the highest one as "latest_acknowledged_version" in local storage
>user decides not to update

Act 3: the urgent update

>the client fetches /pending_updates?version=1.0.0
>now server returns four versions, where the last one (2.0.0) is mandatory
>client merges the changelogs and shows them to the user along with the [Update now] button
>user acknowledges and it only option is to update
>client stores the highest one as "latest_acknowledged_version" in local storage
>user updates

Act 4: the calm after the storm

>client fetches /pending_updates?version=2.0.0
>server returns an empty list
>client proceeds with the app
```

## Mocking routes for showcasing purposes

No bus driver working today? No problem! You can mock the routes by setting `IS_MOCKED=true` in
the env file. The mocked route will be read from `static/mocks/*.json`. See `mock.rs` for more
details.

Once you have set `IS_MOCKED=true`, you can start the server as usual. Mocking works by sending
`POST /driver` requests to ourselves, iterating over the mocked route records.
