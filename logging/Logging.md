# Logging in Rust

---

Logging is an essential part of any application, allowing developers to track events, debug issues, and monitor application behavior.

In rust, there is an official library `log` which forms the foundation of logging in Rust. All libraries should use `log` for simple logging.

```toml
// Cargo.toml
[package]
name = "puzzles"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1.0"
log = "0.4"
```

There are five logging levels defined in the `log` crate. In order of descending severity/priority the are:

- `error!`: For errors that should be reported.
- `warn!`: For warnings that are not critical but should be noted.
- `info!`: For general information about the application's operation.
- `debug!`: For detailed information, typically of interest only when diagnosing problems.
- `trace!`: For the most detailed information, typically only enabled for development and debugging.

You can use these macros to log messages at the appropriate level:

```rust
use log::{error, warn, info, debug, trace};

fn main() {
    // Initialize the logger (e.g., env_logger)
    env_logger::init();

    // Log messages at different levels
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
```

You can use the `env_logger` crate to configure the logging output based on environment variables. This allows you to control the logging level without changing the code.

Only the logs that are at or above the configured level will be printed. For example, if you set the environment variable `RUST_LOG=info`, only `info`, `warn`, and `error` messages will be displayed.

log macros can take optional `target` argument to specify a target for the log message. This can be useful for filtering logs by module or component.

```rust
use log::{info, debug};

fn main() {
    env_logger::init();

    info!(target: "my_module", "This is an info message from my_module");
    debug!(target: "my_module", "This is a debug message from my_module");
}
```

You can use println syntax for interpolating variables into log messages, just like with `println!`.

```rust
let user = "Alice";
info!("User {} has logged in", user);
```

The log library defines a common **interface** via traits, that all basic loggers should confirm to. Libraries use only the `log` crate, they don't defined where to output the logs. Just like plumbing, `log` crate defines something analogous to standardized pipes and fittings, but we need to provide the output of the water.

Both library and binaries can use the `log` crate to create logs. But applications (binaries) should also use an additional logging library that provides the actual output of the logs. The most commonly used logging library is `env_logger`, which outputs logs to the console.

`env_logger` is a simple logger that reads the `RUST_LOG` environment variable to determine the logging level and format. It is easy to set up and use, making it a popular choice for many Rust applications.

`tracing` is a more advanced logging library that provides structured logging and supports various output formats, including JSON. It is useful for applications that require more complex logging capabilities, like async applications, span tracking, and more.
