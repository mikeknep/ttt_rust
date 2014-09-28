# Tic-Tac-Toe in Rust
**or, _rusTTT_**


## Installing Rust

Rust is changing rapidly--a new 0.x version has been released every three months quite regularly for the past couple years. This code was written using Rust 0.11, but history suggests 0.12 is just around the corner. Given Rust has not yet reached a 1.0 version and significant decisions are still being made about the language, not every release is backwards-compatible. Therefore, it is important you install the correct version (0.11).

### The simplest way - Homebrew

Run the following commands to update Homebrew and check the latest version of Rust:

```
$ brew update
$ brew info rust
```

If you see "rust: stable 0.11.0, HEAD", proceed to install Rust via Homebrew:

```
$ brew install rust
```

### The trickier way - from source

If you prefer to install from source, cannot use Homebrew, or Homebrew has updated to Rust 0.12, visit the [release history page](https://github.com/rust-lang/rust/wiki/Doc-releases#0110) and download the appropriate 0.11.0 installation package for your system.

### Confirm success

To confirm a successful installation (via either method), execute the following command:

```
$ rustc -v
```

This should print "rustc 0.11.0".



## Compiling and executing

Rust ships with a compiler, `rustc`, that compiles Rust code into executable binaries. For convenience, rather than use `rustc` directly, I've added a custom Makefile with targets to compile, execute, and delete both test and production executables.

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

