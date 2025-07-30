# Benchmarking in Rust

---

Rust has a built-in way to benchmark your code, but it is not really finished. It is only available in the nightly version of Rust. You can use the `#[bench]` attribute to mark a function as a benchmark, and then you can run it using the `cargo bench` command.

---

## Criterion

---

Criterion is a powerful benchmarking library for Rust. It provides a way to write benchmarks that are more accurate and reliable than the built-in `#[bench]` attribute. It also provides a way to compare the performance of different versions of your code.
You can use Criterion to write benchmarks for your code by adding the `criterion` crate to your `Cargo.toml` file:

```toml
[dev-dependencies]
criterion = { version = "0.3", features = ["html_reports"] }

[[bench]]   // This is the toml syntax for defining section which you can have more than one
name = "snuggle_speed"  // Name of the rust source file, that will hold your
harness = false // Harness will be false to disable the default harness, so you can use Criterion
```

Then you can create a new file in the `benches` directory of your project. If the directory does not exist, you can create it. The file should have a `.rs` extension, and it should contain the benchmark code.
Name in `benches` directory should match the bench name in `Cargo.toml`. For example, if you have a benchmark named `snuggle_speed`, you should create a file named `snuggle_speed.rs` in the `benches` directory.

```bash
|- Cargo.lock
|- Cargo.toml
|- src
   |- lib.rs
|- benches
   |- snuggle_speed.rs
```

Then you can write a benchmark function using the `criterion::Criterion` type. Here is an example of a simple benchmark:

```rust
// In benches/snuggle_speed.rs
use criterion::{criterion_group, black_box, criterion_main, Criterion};
use hello::snuggle;

pub fn snuggle_benchmark(c: &mut Criterion) {   // Name of the function can be anything
    c.bench_function("snuggle 5 bunnies", |b| {
        b.iter(|| snuggle(black_box(5)))
    });
}

criterion_group!(benches, snuggle_benchmark);
criterion_main!(benches);
```

Then you can run the benchmark using the `cargo bench` command. This will compile your code and run the benchmarks defined in your project. Criterion will automatically generate a report in the `target/criterion` directory, which you can view in your web browser.
