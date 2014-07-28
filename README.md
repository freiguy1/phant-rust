phant-rust [![Build Status](https://travis-ci.org/freiguy1/phant-rust.svg?branch=master)](https://travis-ci.org/freiguy1/phant-rust)
==========

A library in rust for manipulating data on a [Phant](phant.io) server.  Check out the [generated documentation](http://www.rust-ci.org/freiguy1/phant-rust/doc/phant/) for detailed information.

### To use with cargo:

In your project's Cargo.toml, include:
```
[dependencies.phant]

git = "https://github.com/freiguy1/phant-rust.git"
```

To use the library, include the crate with `extern crate phant;` and then use it:

```
let mut phant = phant::Phant::new("data.sparkfun.com", "123abc", "456def");

phant.add("computer_name", "my-computer");
phant.add("external_ip", "123.321.111.222");
phant.add("internal_ip", "192.168.1.104");

phant.push().ok().expect("Pushing to server did not succeed");
```
