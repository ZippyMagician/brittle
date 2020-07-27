# Brittle
[![Crates.io](https://img.shields.io/crates/v/brittle.svg)](https://crates.io/crates/brittle)
[![Documentation](https://docs.rs/brittle/badge.svg)](https://docs.rs/brittle/)

A simple TOML parser written in Rust

## Using Brittle

Get started using this example here:
```rust
extern crate brittle;

use std::io;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("path/to/file.toml")?;
    let buffer = io::BufReader::new(file);

    let mut lines = buffer
        .lines()
        .map(|x| x.unwrap_or("".to_string()))
        .collect::<Vec<String>>();

    let result: Vec<brittle::Parser::TOML> = brittle::parse(lines.join("\n"));
    // Do things with the result here
}
```

## Practicality
This was mainly created as an experiment, there are most likely much better crates available that are more useful. Feel free to use it, however!
The other issue is the fact that there is no documentation. Sorry, maybe I'll consider adding some in the future

## License

Brittle is licensed under the Apache-2.0 license.