# CLI Tools

Uses the [Clap](https://docs.rs/clap/latest/clap/) crate for building input structures for CLI tools.

## Running CLI Commands

There are two ways. One is via `cargo` and the other is to call the binary directly. I prefer running the binary directly as it mimics the experience a user would have.

* w/ cargo: `$ cargo run --bin <the command>`
* directly: `$ ./target/debug/<the command>`

This Carton is configured to create a binary for each example. That is configured with each `[[bin]]` in the Carton's [Cargo.toml](Cargo.toml). The source files use underscores, but the commands follow Rust conventions by using hyphens.

The following toml tells Cargo to create a binary named `kittyctl` from a source file called `src/sub_cmds.rs`

```toml
[[bin]]
name = "kittyctl"
path = "src/sub_cmds.rs"
```

## Projects

* kittyctl: A tool that demonstrates how to use Clap for sub commands, each with their own inputs. Creates a binary called `kittyctl` with subcommands that help with kitty management. [ [source code](src/sub_cmds.rs) ]

### kittyctl

Help output for the main command:

```shell
> ./target/debug/kittyctl -h
A CLI for managing kitties

Usage: kittyctl <COMMAND>

Commands:
  pet   Pet a kitty
  feed  Feed the kitties
  call  Call a kitty over
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Help output for the `pet`` subcommand:

```shell
> ./target/debug/kittyctl pet -h
Pet a kitty

Usage: kittyctl pet <NAME>

Arguments:
  <NAME>  The kitty to pet

Options:
  -h, --help  Print help
```

Executing the pet subcommand:

```shell
> ./target/debug/kittyctl pet sierra
Petting sierra
```

