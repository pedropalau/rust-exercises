# Rust programming language exercises

This repository contains some exercises implementations and code exploration for the `rust` language.

## Executing the exercises

For executing the exercises, is required to have installed [rust](https://www.rust-lang.org/tools/install) on your system.


### Installing Rust on macOS, Linux or another Unix-like OS

```
curl https://sh.rustup.rs -sSf | sh
```

To execute an exercise you can use the command:

```
$ rustc [filename].rs
```

`rustc` will produce a binary that can be executed:

```
$ ./filename
```


### Using Cargo

Cargo is Rust’s build system and package manager.

#### Building and running a Cargo project

From the main directory, you can build the project by entering the following command:

```
$ cargo build
```

This command creates an executable file in `target/debug/[executable]`.

You can run the executable with this command:

```
$ ./target/debug/[executable]
```

You can also use `cargo run` to compile the code and then run the resulting executable all in one command:

```
$ cargo run --bin [executable]
```
