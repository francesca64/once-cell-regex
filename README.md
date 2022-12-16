# once-cell-regex

[![crates.io badge](https://img.shields.io/crates/v/once-cell-regex.svg)](https://crates.io/crates/once-cell-regex)
[![Docs.rs badge](https://docs.rs/once-cell-regex/badge.svg)](https://docs.rs/once-cell-regex)
[![GitHub Actions badge](https://github.com/francesca64/once-cell-regex/actions/workflows/ci.yml/badge.svg)](https://github.com/francesca64/once-cell-regex/actions)

Wow! Super exciting crate! Amazing! 💯💯💯

This crate just gives you the `regex` macro from the `once_cell` docs:
https://docs.rs/once_cell/*/once_cell/index.html#lazily-compiled-regex

I also threw in a `regex_multi_line` macro, since it's a nice thing to have.

## Example

```rust
use once_cell_regex::regex;

fn main() {
    let r = regex!("hello");
    let x = r.is_match("hello world");
    println!("{}", x); // prints "true"
}
```
