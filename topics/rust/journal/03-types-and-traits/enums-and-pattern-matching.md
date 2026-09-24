---
title: Enums and Pattern Matching
tags: [rust, journal, enums, pattern-matching]
---

# Enums and Pattern Matching

[[../../journal|Journal index]]

## Entries

### 2026-09-14 - Shape enum and area method
**Working on:** Shape area - `code/03-types-and-traits/shape-area/`
**What clicked:** A single `Shape` enum can represent several alternatives whose variants carry different named fields. An `area(&self)` method can immutably borrow the current shape, and an exhaustive `match` can destructure its active variant and return the appropriate calculation.
**What didn't:** `area` was initially treated like a free function taking a string rather than a method on a `Shape` value. The triangle constructor briefly contained calculation statements instead of its `a`, `b`, and `c` fields, and the purpose of `&self` was blended together with the separate role of match patterns. The circle test uses exact floating-point equality; it passes for the copied result but a tolerance comparison would be less brittle.
**Questions asked this session:**
- **Q:** "Example::First what is first?"
  - **Technical answer:** `Example` was a placeholder enum type and `First` was one of its placeholder variants. In this exercise the concrete paths are `Shape::Circle`, `Shape::Rectangle`, and `Shape::Triangle`; `::` selects a named item associated with the enum.
  - **Plain-English analogy / example:**
    ```rust
    enum Shape { Circle { radius: f64 } }
    let shape = Shape::Circle { radius: 2.0 };
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
- **Q:** What do `&self` and `match self` do?
  - **Technical answer:** In `impl Shape`, `&self` is shorthand for `self: &Shape`, so the method reads a particular `Shape` without consuming it. `match self` inspects which variant the borrowed value contains, while patterns such as `Shape::Rectangle { w, h }` destructure that variant and bind references to its fields.
  - **Plain-English analogy / example:**
    ```rust
    match self {
        Shape::Rectangle { w, h } => w * h,
        // the other variants must also be covered
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/pattern-matching.md`
**Question to answer later:** Can Liam reconstruct the enum, exhaustive method, and tolerance-based floating-point tests without step-by-step prompting?
**Next:** Read `topics/rust/03-types-and-traits/structs.md`, then explain how a struct differs from an enum before starting the next Ch 3 exercise.

### 2026-09-14 - Struct versus enum retrieval
**Working on:** Preparation for Ch 3 network state machine - `code/03-types-and-traits/network-state/`
**What clicked:** Liam corrected the distinction that a `Shape` value is exactly one variant, while each variant stores only the fields needed for that case. An enum therefore makes circle, rectangle, and triangle mutually exclusive at the type level.
**What didn't:** The first recall used `field` where `variant` was intended. Liam did not initially identify an invalid value permitted by an all-in-one struct until shown optional radius, width, and height fields that could represent multiple shapes or no shape.
**Questions asked this session:**
- **Q:** Why is `Shape` better modeled as an enum instead of one struct containing every possible shape field?
  - **Technical answer:** A struct instance contains all fields declared by that struct, whereas an enum value contains exactly one active variant. Each enum variant may carry different fields, so `Shape` can require precisely the data belonging to its selected case.
  - **Plain-English analogy / example:**
    ```rust
    enum Shape {
        Circle { radius: f64 },
        Rectangle { w: f64, h: f64 },
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/structs.md`, `topics/rust/03-types-and-traits/enums.md`
- **Q:** What invalid combination could an all-in-one `Shape` struct accidentally allow?
  - **Technical answer:** Optional fields could all be present, claiming that one value is both a circle and rectangle, or all be absent, representing no shape. The enum declaration rules out both combinations because only one named variant can be active and that variant requires its own payload.
  - **Plain-English analogy / example:**
    ```text
    radius = Some(2.0)
    w      = Some(3.0)
    h      = Some(4.0)
    invalid: circle and rectangle at the same time
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
**Question to answer later:** Can Liam independently name both an impossible all-fields-present state and an all-fields-absent state?
**Next:** Open `code/03-types-and-traits/network-state/BRIEF.md`, then type the enum and one initial state in `src/main.rs`.

### 2026-09-14 - Network connection state machine
**Working on:** Network connection state machine - `code/03-types-and-traits/network-state/`
**What clicked:** A connection enum expresses one active state at a time, and each variant carries only state-specific data. Consuming methods take the old state as `self`, return a replacement `Self`, and allow a transition sequence to use shadowing. Match patterns both identify a variant and extract its stored fields for output or assertions.
**What didn't:** Method-call syntax was repeatedly confused with namespace syntax, including naming a method as a function item and writing `connection::method()` instead of `connection.method()`. The first failure transition ignored its input message, and the first failure-message test compared `message` with itself rather than an independent expected value.
**Questions asked this session:**
- **Q:** "When do we use enum instead of struct and on the other hand?"
  - **Technical answer:** A struct models fields that coexist in every value, whereas an enum models a value that is exactly one of several variants. Enum variants can carry different payloads, preventing contradictory combinations of fields that do not belong to the active state.
  - **Plain-English analogy / example:**
    ```rust
    struct App { name: String, connection: ConnectionState }
    enum ConnectionState { Disconnected, Connected { peer: String } }
    ```
  - **See also:** `topics/rust/03-types-and-traits/structs.md`, `topics/rust/03-types-and-traits/enums.md`
- **Q:** "What do u mean by transition? Where is that?"
  - **Technical answer:** A transition is a state-machine term, not a Rust keyword: an event consumes or examines an old state and produces a new one. Here, calling `on_connect_success` moves the old connection into `self` and returns a `Connected` value.
  - **Plain-English analogy / example:**
    ```rust
    let state = ConnectionState::Disconnected;
    let state = state.on_connect_success(String::from("server"));
    // Disconnected --success--> Connected
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
- **Q:** "Why did we use `Self` here?"
  - **Technical answer:** Inside `impl ConnectionState`, uppercase `Self` is an alias for the implemented type, `ConnectionState`. It avoids repeating the type name in return types and variant paths such as `Self::Connecting`.
  - **Plain-English analogy / example:**
    ```rust
    impl ConnectionState {
        fn reset(self) -> Self { Self::Disconnected }
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
- **Q:** "It mean `self` is what user pass to the function call and `Self` is the type of `ConnectionState`, right?"
  - **Technical answer:** Yes: lowercase `self` is the particular receiver value supplied by the expression before `.`, while uppercase `Self` denotes its type inside the `impl`. A plain `self` receiver moves that value into the method; `&self` would borrow it instead.
  - **Plain-English analogy / example:**
    ```rust
    connection.on_connect_attempt();
    // approximately: ConnectionState::on_connect_attempt(connection)
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`, `topics/rust/02-ownership/ownership.md`
- **Q:** "What do you mean by number 5?"
  - **Technical answer:** The fifth testing step required a wildcard match arm that fails when the result has an unexpected variant. Because Rust matches must be exhaustive, `_` covers the remaining variants, and `panic!` ensures those variants cannot make the test pass silently.
  - **Plain-English analogy / example:**
    ```rust
    match state {
        ConnectionState::Connecting { attempts } => assert_eq!(attempts, 2),
        _ => panic!("expected Connecting"),
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/pattern-matching.md`
- **Q:** "You mean I should write a test, right?"
  - **Technical answer:** Yes. A focused unit test sets up a state, runs one behavior, and asserts both the returned variant and its stored data so the transition policy is executable rather than assumed.
  - **Plain-English analogy / example:**
    ```text
    arrange: Disconnected
    act:     attempt twice
    assert:  Connecting with attempts = 2
    ```
  - **See also:** `topics/rust/07-testing/unit-tests.md`
- **Q:** "Give me the hint to write the test because I am not familiar with the syntax right now."
  - **Technical answer:** `#[cfg(test)]` includes the test module only for test builds, `#[test]` registers a function with the test runner, and `use super::*` imports the surrounding module's items. `assert_eq!` fails when actual and expected values differ, while a fallback `panic!` rejects the wrong enum variant.
  - **Plain-English analogy / example:**
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] fn transition_works() { /* arrange, act, assert */ }
    }
    ```
  - **See also:** `topics/rust/07-testing/unit-tests.md`
**Question to answer later:** Can Liam independently write `value.method()` calls and a non-tautological enum-transition test from a behavior statement?
**Next:** After recalling `self` versus `Self` and `.` versus `::`, scaffold Ch 3 exercise 3 for the generic `largest` functions.

### 2026-09-24 - Shipping the enum-driven state-machine library
**Working on:** State-machine library milestone - `code/03-types-and-traits/network-state/`
**What clicked:** `ConnectionState` is the enum type for mutually exclusive connection conditions. Moving it into `src/lib.rs`, marking caller-facing items `pub`, and importing it with `use network_state::ConnectionState` made the exercise reusable as a library. A retry test confirmed that the wildcard arm maps a failed state to `Connecting { attempts: 1 }`.
**What didn't:** The difference between the enum type and its variants needed review. Transition predictions initially followed the desired label rather than the event and actual `match` arm; the test also needed guidance for its binding name and terminating semicolon.
**Questions asked this session:**
- **Q:** `wait waht is ConnectionState`
  - **Prompt context:** The shipping brief asked for the existing state machine to move from the binary into a reusable library.
  - **Prompt code:** `pub enum ExampleState { Ready }`
  - **Liam's answer:** -
  - **Technical answer:** `ConnectionState` is a user-defined enum type, not a Rust built-in. Each value has exactly one active variant, and variants may carry state-specific data.
  - **Plain-English analogy / example:** `ConnectionState` is a status board; `Disconnected`, `Connecting`, `Connected`, and `Failed` are mutually exclusive marker positions.
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
- **Q:** `I dont understand this question` (old state + event -> resulting state)
  - **Prompt context:** The warm-up asked Liam to describe one existing transition before editing the crate.
  - **Prompt code:** -
  - **Liam's answer:** `Connected`
  - **Technical answer:** A transition describes the current enum variant, the event method applied to it, and the replacement variant returned. `on_connect_attempt` starts an attempt and returns `Connecting`; reaching `Connected` requires the separate success event.
  - **Plain-English analogy / example:** `Disconnected + on_connect_attempt() -> Connecting { attempts: 1 }` is like dialing a phone: dialing starts an attempt but does not mean the other person answered.
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
- **Q:** `how to make the ConnectionState public in lib.rs`
  - **Prompt context:** After moving the enum to `src/lib.rs`, the binary needed permission to name and use it.
  - **Prompt code:** `enum ConnectionState { /* variants */ }`
  - **Liam's answer:** -
  - **Technical answer:** `pub` is Rust's visibility keyword. `pub enum ConnectionState` exposes the type and variants outside the library module, while each caller-facing method also needs `pub fn`.
  - **Plain-English analogy / example:** `pub` is an unlocked front door on the library API; items without it remain private rooms.
  - **See also:** `topics/rust/06-modules/modules-and-paths.md`
- **Q:** `use network_state::ConnectionState; explain this to me`
  - **Prompt context:** `main.rs` needed to bring the public library type into the binary crate's scope.
  - **Prompt code:** `use network_state::ConnectionState;`
  - **Liam's answer:** -
  - **Technical answer:** A crate is a Rust compilation unit. `use` imports the item at that path so the binary can write the short name; Cargo converts the package hyphen in `network-state` to an underscore in the Rust crate identifier.
  - **Plain-English analogy / example:** Without the import, the full address is `network_state::ConnectionState::Disconnected`; `use` lets the file refer to the imported type by its final name.
  - **See also:** `topics/rust/06-modules/modules-and-paths.md`
- **Q:** `what do u mean?` (add another transition case)
  - **Prompt context:** The shipping brief required one focused test beyond the three transitions already covered.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** A transition case is one starting variant, one event method, and the resulting variant. Testing another combination makes behavior hidden in a wildcard arm explicit.
  - **Plain-English analogy / example:** A vending-machine rule can be written as `Idle + insert_coin -> HasCredit`; another starting condition and event is another transition case.
  - **See also:** `topics/rust/03-types-and-traits/pattern-matching.md`
- **Q:** `it is Failed right`
  - **Prompt context:** Liam predicted the result of calling `on_connect_attempt` on a `Failed` value carrying the text `timeout`.
  - **Prompt code:** `match self { Self::Connecting { attempts } => /* increment */, _ => Self::Connecting { attempts: 1 } }`
  - **Liam's answer:** `it is Failed right`
  - **Technical answer:** In the current implementation, `Failed` does not match the first arm, so `_` handles it and returns `Connecting { attempts: 1 }`. Keeping it failed would be a different design requiring an explicit `Failed` arm.
  - **Plain-English analogy / example:** `_` is the everyone-else queue: because only `Connecting` has its own queue, `Failed` follows the fallback arm.
  - **See also:** `topics/rust/03-types-and-traits/pattern-matching.md`
- **Q:** `what should i do`
  - **Prompt context:** After the retry policy was clarified, Liam needed one concrete next coding action.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** Add a focused unit test that arranges a failed state, calls `on_connect_attempt`, and asserts the returned variant and attempt count. Reusing the existing test structure keeps the change limited to the new behavior.
  - **Plain-English analogy / example:** Arrange the starting card, perform one action, then inspect the replacement card and its counter.
  - **See also:** `topics/rust/07-testing/unit-tests.md`
- **Q:** `i am not familiar with the syntax please`
  - **Prompt context:** Liam needed to write the retry test but had not retained the Rust test and match syntax independently.
  - **Prompt code:** `#[test] fn failed_connection_can_retry() { /* arrange, act, match */ }`
  - **Liam's answer:** -
  - **Technical answer:** `#[test]` registers a test; a `let` statement binds a value and ends with `;`; a `match` arm destructures `Connecting { attempts }`; `assert_eq!` compares values; and `_` with `panic!` rejects wrong variants. The method call must use the binding name created by the preceding `let`.
  - **Plain-English analogy / example:** The test creates a starting state, performs the event, opens the returned variant, checks its counter, and fails if another variant appears.
  - **See also:** `topics/rust/07-testing/unit-tests.md`
**Question to answer later:** Without notes, can Liam explain why `main.rs` needs both a public library item and a `use` path, then trace `Failed + on_connect_attempt()` through the wildcard arm?
**Next:** Complete the Week 4 structs/enums reading checkbox by reviewing `topics/rust/03-types-and-traits/structs.md` and `enums.md`, then type one small example from memory.
