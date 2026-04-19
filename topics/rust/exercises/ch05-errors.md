---
title: Ch 5 — Error Handling Exercises
tags: [rust, exercises, errors]
---

# Ch 5 — Error Handling Exercises

1. Convert the following to use `?` and return `Result<(), Box<dyn Error>>`:
   ```rust
   fn main() {
       let s = std::fs::read_to_string("nums.txt").expect("read");
       let nums: Vec<i32> = s.lines().map(|l| l.parse().expect("parse")).collect();
       println!("sum = {}", nums.iter().sum::<i32>());
   }
   ```

2. Define a `ConfigError` enum with variants `Io(io::Error)`, `Parse(String)`, `MissingField(&'static str)`. Use `thiserror`. Use it in a function that reads and parses a JSON config.

3. Write `fn parse_line(s: &str) -> Result<(String, i32), MyError>` that expects `"name=number"` format. Propagate a `ParseIntError` via `#[from]`.

4. Using `anyhow`: write a CLI that reads a filename from argv, prints its word count, and adds context at each step (`"reading {path}"`, `"counting words"`) so the error chain is informative on failure.

5. `Option` → `Result`: given a function `lookup(key: &str) -> Option<User>`, turn it into `fn find_user(key: &str) -> Result<User, NotFound>` with a single `.ok_or(...)` call.

Stretch: write a custom error type for a **library** crate — concrete enum, `impl Error`, `source()` chain, no `anyhow` dependency.
