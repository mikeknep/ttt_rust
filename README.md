# Tic-Tac-Toe in Rust
**Rust version: 0.12**


## Installing Rust

Rust is changing rapidly in pursuit of a 1.0 version, and given significant decisions are still being made about the language, not every release is backwards compatible. Be sure to check the current Rust version and the version on which this code has been confirmed to run (beneath the title, above).

### The simplest way - Homebrew

Run the following commands to update Homebrew and check the latest version of Rust:

```
$ brew update
$ brew info rust
```

If the brew package matches the version above, proceed to install Rust via Homebrew:

```
$ brew install rust
```

### The trickier way - from source

If you prefer to install from source, cannot use Homebrew, or need to download an older version of Rust, visit the [release history page](https://github.com/rust-lang/rust/wiki/Doc-releases) and download the appropriate installation package.

### Confirm success

To confirm a successful installation (via either method), execute the following command:

```
$ rustc -v
```



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

