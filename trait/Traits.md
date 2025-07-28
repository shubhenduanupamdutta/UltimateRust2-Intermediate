# Common Trait

---

## What can implement a Trait?

There are four things that can implement a Trait

- **Struct**
- **Enum**
- **Function**
- **Closures**

Function and Closures only every implement a few advanced trait. So we will focus on Traits for Struct and Enums.

---

## Derivable Traits

- Traits can be derived if it has a `derive` macro defined for it.

---

### `Debug`

One of the most common trait to derive is `Debug`. Debug trait is used for Debug formatting or Pretty Debug formatting.

```rust

#[derive(Debug)]
struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

let puzzle = Puzzle {num_pieces: 500, name: "Draconic Equestrian"};

// Debug print
println!("{:?}", puzzle);

// Pretty Debug print
println!("{:#?}", puzzle);
```

---

### `Clone`

Another common derivable trait is `Clone`. Implementing this trait allows the value to be cloned by calling the `.clone()` method. If everything (every field) inside your struct or enum implements `Clone` trait then `Clone` trait for the struct/enum can be derived via `derive` macro.

```rust
#[derive(Clone, Debug)]
struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

let puzzle = Puzzle {num_pieces: 500, name: "Draconic Equestrian"};
let puzzle2 = puzzle.clone();
```

---

### `Copy`

Closely related to clone, is `Copy` trait. This is a special marker trait. If your type implements `Copy` then it will be copied instead of moved in moved situations. It makes sense for small values that can fit onto stack. That's why integer, floats and booleans all implement `Copy`. If type uses heap at all then it can't implement `Copy`. We can't implement `Copy` trait on `Puzzle` struct because it has a `String` field, which doesn't implement `Copy` trait.

`Copy` can't be implemented without `Clone`, since it is a sub-trait of `Clone`. So `Clone` needs to be implemented for `Copy` to be derived.

```rust
#[derive(Clone, Copy)]
pub enum PuzzleType {
    Jigsaw
}
```

---

## Traits you Implement

Steps to manually implement Traits

- **Step 1:** _Bring the Trait Into Scope_
- **Step 2:** _Boilerplate, using your IDE or by documentation_
- **Step 3:** _Implementation, actually implement the required methods_

### `Derive`

Let's see how can we implement `Derive` trait manually. It is available in Prelude, so nothing to import. It can actually be derived, but that usually results in 0 or empty strings.

```rust
#[derive(Clone, Copy, Debug)]
struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num_pieces: PUZZLE_PIECES,
            name: "Forest Lake".to_string(),
        }
    }
}
```

Why would you want to implement a `Default` trait, instead of putting this into some associated function like `new()`?
There are some idiomatic use cases, which uses default, like `Struct Update Syntax`.

```rust
let puzzle = Puzzle {
    num_pieces: 3000,
    ..Default::default()
}
```

---

### `PartialEq` / `Eq`

`PartialEq` is the trait that does the calculation to test for equality. `Eq` is the marker trait that you can implement if equality logic is **reflexive, transitive and symmetric**. For example if every possible value is not equal to itself, then the type can't have a `Eq` marker trait.
For example `NaN != NaN`. So float implement `PartialEq`, not `Eq`

Boilerplate for PartialEq

```rust
impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
```

Actual implementation

```rust
impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        (self.num_pieces == other.num_pieces) && (self.name == other.name)
    }
}

impl Eq for Puzzle {}
```

#### What does implementing `Eq` gets us?

Not a whole lot. We can use `Puzzle` as a HashMap key now, and you can use it other niche cases. Most cases equality checks only use `PartialEq` trait.

#### If we drop the `Eq` implementation, then we can modify `PartialEq` implementation to be more flexible, like not caring about case in string

```rust
impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        (self.num_pieces == other.num_pieces) && (self.name.to_lowercase() == other.name.to_lowercase())
    }
}
```

#### NOTE

- **So if you want to be flexible, or partial in your equality comparison, you should use `PartialEq` trait, and only that.**
- **If you are willing to implement fully _transitive, reflexive and symmetric_ equality then you can also implement `Eq` marker trait.**

---

### `From` / `Into`

They are interesting pair. If you implement `From` then `Into` gets automatically implemented for you. So you should always implement `From`, so you get both. But its more idiomatic to use into in Generic functions. So you implement `From` so you can use `Into`. These are both in std prelude, so no need for imports.

`From<T> for U`
`Into<U> for T`
They describe transformation between two types, but from different viewpoints.

#### In example, we want to end up with `Into<String> for Puzzle` so we will implement `From<Puzzle> for String`

Boilerplate for From

```rust
impl From<Puzzle> for String {
    fn from(_: Puzzle) -> Self {
        todo!()
    }
}
```

#### Actual implementation of `From`

```rust
impl From<Puzzle> for String {
    fn from(puzzle: Puzzle) -> Self {
        puzzle.name     // We are intentionally loosing num_pieces data.
    }
}
```

#### How do we use `From`. Let's see an example for `Puzzle`

```rust
let puzzle = Puzzle::default();
let s = String::from(puzzle);
```

#### Let's see an example of using `Into` in a generic function

```rust
pub fn show<T: Into<String>>(s: T) {
    println!("{}", s.into())
}
```

#### NOTE: Both `From` and `Into` consume the Puzzle, now. That will not be efficient for large data in puzzle. Let's modify `From` to work with immutable reference

```rust
impl From<&Puzzle> for String {
    fn from(puzzle: &Puzzle) -> Self {
        puzzle.name.clone()
    }
}
```

Above implementation will still be cheaper than cloning whole Puzzle, if that contains images.
But in `show` we don't have to change anything.

```rust
pub fn show<T: Into<String>>(s: T) {
    println!("{}", s.into())
}

let puzzle = Puzzle::default();
show(&puzzle)   // Puzzle is still available
```

---

### NOTE: References for things can have different set of traits implemented than the thing itself
