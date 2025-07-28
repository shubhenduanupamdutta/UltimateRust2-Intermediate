# Closures

## Closures are anonymous function that can borrow or capture some data from the scope, it is nested in. Syntax for the closure is

`|params| expr`
`|params| {expr1; expr2}`

## The compiler figures out they type of arguments and return type by how the closure is used

## Examples

```rust
let add = |x, y| x + y;
add(1, 2); // returns 3
```

```rust
|| {} // Closure doesn't need an argument or even an expression, it can work without them
```

## Closure can automatically borrow items from the scope

```rust
let s = "🍓";
let f = || {
    println!("{}", s);
};

f(); // prints 🍓
```

## Closure support move to take ownership of scope items

```rust
let s = "🍓";
let s2 = s.clone();
let f = move || {
    println!("{}", s2);
};

f(); // prints 🍓
// println!(s2);    // this will throw an error
```

## If you want to pass Closures to and from function, then we need to specify the type of the closure need to use one of following traits as type

- `Fn`: It captures variables in scope by immutable reference
- `FnMut`: It captures variables in scope by mutable reference
- `FnOnce`: Captures the variable by move semantics, i.e. by taking ownership
