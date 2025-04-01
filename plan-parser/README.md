# Plan Parser

Uses the [Winnow](https://docs.rs/winnow/latest/winnow/) crate for building input structures for CLI tools.

## Running It

```shell
cargo run -- commands.txt
```

You'll get output that looks like this:

```
Successfully parsed 5 commands:
  1: Execute: cat meow
  2: Upload: local=meow.cat, remote=/tmp/runaway.cat
  3: Execute: echo "kitty? where are you?"
  4: Download: remote=/tmp/runaway.cat, local=happy.cat
  5: Execute: cat meow
``` 

