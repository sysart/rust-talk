# Sysart Tech Talk: Rust

Material for Sysart internal tech talk held in Finnish on 2021-06-02.

## Getting started with Rust

* [Installing Rust](https://www.rust-lang.org/tools/install)
* VSCode plugins [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
* [Rust Book](https://doc.rust-lang.org/stable/book/), the first chapters provide a good introduction to the language
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Rustlings](https://github.com/rust-lang/rustlings) contains small exercises to get you used to reading and writing Rust code
* [Rust and WebAssembly](https://rustwasm.github.io/book/) is an approachable book for getting into writing wasm

## Slides

[Slides](https://github.com/mattikl/rust-talk/blob/master/slides/slides.md) are in Finnish.

Start a spectacle presentation at http://localhost:3000/ with

```text
cd slides
npx spectacle-cli -s slides.md
```

## Simple example

Uses `unwrap` for error handling.

```text
$ cd code-simple
$ cargo run
```

## Refactored example

Error handling refactored to using a custom error type. [This blog post](https://blog.burntsushi.net/rust-error-handling) provides a detailed description of the approach.

```text
$ cd code-refactored
$ cargo run
```
