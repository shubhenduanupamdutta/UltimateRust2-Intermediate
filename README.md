# [Ultimate Rust 2: Intermediate Concepts (Udemy Course)](https://www.udemy.com/course/ultimate-rust-2/)

This repository contains the code examples and exercises for the Udemy course "Ultimate Rust 2: Intermediate Concepts". The course is designed to help you deepen your understanding of Rust programming and its advanced features.

---

## Idiomatic Rust

---

_Idiomatic:_ Expressions that are natural to a native speaker of a language. In programming, idiomatic code is code that is written in a way that is natural and expected by other programmers familiar with the language.

### There are two tools developed by community members that help you write idiomatic Rust code

- _rustfmt_ - A tool that formats Rust code according to style guidelines. It can be run manually using `cargo fmt` or automatically on save in your editor.
- _clippy_ - A linter that provides suggestions for improving your Rust code. It can be run manually using `cargo clippy` or automatically on save in your editor.

### More about `clippy`

Running `cargo clippy` compiles your code and checks for over 450 specific problems, patterns that are considered non-idiomatic or could be improved. It provides suggestions for better practices, such as using `if let` instead of `match` when you only care about one pattern, or using `?` for error handling instead of `unwrap()`.

Generally, the problem can be categorized into 4 categories:

- **Style**: Issues related to code formatting and style conventions. If there is more idiomatic way to write the code, clippy will suggest it.

- **Correctness**: Issues that may lead to bugs or unexpected behavior. Clippy will warn you about potential mistakes in your code. It will also warn you about codes that may lead to undefined behavior or are completely unnecessary.

- **Complexity**: Issues that indicate the code is too complex or hard to read. Clippy will suggest ways to simplify your code, making it more maintainable and understandable. These warning comes into two flavors:

  One where clippy knows for sure that the code is too complex, and one where it is just a suggestion that the code could be simplified.

- **Performance**: Issues that may lead to inefficient code. Clippy will suggest ways to improve the performance of your code, such as using more efficient data structures or algorithms.

You can google `clippy lints` to find the full list of lints that clippy checks for.

## Attributes

Attributes in Rust are metadata applied to some module, crate, or item. They can be used to control the behavior of the compiler or to provide additional information about the code.

They start with a `#` symbol and can be applied to various items in Rust, such as functions, structs, enums, and modules. Attributes can be used to enable or disable certain features, provide documentation, or control the visibility of items.
e.g. `#[allow(dead_code)]` is an attribute that tells the compiler to allow dead code, which is code that is never used or called. It is also called outer attribute, because it is applied just before the item it applies to.
