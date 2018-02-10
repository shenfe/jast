# jast
 
**>> 0.1% development, not available <<**

Jast is a JavaScript interpreter/compiler to Rust.

## Supported Grammars

[ECMAScript-262](https://www.ecma-international.org/ecma-262/8.0/index.html).

## Environment

### rust

To develop, you have to install **rust**. To install rust (and cargo, the rust package manager), refer to the [official](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html), or just:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

### Node.js

To test and compare, you have to install **Node.js**. To install Node.js, refer to the [official](https://nodejs.org/zh-cn/download/).

## Commands

### compile

```bash
$ cargo build
```

### run

```bash
$ ./target/debug/jast
```

### compile and run in one step

```bash
$ cargo run
```

## Related Work

### parser

* [pest](https://crates.io/crates/pest)
* [peg](https://crates.io/crates/peg)
