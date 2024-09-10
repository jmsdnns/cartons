# Using Config Files

Using the [Serde](https://docs.rs/serde/latest/serde/) library to read TOML config files.

## Example Configs

Config 1 does not have the `maybe_matrix` field.

```toml
a_list = ["woof", "woof", "WOOF"]
a_string = "raaaarrrrrrr"
a_int = 1728
a_float = 3.14
maybe_string = "string was here"
maybe_ints = [3, 42, 180, 360]
```
Config 2 does not have `maybe_string` or `maybe_ints`. It also has a field not present in the Rust struct, `maybe`.

```toml
a_list = ["meow", "meow", "MEOW"]
a_string = "weeeee"
a_int = 42
a_float = 2.78
maybe = "AWW YEAH"
maybe_matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]
```

## Output

```shell
$ cargo run
...
CONFIG: config1.toml
- a list: ["woof", "woof", "WOOF"]
- a string: raaaarrrrrrr
- a int: 1728
- a float: 3.14
- maybe string: "string was here"
- maybe ints: [3, 42, 180, 360]
- maybe matrix: [[]]
 
CONVERT:
- maybe string: STRING WAS HERE
- maybe ints: 3, 42, 180, 360
- maybe matrix: none
 
CONFIG: config2.toml
- a list: ["meow", "meow", "MEOW"]
- a string: weeeee
- a int: 42
- a float: 2.78
- maybe string: ""
- maybe ints: []
- maybe matrix: [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]
 
CONVERT:
- maybe string: none
- maybe ints: none
- maybe matrix: [["1", "2", "3", "4"], ["5", "6", "7", "8"], ["9", "10", "11", "12"]]
```
```
