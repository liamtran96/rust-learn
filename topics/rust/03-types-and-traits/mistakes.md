---
title: Chapter 3 Mistakes
tags: [rust, mistakes, types, enums, traits]
---

# Chapter 3 Mistakes

## Open mistakes

### 2026-09-23 - Treating the generic placeholder as the selected type (`summary-trait` retrieval)
- **What I wrote:** `It's T type right`
- **Why it's wrong:** `T` is the parameter in the generic definition of `Vec`; it is replaced by the concrete element type at a use site. In `Vec<Box<dyn Summary>>`, the selected element type is `Box<dyn Summary>`.
- **The rule:** Generic parameters are placeholders for types; read the type argument between `<...>` at the use site to see what replaced the placeholder.
- **Status:** fresh

### 2026-09-23 - Mixing named-field and tuple-struct syntax (`typed-ids`)
- **What I wrote:** `struct TypeName{ UserId: u64, OrderId: u64, }`, followed by `struct <UserId>(u64);`
- **Why it's wrong:** The first form declares one type containing two named fields rather than two distinct ID types. In the second form, `<UserId>` was placeholder notation copied as literal syntax; a tuple struct expects its identifier directly after `struct`.
- **The rule:** Declare each newtype independently with `struct TypeName(InnerType);`; angle brackets denote generic arguments only in valid generic syntax, not an editable placeholder.
- **Status:** fresh

### 2026-09-23 - Missing why raw IDs lose type safety (`typed-ids`)
- **What I wrote:** `because we just pass the user id as a parameter to the function`
- **Why it's wrong:** This describes the successful call but not the protection. If the parameter is changed to `u64`, callers can pass either `user_id.0` or `order_id.0`, and the compiler can no longer distinguish their domain meanings.
- **The rule:** Keep domain meaning in parameter types; unwrapping distinct newtypes to the same primitive before a call erases the distinction the compiler could enforce.
- **Status:** fresh

### 2026-09-22 - Putting notification consumers inside the trait (`summary-trait`)
- **What I wrote:** `trait Summary { fn summarize(&self) -> String; fn notify<T: Summary>(s: &T); fn notify_dyn(items: &[Box<dyn Summary>]); }`
- **Why it's wrong:** This made both notification helpers required associated functions on every implementation. Because they have no `self` receiver, they also prevented `Summary` from being dyn-compatible and therefore blocked `Box<dyn Summary>`.
- **The rule:** Put only implementor behavior in a trait; functions that consume any implementor should normally be free functions. A trait used behind `dyn Trait` must satisfy Rust's dyn-compatibility rules.
- **Status:** dYY - fresh

### 2026-09-22 - Formatting a unit-returning function call (`summary-trait`)
- **What I wrote:** `println!("this is notify{}", notify());`
- **Why it's wrong:** `notify` requires a borrowed summarizable argument, but the call supplied none. It also prints internally and has no return arrow, so its return type is unit `()`, which does not implement `Display` for `{}` formatting.
- **The rule:** Supply every declared argument, and use a side-effecting unit-returning function as a statement rather than as a value to format.
- **Status:** dYY - fresh

### 2026-09-21 - Replacing a method receiver with a string type (`animal-trait`)
- **What I wrote:** `fn name(&str) -> &str`
- **Why it's wrong:** `&self` is the special receiver that borrows the current `Dog`; `&str` is a type and cannot replace the receiver name. The implementation signature must match the trait declaration.
- **The rule:** Keep `&self` in an implemented method when the trait declares `&self`; put `&str` after `->` when it is the return type.
- **Status:** dYY - fresh

### 2026-09-21 - Moving an owned field through `&self` (`animal-trait`)
- **What I wrote:** `fn sound(&self) -> String { self.sound }`
- **Why it's wrong:** `self` is only shared-borrowed, so moving its non-`Copy` `String` field out would leave the borrowed `Dog` partially moved.
- **The rule:** A method receiving `&self` may borrow fields; returning an owned field requires creating an independent owned value or consuming `self` when the API permits it.
- **Status:** dYY - fresh

### 2026-09-21 - Comparing a struct with its display text (`animal-trait` tests)
- **What I wrote:** `assert_eq!(dog, "Dog #1: HEH says woof");`
- **Why it's wrong:** `dog` has type `Dog`, while the expected value has type `&str`; the printed label from `main` is also not the return value of any trait method.
- **The rule:** Assert the observable method result against an independently written value of the same type, such as comparing `dog.describe()` with the expected `&str`.
- **Status:** dYY - fresh


### 2026-09-21 - Using type parameters as runtime values (`pair-swap`)
- **What I wrote:** `Pair { first: U, second: T }`
- **Why it's wrong:** `T` and `U` are compile-time type parameters, not expressions containing the pair's stored data. A struct constructor requires a value expression for every field.
- **The rule:** Generic parameters name types; access owned field values through expressions such as `self.first` and `self.second`.
- **Status:** dYY - fresh

