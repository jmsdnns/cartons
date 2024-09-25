# Micro Macros

There were several projects here. Now just one. That will change soon enough.

## Projects

* [Vek](vek/): Using declarative macros to reimplement parts of the builtin `vec!` [ [tests](tests/the_test.rs) ]
* [Iterable Structs](iterable/): A macro that uses Rust's `Any` type to allow iterating across field name-value pairs, similar to what one might do in Python. The Any type essentially erases the actual type information, but reflection can be used to go back from the `Any` to actual types. [ [tests](iterable/tests/the_test.rs) ]

## CLI

Instead of `cargo run`, like the other cartons, each project in this carton uses `cargo test`.

```shell
$ cd micromacros/iterable
$ cargo test
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

