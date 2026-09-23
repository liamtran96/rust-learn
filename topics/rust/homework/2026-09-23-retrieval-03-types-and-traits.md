# Rust Retrieval Homework - 2026-09-23

**Focus:** Chapter 3 types, enums, generics, traits, and derives
**Based on:** completed material through Phase 3 / Week 4
**Timebox:** 25-35 minutes

## Instructions

Answer from memory first. For code questions, predict before running anything. Explain the Rust rule behind each answer. Write answers directly below each prompt.

## Questions

### 1. Definition versus substitution

In your own words, explain the different jobs performed by `T`, `T: Clone`, and `::<Point>` in these declarations and calls:

```rust
fn requires_clone<T: Clone>() {}
requires_clone::<Point>();
```

Why would putting `T: Clone` inside the turbofish be invalid?

**Your answer:**

### 2. Diagnose the selected type

Predict whether this test proves that `Badge` implements `Hash`. State the inferred element type of `seen` and explain your reasoning.

```rust
#[derive(Hash, PartialEq, Eq)]
struct Badge(u32);

let mut seen = std::collections::HashSet::new();
seen.insert(42);
assert!(seen.contains(&42));
```

**Your answer:**

### 3. Trace an enum transition

Predict the final variant and stored value. Explain what lowercase `self` and uppercase `Self` mean and whether the first state remains usable after the method call.

```rust
enum Download {
    Waiting,
    Running { percent: u8 },
}

impl Download {
    fn start(self) -> Self {
        Self::Running { percent: 0 }
    }
}

let state = Download::Waiting;
let state = state.start();
```

**Your answer:**

### 4. Struct or enum?

A checkout can be exactly one of these states: empty cart, awaiting payment with a total, paid with a receipt number, or rejected with a reason. Choose a struct or enum and explain which invalid combinations your choice prevents. Sketch only the type declaration.

**Your answer:**

### 5. Write a capability-safe ID

Write a `TicketId` newtype around `u64`. Derive the traits needed to debug-print it, copy it, compare it for full equality, and store it in a `HashSet`. Then write only a function signature that accepts `TicketId` but rejects a raw `u64`.

Explain why every derived trait is permitted by the inner field type.

**Your answer:**

### 6. Static or dynamic dispatch?

You are building a notification screen that stores email alerts and system alerts in one collection, and both implement `Summary`. Decide whether the collection should use a generic `Vec<T>` or trait objects such as `Vec<Box<dyn Summary>>`. Explain why, including the concrete outer element type of your chosen collection and one ownership consequence of using `Box`.

**Your answer:**

## Confidence check

For each question, rate confidence from 1 (guessing) to 5 (certain) before checking notes.
