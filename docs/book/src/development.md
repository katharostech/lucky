# Development

## Building Lucky

Lucky is written in Rust and can be built with cargo by running

    cargo build --release

The resulting binary named `lucky` will be in `target/release`. 

## Building Documentation

The documentation is built with [mdbook](https://github.com/rust-lang/mdBook/). The documentation is located in the `docs/book` folder. To host the docs locally you can navigate to the `docs/book` dir and run `mdbook serve`.

### CLI Documentation

In addition to the book content in `docs/book`, the CLI documentation is automatically built by Lucky and can be built by running the doc generator in the root of the project:

    cargo run --features doc-gen docs/book/src/

> **Warning** The `SUMMARY.md` file in `docs/book/src` will be automatically updated with the CLI documentation links when the doc generator is run. Do not put any links after the `Lucky CLI` link section or it will be overwritten.
