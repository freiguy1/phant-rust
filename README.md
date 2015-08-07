phant-rust [![Build Status](https://travis-ci.org/freiguy1/phant-rust.svg?branch=master)](https://travis-ci.org/freiguy1/phant-rust)
==========

A library in rust for manipulating data on a [Phant](phant.io) server.  Check out the [generated documentation](http://static.ethanfrei.com/phant/phant) for detailed information.  A more thorough example is provided at `examples/main.rs`.

### Depends on:

- [rust-url](https://github.com/servo/rust-url)

### To use with cargo:

In your project's Cargo.toml, include:
```toml
[dependencies]

phant = "*"
```

To use the library, include the crate with `extern crate phant;` and then use it:

```rust
let mut phant = phant::Phant::new("http://data.sparkfun.com", "your_public_key", "your_private_key");

//         COLUMN NAME        DATA VALUE
phant.add("computer_name",   "my-computer");
phant.add("external_ip",     "123.321.111.222");
phant.add("internal_ip",     "192.168.1.104");

phant.push().ok().expect("Pushing to server did not succeed");
```
