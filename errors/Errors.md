# Errors in Rust

---

## Creating Custom Errors / Error Types

---

### Creating an Error Type for a _Library that you intend to publish_

Things to remember when creating an error:

**1. Error should be an Enum:** Technically, anything that can implement `std::error::Error` can be an error type, but using an enum is the most common and idiomatic way to represent multiple error types in Rust.

```rust
pub enum PuzzleError {
    WontFit(u16),
    MissingPiece,
}
```

**2. Group Related Errors:** You should always group related errors together, and make sure that the number of groupings are small. Basically group all the errors in as few enums as possible. This makes it easier to handle errors in a consistent way.

**3. Only YOUR Errors:** You should only return your own error from your library. You should not return errors from other libraries, unless you are wrapping them in your own error type. This is because you want to keep your error type simple and easy to understand. You don't want to pass through errors from external dependencies, because

1. It gives external dependencies a way to break your public API.

2. It prevents you from changing your own libraries backend implementation without breaking your public API.

There is an exception to this rule, when you receive a std library error, it might makes sense to pass it through.

**4. Non Exhaustive Errors:** Your error enum should be non-exhaustive. This can be done by adding `#[non_exhaustive]` attribute to the enum. This allows you to add new error variants in the future without breaking existing code that uses your library. It forces users to handle the error with a catch-all wildcard pattern. This means we don't break existing code when we add new error variants.

**5. Debug + Display + Error:** Your error type should implement `Debug`, `Display` and `Error` traits in that order. This is because `Error` trait is defined as sub-trait of `Debug` and `Display`. So you need to implement `Debug` and `Display` first, before implementing `Error`.

```rust
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
#[non_exhaustive]
pub enum PuzzleError {
    WontFit(u16),
    MissingPiece,
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PuzzleError::*;
        match self {
            WontFit(size) => write!(f, "Puzzle piece won't fit, size: {}", size),
            MissingPiece => write!(f, "Puzzle is missing a piece"),
        }
    }
}

impl Error for PuzzleError {}
```

**5b. Use `thiserror` crate:** If you want to avoid writing boilerplate code for your error types, you can use the `thiserror` crate. It provides a convenient way to define error types with minimal boilerplate. You can use `#[derive(Error)]` to automatically implement the `Error` trait.

```toml
// Cargo.toml
[dependencies]
thiserror = "1.0"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("Puzzle piece won't fit, size: {0}")]
    WontFit(u16),
    #[error("Puzzle is missing a piece")]
    MissingPiece,
}
```

---

## Handling Errors in an _Application_

---

### What does `Result` looks like?

```rust
#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Non-Recoverable Errors

If there is no way for a program to sanely proceed any further, then you have a nuclear option, that is `panic!(<text>)`. This is what is called under the hood when you call `expect("some message")` or `unwrap()`. With expect we have a panic message, but with unwrap panic happens without a message.

#### You should only use `panic!` when you are sure that there are no other reasonable options, otherwise why are you crashing the program?

- Library authors should never use `panic!` if they can avoid it. Don't even call `unwrap()` or `expect()` in your library code, because it will panic if the value is `None` or `Err`. Instead, you should return a `Result` or an `Option` type.

- Application can `panic!`, whenever they feel appropriate, but it still should be a last resort. Don't try to catch panic, that defeats the whole purpose.

### Recoverable Errors

If error is recoverable, then you should either, _Handle_ or _Return_ it.

- `if let` expression is a good way to handle error, if you care more about the error than the value.

```rust
if let Err(e) = do_something() {
    println!("Error occurred: {}", e);
    // Handle the error, maybe log it or notify the user
}
```

- `match` expression is a good way to handle error, if you care about both the value and the error.

```rust
let score = match get_saved_score() {
    Ok(score) => score,
    Err(_) => 0, // Default score if there is an error
}
```

- _Return the error_ if you don't know how to handle it, or if you want to propagate it up the call stack. This is done by returning a `Result` type.

```rust
fn poem() -> Result<String, io::Error> {
    let file = match File::open("pretty_words.txt") {
        Ok(f) => f,
        Err(e) => return Err(e), // Propagate the error
    }

    // Do something with the file
}
```

This type of thing is so common that Rust has a shorthand for it, called the `?` operator. It is used to propagate errors up the call stack.

```rust
fn poem() -> Result<String, io::Error> {
    let file = File::open("pretty_words.txt")?; // Propagate the error if it occurs

    // Do something with the file
}
```

This is same thing as the previous example, but it is more concise and easier to read. The `?` operator will return the error if it occurs, otherwise it will return the value. This is very useful when you want to chain multiple operations that can fail, without having to write a lot of boilerplate code.

For example, here the `?` operator can be used to chain multiple operations that can fail with similar error types:

```rust
pub fn autobots_rollout() -> Result<Vehicle, TransformerError> {
    let optimus = Transformer::new("Optimus Prime")?;
    optimus.stand()?.transform()?.rollout()?.chase()?
}
```

#### What if you are dealing with multiple different types of errors?

In a library, advice is to handle the error and return a error type that you defined. Then you can use the `?` operator to propagate the error up the call stack. This way you can handle the error in a consistent way, and you can also provide more context about the error.

For application, we can leverage the power of traits using `anyhow` crate.

```toml
[package]
name = "puzzle_game"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
puzzles = { path = "../puzzles" }
```

`anyhow` has a special result type. This Result type does all the right thing with error trait and generics, to make sure that it works well with any kind of error.

```rust
use anyhow::{Context, Result};
use puzzles::Puzzle;
use std::fs::File;

fn get_puzzle(filename: &str) -> Result<Puzzle> {
    let fh = File::open(filename).with_context(|| format!("Failed to open puzzle file: {}", filename))?;
    let puzzle = Puzzle::from_file(fh).context("Failed to convert data into a puzzle")?;
    Ok(puzzle)
}

fn main() -> Result<()> {
    let puzzle = get_puzzle("puzzle.dat").context("Couldn't get the first puzzle")?;
    println!("Puzzle loaded: {:?}", puzzle);
    Ok(())
}
```

`Closure` inside `with_context` is used to provide additional context about the error, if it occurs. This is very useful for debugging, because it gives you more information about what went wrong. Closure is only called when error occurs, so we avoid overhead of allocating and formatting the string if everything goes well.
