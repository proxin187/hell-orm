# Hell ORM

A lightweight, type-safe SQLite ORM for Rust.

## Why Hell ORM?

- No async runtime overhead for simple database operations
- No complicated migration systems to learn
- No excessive dependencies bloating your binary

## Features

- **Minimal Dependencies** - Only depends on [rusqlite](https://github.com/rusqlite/rusqlite). That's it
- **Type-Safe** - Leverages Rust's type system to catch errors at compile time, not runtime
- **Smart Schema Management** - Automatically generates and executes schemas from your models
- **Blocking by Design** - Perfect for CLIs, scripts, and applications where async complexity isn't needed

## When to use Hell ORM

- **Embedded applications** - Desktop apps, IoT devices, or any software that bundles SQLite
- **Command-line tools** - Scripts and CLIs where simplicity and fast compilation matter
- **Single-user applications** - Personal productivity tools, local-first apps, or desktop utilities
- **Testing and development** - Quick database setup for unit tests or local development

## When not to use Hell ORM

- **When you need migrations** - No built-in migration system
- **Web development in general** - Async frameworks like Actix/Axum expect async database operations
- **High concurrency scenarios** - SQLite's write locking may become a bottleneck

## License
Hell ORM is licensed under the MIT license.