### 2026-09-21 - Calling a consuming method without its receiver (`pair-swap`)
- **What I wrote:** `Pair::swap(1,2)`
- **Why it's wrong:** `swap` takes one `self` argument whose type is `Pair<T, U>`; two independent integers are not a pair and cannot supply that receiver.
- **The rule:** Construct the value first, then use `value.method()` when the method has a `self`, `&self`, or `&mut self` receiver.
- **Status:** dYY - fresh

### 2026-09-21 - Treating a field list as a test value (`pair-swap`)
- **What I wrote:** `assert_eq!(result, {first: liam, second: 2});`
- **Why it's wrong:** A brace block containing `first: ...` and `second: ...` is not a struct value without a type path, `liam` without quotes is not a string literal, and the expected number did not match the test input. Comparing the fields directly avoids requiring `Pair` itself to implement `PartialEq` and `Debug`.
- **The rule:** Each side of `assert_eq!` must be a valid expression; compare `result.first` and `result.second` with independent expected values when the enclosing struct lacks comparison traits.
- **Status:** dYY - fresh

### 2026-09-18 - Confusing empty input with an empty element (`generic-largest`)
- **What I wrote:** "because the String value can be empty so we should optional and return None if it's empty"
- **Why it's wrong:** `largest_borrowed` may validly return a reference to an empty `String` if that element wins the comparison. `None` represents a slice containing no elements at all, because only then is there no possible winner.
- **The rule:** Use `Option` when a result may be absent; here absence depends on the slice length, not on the contents of a `String` element.
- **Status:** dYY - fresh

### 2026-09-18 - Test name shadowed the function under test (`generic-largest`)
- **What I wrote:** Named test functions `largest_optional` and `largest_borrowed`, then tried to call the production functions by those names inside the test module.
- **Why it's wrong:** A test function declared in the module shadows an item with the same name brought in by `use super::*`, so the call resolved to the zero-argument test function instead.
- **The rule:** Give tests behavior-oriented names such as `optional_returns_none_for_empty`; avoid reusing the exact name of the function under test in the same namespace.
- **Status:** dYY - fresh

### 2026-09-14 - Calling an enum method without an enum value (`shape-area`)
- **What I wrote:** Tried to call `area("Circle")` instead of constructing a `Shape` and calling its method.
- **Why it's wrong:** `area` has a `self` receiver, so it operates on a concrete enum value whose active variant carries the required dimensions. A string such as `"Circle"` has neither the `Shape` type nor a radius.
- **The rule:** Construct the correct variant first, then use method syntax such as `shape.area()`; `&self` borrows that value for the call.
- **Status:** fresh

### 2026-09-14 - Putting calculations inside an enum constructor (`shape-area` tests)
- **What I wrote:** Placed `let s = ...` and `let area_squared = ...` inside `Shape::Triangle { ... }`.
- **Why it's wrong:** Struct-style variant braces initialize named fields; the parser therefore expects field names such as `a`, `b`, and `c`, not statements. The area formula belongs in the method that consumes those stored values.
- **The rule:** A constructor supplies the declared payload fields; behavior belongs in functions or methods.
- **Status:** fresh

### 2026-09-14 - Exact equality for a calculated floating-point result (`shape-area` tests)
- **What I wrote:** `assert_eq!(shape.area(), 12.566370614359172);`
- **Why it's wrong:** This test currently passes, but exact equality is brittle when floating-point calculations or their operation order change because many real numbers cannot be represented exactly in binary.
- **The rule:** For calculated `f64` values, usually assert that `(actual - expected).abs()` is smaller than a chosen tolerance.
- **Status:** fresh

### 2026-09-14 - Confusing an enum variant with a field (struct-versus-enum retrieval)
- **What I wrote:** "each shape value is exactly one field"
- **Why it's wrong:** A field is one stored piece of data, such as `radius`; it is not the alternative case represented by the whole value. `Circle` and `Rectangle` are variants, and their payloads contain fields.
- **The rule:** An enum value has exactly one active variant; that variant stores zero or more fields.
- **Status:** fresh

### 2026-09-14 - Using a namespace path instead of a method call (`network-state`)
- **What I wrote:** `connection::on_connect_attempt()` and, earlier, `ConnectionState::on_connect_attempt` without calling it on the existing value.
- **Why it's wrong:** `::` selects an item through a type or module path, while the lowercase `connection` binding is a value whose consuming method must receive that value as `self`. Naming a method without the call syntax produces a function item rather than the returned `ConnectionState`.
- **The rule:** Use `Type::Variant` to select an enum variant and `value.method()` to call a method on a particular value; the expression before `.` supplies `self`.
- **Status:** 🟥 fresh

### 2026-09-14 - Comparing a test result with itself (`network-state`)
- **What I wrote:** `assert_eq!(message, message)`
- **Why it's wrong:** Both sides refer to the same value, so the assertion passes regardless of which message the transition stored. It cannot detect a regression that changes or discards the caller-provided message.
- **The rule:** Compare the actual result with an independently specified expected value, such as `assert_eq!(message, "timed out")`.
- **Status:** 🟥 fresh

## Resolved (kept for reference)

*(none yet)*
