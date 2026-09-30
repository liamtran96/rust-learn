# Rust Retrieval Homework - 2026-09-30

**Focus:** Chapter 3 - trait bounds, static and dynamic dispatch, boxed ownership, and enum modeling
**Based on:** completed Chapter 3 / Week 4; official tracker now Phase 3 / Week 5. No Week 5 material is tested.
**Timebox:** 25-35 minutes

## Instructions

Answer from memory first. For code questions, predict before running anything. Explain the Rust rule behind each answer. Write answers directly below each prompt. The application context is included so you know what each piece of code is for.

## Questions

### 1. What does the signature promise?

A reporting app creates labels for receipts and shipping notices. Both types implement this trait:

```rust
trait Label {
    fn label(&self) -> String;
}

fn announce<T: Label>(record: &T) {
    println!("{}", record.label());
}
```

Explain the separate purposes of `T`, `T: Label`, and `&T`. For a call with a `Receipt`, what does `T` become? Could this function accept a third type, `DraftReport`, that does not implement `Label`? Explain how the method implementation is selected.

**Your answer:**

### 2. Who owns the feed items?

A dashboard stores invoices and shipping notices in one list. Both types implement `Label`:

```rust
let records: Vec<Box<dyn Label>> = vec![
    Box::new(invoice),
    Box::new(shipping_notice),
];
```

Explain the jobs of `Vec`, `Box`, and `dyn Label`. Name the vector's element type, say whether the inner values keep their concrete types, and describe what happens to ownership of the original `invoice` when it is boxed. What happens to the owned values when `records` is dropped?

**Your answer:**

### 3. Diagnose a printing mistake

A developer wants to print a report's label. Assume the trait definition is in scope:

```rust
trait Label {
    fn label(&self) -> String;
}

fn print_label<T: Label>(record: &T) {
    println!("{}", record);
}
```

Does the generic function compile as written? Explain which capability `{}` requires and what the declared bound guarantees. Change only the print statement to fulfill the stated purpose, without adding a trait bound.

**Your answer:**

### 4. Trace one item and a whole loop

```rust
trait Label {
    fn label(&self) -> String;
}

struct Invoice;
struct ShippingNotice;

impl Label for Invoice {
    fn label(&self) -> String {
        String::from("invoice ready")
    }
}

impl Label for ShippingNotice {
    fn label(&self) -> String {
        String::from("parcel dispatched")
    }
}

fn main() {
    let records: Vec<Box<dyn Label>> = vec![
        Box::new(Invoice),
        Box::new(ShippingNotice),
    ];
    println!("{}", records[1].label());
    for record in &records {
        println!("{}", record.label());
    }
}
```

Predict every output line in order. For the indexed call and each loop iteration, identify which implementation runs. Does the loop consume `records`? Explain your reasoning.

**Your answer:**

### 5. Write a small state model

A file upload is exactly one of: waiting, sending with a byte count, finished with a receipt string, or failed with a reason string.

Write an enum for these states and a method `fn status(&self) -> &str` that returns a fixed short label for each state. Use an exhaustive `match` with explicitly named variants and no catch-all arm. Explain why you chose `&self` and what compiler feedback you would expect if a new variant were added without updating the method.

**Your answer:**

### 6. Transfer to a settings screen

A settings screen stores text fields and checkboxes. They have different fields and rendering behavior, but the screen needs to call `render(&self) -> String` on each. It must own both kinds of controls in one list.

Propose a trait declaration and the list's type. Explain why `Vec<TextField>` would not fulfill the requirement. Also propose a generic function signature for rendering one control, and explain how its dispatch differs from calls through your mixed list.

**Your answer:**

## Confidence check

For each question, rate confidence from 1 (guessing) to 5 (certain) before checking notes.
