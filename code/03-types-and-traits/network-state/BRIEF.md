# Network connection state machine - Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Model a network connection as one enum whose variants represent disconnected,
connecting, connected, and failed states. Add methods for events that return a new
state, beginning with a successful connection event that stores the peer name.

## Expected input and output

Input and output are method arguments and return values, not stdin or a required print
format. For example, a success event receives an owned peer name and returns a
`Connected` state containing that name. The specification intentionally leaves the
remaining event names and transition policy for you to choose.

## Required Rust syntax

```rust
enum ConnectionState {
    Disconnected,
    Connecting { attempts: u32 },
    Connected { peer: String },
    Failed(String),
}

impl ConnectionState {
    fn on_connect_success(self, peer: String) -> Self {
        todo!()
    }
}
```

- `enum ConnectionState` declares one type with exactly one active state.
- `Disconnected` carries no extra data.
- `Connecting { attempts: u32 }` is a struct-style variant with a named counter.
- `Connected { peer: String }` owns the connected peer's text.
- `Failed(String)` is a tuple-style variant whose position stores an error message.
- `impl ConnectionState` groups behavior belonging to this state type.
- `self` without `&` moves the old state into the method, which fits a transition that
  replaces it with a new state.
- `peer: String` names an owned input parameter and its type.
- `-> Self` means the method returns another `ConnectionState`; inside this `impl`,
  `Self` is an alias for `ConnectionState`.
- `todo!()` is a temporary placeholder that compiles but panics if called.

## Your coding steps

1. Type the enum, construct one initial state in `main`, and print a scaffold message.
2. Add the `impl` block with the compiling success-method placeholder, then replace the
   placeholder with a returned state.
3. Choose the remaining events, demonstrate a short transition sequence, and add focused
   tests that verify the returned variants and their stored data.

## Concepts in play

- Enums as mutually exclusive application states
- Variants carrying only state-specific data
- Consuming methods that return replacement states

## Watch out for

Because a method taking `self` consumes the old state, assign its returned state before
trying to use the connection again.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/enums.md`, `topics/rust/03-types-and-traits/pattern-matching.md`

## Checklist

- [x] I can explain each part of the required syntax
- [x] `cargo run` compiles and prints a stub
- [x] Implement the spec
- [x] `cargo clippy -- -D warnings` is clean
- [x] `cargo fmt` applied
- [x] Tests pass when the exercise requires them
- [x] Tell Codex "done" so `$journal` logs the session

## Run

```text
cd code/03-types-and-traits/network-state
cargo run
```
