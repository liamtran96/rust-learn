---
title: Common Pitfalls
tags: [rust, pitfalls, gotchas]
---

# Common Pitfalls

The bugs every Rust learner hits — mostly once.

## 1. Accidental move in a loop
```rust
for x in v { /* consumes v */ }
// Then you try to use v again
```
**Fix:** `for x in &v` (borrow) or `for x in &mut v` (mut borrow).

## 2. Trailing `;` on a returning expression
```rust
fn add(a: i32, b: i32) -> i32 { a + b; }  // returns ()
```
**Fix:** drop the semicolon.

## 3. Stringly-typed APIs
Using `String` everywhere instead of strong types.
**Fix:** newtype pattern — `struct Email(String); struct UserId(u64);`.

## 4. `.clone()` spam
Cloning to silence borrow checker errors.
**Fix:** sit with the error. Clone is usually a last resort.

## 5. `unwrap()` in production code
Works in examples, crashes in real use.
**Fix:** use `?` to propagate, or explicit `match`/combinators.

## 6. `&String` parameters
Less flexible than `&str`.
**Fix:** take `&str`. Same for `&Vec<T>` → `&[T]`, `&Box<T>` → `&T`.

## 7. Cloning `Rc`/`Arc` thinking it's expensive
`Rc::clone`/`Arc::clone` is just a refcount bump. Cheap. Don't avoid it — do prefer the explicit `Rc::clone(&x)` form over `x.clone()` for readability.

## 8. `std::sync::Mutex` across `.await`
Deadlocks your runtime on contention.
**Fix:** use `tokio::sync::Mutex`, *or* release the lock before the await.

## 9. Forgetting to `.await`
Your async function "does nothing" because futures are lazy.
**Fix:** `.await` it, or `tokio::spawn` to run it in the background.

## 10. Missing `move` on spawn closures
```rust
let x = String::from("hi");
tokio::spawn(async { println!("{x}"); });  // ❌ needs move
```
**Fix:** `tokio::spawn(async move { ... })`.

## 11. `.chars()` vs `.bytes()`
`"café".len() == 5` (bytes) but `.chars().count() == 4`.
**Fix:** reach for `chars()`/`char_indices()` when doing text work.

## 12. `&str` indexing by number
`s[0]` doesn't exist. `s[0..1]` panics if byte 0 isn't a char boundary.
**Fix:** `s.chars().next()` or `s.char_indices()`.

## 13. Silent integer overflow in release mode
Debug panics; release wraps. If wrap is wrong, use `checked_add`, `saturating_add`, or enable overflow checks.

## 14. Growing a `Vec` while holding a reference
```rust
let first = &v[0];
v.push(4);      // may reallocate, invalidating first
```
**Fix:** compute what you need from the ref first, *then* push. The borrow checker catches this.

## 15. Over-generifying
`fn f<T: Clone + Debug + Default + Eq + Hash + Serialize + ...>(...)` — probably too much.
**Fix:** start concrete, add bounds as needed.

## 16. Async runtime mismatch
Mixing libraries that require different runtimes (tokio vs async-std) leads to `not running inside a runtime` panics.
**Fix:** pick one runtime (tokio) and stick with it.

## 17. Unbounded channels
`mpsc::channel()` (unbounded) can OOM under load.
**Fix:** use `sync_channel(cap)` / `mpsc::channel(cap)` for back-pressure.

## 18. Ignoring `#[must_use]`
`v.sort()` mutates, fine. `it.map(f)` does nothing on its own. `result.is_ok()` without a use prints a warning — take it seriously.

## 19. Large binaries / slow builds due to generics
Heavy monomorphization everywhere.
**Fix:** accept `impl Trait` at the API surface but convert internally to a boxed trait object. Or cache intermediate results.

## 20. Chasing perfection in the type system
Rust's types can model *almost* anything — but fancy type tricks often aren't worth it in app code.
**Fix:** prefer ordinary enums and structs until you have real evidence the code would benefit.
