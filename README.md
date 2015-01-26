# Tic-Tac-Toe in Rust
**Rust version: 0.12**


## Installing Rust

This code was originally written using Rust 0.11.
It has since been updated to version 0.12, but *has not yet been updated to Rust 1.0*.
To install the correct version of Rust to run this code, visit the [release history page](https://github.com/rust-lang/rust/wiki/Doc-releases#0120) and download the appropriate installation package.
I'd like to update this codebase to Rust 1.0, and have a WIP branch attempting to do so, but there were several significant changes to the language and it's not particularly high on my list of priorities, so don't hold your breath.


## Compiling and executing

Rust 0.12 ships with a compiler, `rustc`, that compiles Rust code into executable binaries.
It does *not* ship with [Cargo](https://crates.io/), the as-of-1.0 official package manager for Rust.
Again, I'd like to update this codebase to use Cargo, but for now there is a custom Makefile with targets to compile, execute, and delete both test and production executables.

### Running the tests

To run the tests, execute the following command from the project's root directory:

```
$ make test
```

(Note: `test` is the default Makefile target, so you can also just run `make`.)


### Playing the game

To start a game of Tic-Tac-Toe, execute the following command from the project's root directory:

```
$ make play
```

Follow the opening instructions in console to set up the players.

When it is a human's turn to play, you may place your token on the board by entering the index of your chosen cell. The board is zero-indexed, and ascends left-to-right, top-to-bottom (like reading English, example below).

0 | 1 | 2

3 | 4 | 5

6 | 7 | 8

