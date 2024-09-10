# Cartons

![Animation of a happy milk carton doing a little jig](carton.gif)

🦀 Hello, and welcome to Cartons!

This project is where I store Rust implementations of ideas I build while learning Rust. My hope in sharing them is that they can act as a kind of Rosetta stone for experienced programmers that are curious about Rust.

## Relevant Tools

- [just](https://just.systems/man/en/): Super handy tool to save and run commands. Some of the projects have `.justfile` that shows what commands I use with the project. Especially handy when using containers or VMs.
- [containerd](https://containerd.io/): The best way to manage to containers. It has completely replaced docker for me.
- [limavm](https://lima-vm.io/): My favorite way to manage local VMs during development. I use it in both Linux and macOS.

## Projects

Each carton is an independent Rust project. Many of them exist to express common ideas succinctly in Rust. Some of them are significantly more complex and incorporate several of the smaller ideas to achieve something bigger.

Rust's async story is an evolving one. So far, I am most fond of using the [Tokio](https://tokio.rs/) library. Each async carton below uses it.

The Cartons:

1. [Feature Demos](feature-demos/): Showcases how to use Rust's core features. There are implementations for iterators, type conversions, using enums, pattern matching, etc. It would be a decent place to start for anyone who has read the Rust book but haven't written much code yet.
2. [Dates, Times, and Timezones](rw-dates-times/): Simple examples of using the [Chrono](https://docs.rs/chrono/latest/chrono/) library to create and read various formats of dates and times.
3. [Reading & Writing JSON](rw-json/): Using the [Serde](https://docs.rs/serde/latest/serde/) library to read and write JSON.
4. [Config Files](rw-config-files/): Using Serde for toml config files.
5. [Numpy-style Arrays](rusty-numpy/): Numpy users will feel right at home with Rust's [ndarray](https://docs.rs/ndarray/latest/ndarray/) library.
6. [Micro Macros](micromacros/): Examples of different kinds of macros. Includes the basic foundation for an ORM.
7. [Async SSH Pools](async-ssh/): Demonstrates how to run the same command on multiple remote machines simultaneously. Reads a toml config file for information about the remote machines. Includes a .justfile that knows how use Lima VMs for the remotes.
8. [Async Password Hashing](async-passwords): Password hashing where the actual hashing is done in a background thread to keep the cpu bound work off the main thread.
9. [Async Postgres w/ DB Migrations](async-postgres/): Concise example of how to use SQLx to query and manage Postgres. Includes a docker compose file for running Postgres in a container.
10. [Full REST API](webb/): A REST API build with the [Axum web framework](https://docs.rs/axum/latest/axum/). It provides endpoints for creating an account, logging in, and an endpoint that requires authentication for access. Auth is built with [JWT Tokens](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/). It uses [SQLx](https://docs.rs/sqlx/latest/sqlx/) with Postgres for storage and database migrations. It also provides containers for both Postgres and the app. [Axum Extractors](https://docs.rs/axum/latest/axum/extract/index.html) are for reading JSON payloads and for extracting auth tokens from headers.
11. [Async gRPC](geerpc/): Uses the [Tonic](https://docs.rs/tonic/latest/tonic/) library for async gRPC.

## Code Comments

It is rare for the code to have comments. At some point a carton will show up that covers how to do comments using Rust's conventions. Until then, it's just raw code.
