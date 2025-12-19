# Hell ORM

A lightweight, type-safe SQLite ORM for Rust.

## Why Hell ORM?

- No async runtime overhead for simple database operations
- No complicated migration systems to learn
- No excessive dependencies bloating your binary

## Features

- **Minimal Dependencies** - Only depends on [rusqlite](https://github.com/rusqlite/rusqlite). That's it.
- **Type-Safe** - Leverages Rust's type system to catch errors at compile time, not runtime
- **Smart Schema Management** - Automatically generates and executes schemas from your models
- **Blocking by Design** - Perfect for CLIs, scripts, and applications where async complexity isn't needed

## License
Hell ORM is licensed under the MIT license.


