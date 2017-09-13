# Cargo testify

[![Build Status](https://travis-ci.org/greyblake/cargo-testify.svg?branch=master)](https://travis-ci.org/greyblake/cargo-testify)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/greyblake/cargo-testify/master/LICENSE)

Automatically runs tests on your Rust project and notifies about the result.

![rust test with notification](https://raw.githubusercontent.com/greyblake/cargo-testify/master/artifacts/demo.gif)


## Install



Install prerequisites (for Debian/Ubuntu):

```
apt-get install -y libdbus-1-dev
```

Install the crate:

```
cargo install cargo-testify
```

## Usage

Run within your Rust project:

```
cargo testify
```

## How does it work?

It watches changes in the project directory, and reacts by running `cargo test`.
The output result is parsed to identify one of the possible outcomes:
* Tests passed
* Tests failed
* Compilation failed

To display notification in the Desktop environment `notify-send` (Linux) or `osascript` (MacOS) commands are used.

## Watched files

* `src/*`
* `tests/*`
* `Cargo.toml`
* `Cargo.lock`
* `build.rs`

## License

[MIT](https://github.com/greyblake/cargo-testify/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [hoodie](https://github.com/hoodie) Hendrik Sollich - thanks for notify-rust crate
- [waywardmonkeys](https://github.com/waywardmonkeys) Bruce Mitchener - fix typos

