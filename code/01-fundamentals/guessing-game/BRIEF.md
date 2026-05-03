# Guessing game — Brief

> Liam writes this himself. Claude only sets up the runway.
> When it compiles, runs, and you say "done", run `/journal` to log it.

## Spec
**Guessing game** — the classic from the Rust book. Generate a number, prompt the user, compare, loop until right.

## Concepts in play
- `loop` + `break` — infinite loop you exit on a correct guess
- `std::io::stdin().read_line(&mut buf)` — mutable buffer, returns `io::Result`
- `String::trim().parse::<u32>()` — parsing user input, returns `Result<u32, _>`
- `match` on `Ordering` (`std::cmp::Ordering::{Less, Equal, Greater}`) returned by `.cmp(&other)`
- Adding the `rand` crate as a dependency (`cargo add rand`) and calling `rand::thread_rng().gen_range(1..=100)`

## Watch out for
**`unwrap()` will crash on bad input** (pitfall #5). When the user types `"abc"`, `parse()` returns `Err` — handle it with `match` or `if let Ok(n) = ... else continue` so a typo just re-prompts instead of panicking.

## References (read first if stuck)
- Chapter: `topics/rust/01-fundamentals/index.md`
- Specific notes: `topics/rust/01-fundamentals/control-flow.md` (loop/break/match), `topics/rust/01-fundamentals/data-types.md` (parse + integer types)
- Cheatsheet: `topics/rust/cheatsheets/error-handling.md` (Result patterns for the parse step)

## Checklist
- [ ] Read the chapter notes above (or skim if already read)
- [ ] `cargo add rand` — wire up the RNG dependency
- [ ] `cargo run` compiles and prints something — even a stub
- [ ] Implement the spec: generate target, loop, prompt, parse, compare, break on equal
- [ ] Bad input (`"abc"`) re-prompts instead of crashing — no `unwrap()` on `parse()`
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Tell Claude "done" → `/journal` logs it and ticks the roadmap

## Run
```
cd code/01-fundamentals/guessing-game
cargo run
```
