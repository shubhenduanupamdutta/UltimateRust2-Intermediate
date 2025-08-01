# Multithreading in Rust

---

Rust has a portable api wrapping native operating system threads. So, all the code usually works on any platform that support native threads, that means all major platforms. Some smaller platforms (like embedded systems) may not support native threads,in that case the code will simply not compile.

---

## Overview of Multithreading

- Every process or program starts with a single thread, the main thread.
- You can launch (spawn in rust lingo) more threads from the main thread.
- Why not relaunch the main thread? Because threads are cheaper than processes, and you can share memory between threads, which is not possible with processes.
- Then system with multiple CPU cores became a thing, each logical core can run a thread, so you can run multiple threads in parallel. A single CPU can switch between processing thread fast enough that it looks like multiple threads are running at the same time on one CPU core. But in reality, each logical CPU can only run one thread at a time.
- If you split your work into two threads, running on two different CPU cores, then you can get a performance boost up to 2x, minus some overhead of managing threads, and communication between them.
- Multithreading == Parallel Processing

```rust
use std::thread;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(move || {
        // This is the new thread, it runs concurrently with the main thread
        // Do stuff in a child thread
    });

    handle.join().unwrap(); // Wait for the thread to finish
}
```

- Above is an example of spawning a new thread in Rust.
- The `thread::spawn` function takes a closure that contains the code to run in the new thread.
- The `move` keyword is used to move ownership of variables into the closure, allowing the new thread to access them.
- The `join` method is called on the thread handle to wait for the thread to finish before continuing in the main thread.
- `join` returns a `Result`, so you need to handle the error case, which is why we call `unwrap()` to panic if the thread panics.
- Threads aren't free, creating a thread allocates memory on stack (usually few MBs). Whenever a CPU changes from one thread to another, it has to do an expensive context switch, which involves saving the state of the current thread and loading the state of the new thread.

---

## Asynchronous Programming

When you have a lot of I/O operations or waiting for external resource, then the tool of choice is asynchronous programming. It allows you to write code that can handle many tasks at once without blocking the main thread. This is done using the `async` and `await` keywords in Rust. It allows you to write code that looks synchronous, but runs asynchronously under the hood. It is a much more efficient way to handle waiting for some tasks to complete, such as network requests or file I/O.

---

## A code example for multithreading

Let's cook spaghetti in parallel. Our spaghetti will come with spaghetti sauce. Cooking the spaghetti, cooking the sauce and setting the table are all independent tasks that can be done in parallel. We will use threads to run these tasks concurrently.

```toml
// Cargo.toml
[package]
name = "kitchen"
version = "0.1.0"
edition = "2021"

[dependencies]
log = "0.4"
env_logger = "0.11"
```

```rust
// src/main.rs
use log::{error, info};
use std::{thread, time::Duration};

fn sleep(seconds: f32) {
    thread::sleep(Duration::from_secs_f32(seconds));
}

pub mod dad {
    use super::{info, sleep};

    pub fn cook_spaghetti() -> bool{
        info!("Cooking the spaghetti...");
        sleep(4.0);
        info!("Spaghetti is ready!");
        true
    }
}

pub mod mom {
    use super::{info, sleep};

    pub fn cook_sauce_and_set_table() {
        sleep(1.0);
        info!("Cooking the sauce...");
        sleep(2.0);
        info!("Sauce is ready! Setting the table...");
        sleep(2.0);
        info!("Table is set!");
    }
}

fn main() {
    env_logger::init(); // Initialize the logger

    info!("Starting to cook spaghetti and sauce...");

    // Spawn a thread to cook spaghetti
    let spaghetti_thread = thread::spawn(|| {
        dad::cook_spaghetti()
    });

    // Cook sauce and set the table in the main thread
    mom::cook_sauce_and_set_table();

    // Wait for the spaghetti thread to finish
    match spaghetti_thread.join() {
        Ok(_) => info!("Spaghetti time! Yum!"),
        Err(e) => error!("Dad messed up! Order Shahi Paneer instead!😉"),
    }
}
```

## What can you return from a thread?

Anything that implements the `Send` trait can be returned from a thread. This includes most types, such as integers, strings, and structs. However, types that do not implement `Send`, such as `Rc<T>` or `RefCell<T>`, cannot be returned from a thread
