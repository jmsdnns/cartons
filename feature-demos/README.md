# Demos of Rust Features

The [official Rust book](https://doc.rust-lang.org/stable/book/) was a great start for learning Rust. I also learned a lot by implementing solutions to the questions I had while reading.

- [Type Conversion](src/conversion.rs): A significant part of writing Rust is properly converting data from one type to another. The Rust way to do this is by implementing the `From` trait.
- [Enums](src/enums.rs): Using enums to ensure complete handling of possible values & simple pattern matching.
- [Pattern Matching](src/matching.rs): Rust makes it easy to extract data from elaborate, nested data structures.
- [Iterators](src/iterators.rs): Iterators don't manage state by unless we implement state handling ourselves. This shows what that looks like.
- [Trait Inheritance](src/inheritance.rs): An example of how traits can be mixed and matched for complex types without the complexity found in object oriented inheritance.

## CLI

This project uses Cargo's `[[bin]]` directive in its [`Cargo.toml`](Cargo.toml) to make binaries from source files not named `main.rs`. There is an `inheritance.rs` and you can run it with this flag: `--bin inheritance`.

That looks like this:

```shell
$ cargo build
$ cargo run --bin inheritance
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/inheritance`
an amazing beat is played
falco is ripping those drums
a heavy guitar riff is played
an amazing beat is played
DaveGrohl is floating
```

