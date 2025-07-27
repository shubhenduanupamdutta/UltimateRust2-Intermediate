# Documentation for Iterators in Rust

---

Closures are lot of times used with iterators.

**For loops iterate over any iterable values. That is because `for` uses iterator internally.**

## But where does iterator comes from?

Let's look at the following code

```rust
let v = vec![5, 6, 7, 8];

for num in v {
    println!("{}", num)
}
```

Vector `v` is not directly used by the for loop. For loop works on iterators and if the type is not an iterator it calls `into_iter()` function on the type, which converts the type into iterator. `into_iter()` is a method of the `IntoIterator` trait. Anything that implements `IntoIterator` is converted into an iterator by the for loop automatically. `into_iter()` takes ownership of the value, and thus consumes it, so after the for loop the value will not be available.
Let's modify the above code for functional programming.

```rust
let v = vec![5, 6, 7, 8];

v.into_iter().for_each(|num| println!("{}", num));
```

## Why would we go with functional approach with `into_iter()`?

- Iterators, are typically faster than for loop.
- You can take advantage of `Iterator Adaptors`. `Iterator Adaptors` are tool in functional programming paradigm, that takes an iterator and outputs another iterator, by taking some action on the values that pass through it.

### Some of the common adapters are

- **`.map()` Map adaptor, usually converts one value to another, usually takes ownership**
- **`.filter()` Filter adaptor, takes a immutable reference, so that we don't consume needed values**

### Note that, `Iterator Adaptors` produce iterators, and iterators are lazy. They don't do anything until they are consumed. That's why we need to consume using some `Iterator Consumers`. They consume iterators in some way, forcing other iterators to work

### Some of the common `Iterator Consumers` are

- **`.for_each()` Iterator consumer, which takes ownership, passed it to a closure, whose return value is discarded.**
- **`.sum()` It adds all the value and returns the sum. It needs the return type annotation.**
- **`.collect()` It will gather all the items, and put it into a new collection. But it doesn't know which collection to use, so you only need to define collection type `Vec<_>` will work.**

### Note -> `Iterator Consumer` uses generic so heavily, they usually have trouble identifying the type, which can be handled by providing an explicit type annotation for return types. If we are not returning anything, then you can use turbofish syntax. It is used to specify type for a generic function or method

```rust
let v = vec![5, 6, 7, 8];


// Filter adaptor, takes a reference,
v
    .into_iter()            // 5, 6, 7, 8
    .map(|x: i32| x * 3)         // 15, 18, 21, 24
    .filter(|y: &i32| *y % 2 == 0)// 18, 24
    .for_each(|z| println!("{}", z));
```

### Using turbofish with Sum

```rust
let number_vector = vec![5, 6, 7, 8];

number_vector
    .into_iter()
    .map(|x: i32| x * 3)
    .filter(|y: &i32| *y % 2 == 0)
    .sum::<i32>()
```

### Using only collection type in `collect`

```rust
let number_vector = vec![5, 6, 7, 8];

number_vector
    .into_iter()
    .map(|x: i32| x * 3)
    .filter(|y: &i32| *y % 2 == 0)
    .collect::<Vec<_>>()
```

### What is you don't want to consume the collection?

There are other iterators that take immutable and mutable reference of a collection.

- `into_iter()` Consumes, returns owned items, syntactic sugar is `for _ in v`
- `iter()` returns immutable references, syntactic sugar is `for _ in &v`
- `iter_mut()` returns mutable references, syntactic sugar is `for _ in &mut v`

There are other iterators, for example `HashMaps` have iterators for keys, values, and both.

### How to empty out collection without consuming the collection itself?

- `drain()`, returns an iterator taking ownership of items inside the collection, but not the collection itself, leaving the collection blank.
- If you want to take ownership of whole vector v `v.drain(..)`
