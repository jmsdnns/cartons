# Cartons

![Animation of a happy milk carton doing a little jig](carton.gif)

🦀 Hello, and welcome to Cartons!

This project is where I store Rust implementations of ideas I build while learning Rust. My hope in sharing them is that they can act as a kind of Rosetta stone for experienced programmers that are curious about Rust.

## Relevant Tools

- [just](https://just.systems/man/en/): Super handy tool to save and run commands. Some of the projects have `.justfile` that shows what commands I use with the project. Especially handy when using containers or VMs.
- [containerd](https://containerd.io/): The best way to manage to containers. It has completely replaced docker for me.
- [limavm](https://lima-vm.io/): My favorite way to manage local VMs during development. I use it in both Linux and macOS.

## The List

Each carton is an independent Rust project. Many of them exist to express common ideas succinctly in Rust. Some of them are significantly more complex and incorporate several of the smaller ideas to achieve something bigger.

* [Feature Demos](feature-demos/): Showcases how to use Rust's core features. There are implementations for iterators, type conversions, using enums, pattern matching, etc. Captures some of Rust's eccentricies nicely.
* [Dates, Times, and Timezones](rw-dates-times/): Simple examples of how to use the Chrono library to create and read various formats of dates and times.
* [Reading & Writing JSON](rw-json/): Using the Serde library to read and write JSON.
* [Config Files](rw-config-files/): Using Serde for toml config files.
* [CLI Tools](cli-tools/): Building CLI tools with the Clap library.
* [Numpy-style Arrays](rusty-numpy/): Numpy users will feel right at home with Rust's ndarray.
* [Micro Macros](micromacros/): Examples of different kinds of macros. Includes the basic foundation for an ORM.
* [Async SSH Pools](async-ssh/): Demonstrates how to run the same command on multiple remote machines simultaneously. Reads a toml config file for information about the remote machines. Includes a .justfile that knows how use Lima VMs for the remotes.
* [Signed Tokens](signed-tokens/): Creates signed JWT tokens and demonstrates how validation of JWT fields is automatic.
* [Async Password Hashing](async-passwords): Password hashing where the actual hashing is done in a background thread to keep the cpu bound work off the main thread.
* [Async Postgres w/ DB Migrations](async-postgres/): Concise example of how to asynchronously query and manage Postgres. Includes a compose file for running Postgres with the project
* [Async REST API](webb/): An authenticated, database backed REST API built with the Axum framework. Includes compose files for Postgres and an app containers.
* [Async gRPC](geerpc/): Uses the Tonic framework for async gRPC.

## Code Comments

It is rare for the code to have comments. At some point a carton will show up that covers how to do comments using Rust's conventions. Until then, it's just raw code.
