# Micro Macros

Several projects using macros to solve small problems that focus on the macro, not the problem being solved.

## Projects

* [Simple ORM](dbmodel/): Some of the metaprogramming building blocks for generating SQL from struct definitions. [ [tests](dbmodel/tests/the_test.rs) ]
* [Iterable Structs](iterable/): A macro that uses Rust's `Any` type to allow iterating across field name-value pairs, similar to what one might do in Python. The Any type essentially erases the actual type information, but reflection can be used to go back from the `Any` to actual types. [ [tests](iterable/tests/the_test.rs) ]

## CLI

Instead of `cargo run`, like the other cartons, each project in this carton uses `cargo test`.

```shell
$ cd micromacros/dbmodel
$ cargo test
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

I recommend reading the tests before the macro, so I put links to the tests after the project description below.

