# Channels for concurrency in Rust

---

Rust standard library has a channel implementation in the `std::sync::mpsc` module, that was initially created by the rust core developers to use in part of firefox that is called _Servo_. After putting the implementation in the standard library, they realized they have made some poor design choices. They realized they can't change the API without breaking the compatibility. But rust team won't break compatibility, so they created a new crate called `crossbeam` that has a better API, faster and has more features. The `crossbeam` crate is the recommended way to use channels in Rust.

---

## What is a channel?

---

A channel is a one way queue that thread can use to send one type of value to another thread. To able to send the value to another thread, the type needs to implement the `Send` trait. `Send` is a marker trait, much like `Copy` or `Eq`. Rust compiler will automatically implement `Send` for any type that is safe to send between threads. All primitive types, like `i32`, `f64`, `bool`, etc. implement `Send`. All types that implement `Send` can be sent through a channel. In fact, most standard library types implement `Send`.

If you are the type of genius that wants a custom implementation of `Send`, you can do that, but you need to be very careful. If you implement `Send` for a type that is not safe to send between threads, you will create a data race.

### There are two flavors of channels

- **bounded:** A bounded channel has a fixed capacity, once the channel is full, the sender will block until the receiver consumes some of the values. This is a great way to apply backpressure to the sender, so it doesn't overwhelm the receiver with too many values.
- **unbounded:** An unbounded channel has no capacity limit (until you run out of memory and crash), so the sender can send as many values as it wants. This is useful when you don't care about backpressure and just want to send values as fast as possible. This type of channel is great when you have a load that sometime bursts with lot of output, but you are sure it will not overwhelm the memory capacity of the system.

### Fan-Out pattern

One sender can have multiple receivers. Only one of the receivers will receive the value, the other receivers will not receive it. This is called a _fan-out_ pattern. The sender can send values to multiple receivers, but only one of the receivers will receive each value. Who will receive the value is not deterministic.

### Fan-In pattern

If you use `crossbeam` crate, you can also have multiple senders that can send values to a single receiver. This is called a _fan-in_ pattern. In this case, the receiver will receive values from all the senders, but it will not know which sender sent the value.

### Bi-directional communication

You can also have multiple senders and multiple receivers, but this is not supported by the standard library channels. You can use `crossbeam` crate for this. But the flow only goes in one direction from senders to receivers.

If you want bi-directional communication, you can use two channels, one for each direction. This is called a _pipe_. You can also use `crossbeam` crate for this. For this, you have to be very careful with your design, because if your channel flow is cyclical, you will create a deadlock. A deadlock is a situation where two or more threads are waiting for each other to finish, but they are stuck in an infinite loop. This can happen if you have a cyclic dependency between the threads.

---

## Code Examples

```toml
[package]
name = "cafeteria"
version = "0.1.0"
edition = "2021"

[dependencies]
crossbeam = "0.8"
```

```rust
// src/main.rs

use crossbeam::channel::{self, Sender, Receiver};
use std::{thread, time::Duration};

#[derive(Debug)]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!("{} received order: {}", name, order);
        let lunch = match &order {
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("salad") => Lunch::Salad,
            x if x.contains("sandwich") => Lunch::Sandwich,
            x if x.contains("hot dog") => Lunch::HotDog,
            _ => Lunch::Sandwich, // Default to Sandwich if no match
        };

        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.1));
        }
        println!("{} sends a {:?}", name, lunch);
        if lunches.send(lunch).is_err() {
            break; // Exit if the channel is closed
        }
    }
}

fn main() {
    let (orders_tx, orders_rx) = channel::unbounded();
    let orders_rx2 = orders_rx.clone();
    // Create a second receiver since we have two workers, that will share the same orders channel.

    let (lunches_tx, lunches_rx) = channel::unbounded();
    let lunches_tx2 = lunches_tx.clone();
    // Create a second sender since we have two workers, that will share the same lunches channel, to send the lunches back to the main thread.

    let alice_handle = thread::spawn(|| cafeteria_worker("Alice", orders_rx2, lunches_tx2));
    let bob_handle = thread::spawn(|| cafeteria_worker("Bob", orders_rx, lunches_tx));

    for order in vec!["polish dog", "caesar salad", "onion soup", "reuben sandwich"] {
        println!("ORDER: {}", order);
        let _ = orders_tx.send(order);
    }
    drop(orders_tx); // Close the orders channel, only when the channel is empty, the receiving end will exit the loop.

    for lunch in lunches_rx {
        println!("Order Up! -> {:?}", lunch);
    }

    let _ = alice_handle.join();
    let _ = bob_handle.join();
}

```

The receiving end of the channel implements `IntoIterator` trait, and pauses the thread until something is received in the channel. When the channel is closed, the iterator will return `None`, and the loop will exit. Execution will fall to the end of the function, where you can handle the cleanup or any final actions. Usually main thread will handle clean termination of the child threads by closing the channel(s).
