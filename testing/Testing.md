# Testing in Rust

---

## Unit Testing

---

Unit tests are small tests that check the functionality of individual components in your code. They are typically written in the same file as the code they test, using the `#[cfg(test)]` attribute to conditionally compile them only when running tests.
Unit tests are defined using the `#[test]` attribute, and they can use assertions to verify that the code behaves as expected. For example:

Idiomatic place for your tests to go is to the bottom of the same file whose code you are testing. The convention is to put the sub-module in-line by using `{}` right after the module declaration, rather then making a separate file for it. Sub-module is typically named `tests` or `test`.

But if we only define sub-module, test code will be shipped with our library. To avoid that, we can use `#[cfg(test)]` attribute to conditionally compile the test code only when running tests.

`#[cfg(...)]` attribute controls conditional compilation of the item it applies to. `#[cfg(test)]` means that the item will only be compiled when running tests, and not when building the library or binary.

```rust
pub fn snuggle(bunnies: u128) -> u128 {
    bunnies * 8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snuggling_bunnies_multiply() {
        assert_eq!(snuggle(1), 8);
        assert_eq!(snuggle(2), 16);
    }
}
```

### There are three types of assert macros in Rust

- `assert!`: Checks that the expression is true.
- `assert_eq!`: Checks that two expressions are equal. Expression should result in the same type and should implement `PartialEq` trait. It fails when arguments are not equal.
- `assert_ne!`: Checks that two expressions are not equal. It fails when arguments are equal.

### What about panic?

`panic!(...)` usually fails the test, unless it is marked with `#[should_panic]` attribute. This attribute indicates that the test is expected to panic, and it will pass if the code panics as expected.

### NOTE: Attributes stack, so it doesn't matter in which order you put them

Unit tests can also optionally return a `Result<(), E>` type. Success type is ignored, so we can use `()` as the success type. Why would you want to do that? Because it allows you to use the `?` operator to propagate errors, which can make your tests more concise and easier to read. If the test returns an error, the test will fail, and the error will be printed to the console.

Rust tests fail fast by default. It means that they stop after first section that fails by default.

`````rust
#[test]
fn bunny_result() -> Result<(), ParseIntError> {
    let bunnies = "four".parse::<u128>()?;
    assert_eq!(snuggle(bunnies), 32);
}

---

## How to run tests?

You can run tests using the `cargo test` command. This will compile your code and run all the tests defined in your project. If you want to run a specific test, you can use `cargo test <test_name>`.

Cargo will automatically search for tests in your library documentation and run them as well, but only for library crates.

````rust
/// # Example
///
/// ```
/// # use hello::snuggle;
/// let bunnies = snuggle(5);
/// assert_eq!(bunnies, 40);
/// ```
`````

`# use hello::snuggle;` is used to write the code which is necessary for code to run, but you want to hide from the documentation.

---

## NOTE: All the tests for library crates are in the same section. But tests from binary crates are not in the same section. Every library and binary crate has its own section

---

## Integration Testing

---

All integration tests go in the `tests` directory at the root of your project. Each file in this directory is compiled as a separate crate, allowing you to test your library as if it were used by an external user.

```bash
|- Cargo.lock
|- Cargo.toml
|- src
   |- lib.rs
|- tests
   |- integration_test.rs
```

Integration tests can use the same `#[test]` attribute as unit tests, and they can also use the same assertion macros. However, they are typically used to test the public API of your library, rather than individual components.

---

## Testing in Binary Crates

---

Convention is to put bare minimum of tests in the binary crate. Most of the tests should be in the library crate, and the binary crate should only have tests that are specific to the binary.

Usual way to test binary crates is to use `std::process::Command` to run the binary and check its output. This allows you to test the binary as if it were run from the command line.
