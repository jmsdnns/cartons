# Full Rest API

A REST API build with the [Axum web framework](https://crates.io/crates/axum). It provides endpoints for creating an account, logging in, and an endpoint that requires authentication for access. Auth is built with JWT Tokens. It uses [SQLx](https://crates.io/crates/sqlx) with Postgres for storage and database migrations. It also provides containers for both Postgres and the app to simplify running it locally.

## Two Justfiles

One `.justfile` exists at the Carton root and one exists inside the app directory. The root justfile makes it easy to turn on all the infrastructure, like postgres. The one in the app is used for curl commands that create accounts and access authenticated URLs. After making the login call, copy the returned Bearer token into the app justfile so auth will work with your setup.

## CLI

Turn on the infrastructure first. _If you're on a mac, turn on a VM for your containers, eg. `limactl start default`._

```shell
$ cd cartons/webb/
$ just infra-up
```

Then turn on the app:

```shell
$ cd app
$ cargo run
```
Open a second terminal and go to the same app dir:

```shell
$ cd cartons/web/app
$ just mkaccount
$ just login
```

The login call will print a token as part of its output. Copy that into `cartons/web/app/.justfile`. You'll see where right at the top. Then verify it with the following command, which accesses an authenticated URL.
```
$ just landing
```

Once you're done, hit ctrl-c on the app and do the following:

```shell
$ cd ..
$ just infra-down
```
