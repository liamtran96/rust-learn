---
title: Ch 9 — Smart Pointers Exercises
tags: [rust, exercises, smart-pointers]
---

# Ch 9 — Smart Pointers Exercises

1. **Recursive enum**: define a `BinaryTree<T>` using `Box`. Implement in-order traversal as an iterator.
2. **Shared config**: build `Arc<Config>` and `Arc::clone` it into 4 threads; each thread reads a field.
3. **Rc<RefCell<T>> graph**: build a small social-graph `User { followers: RefCell<Vec<Rc<User>>> }`. Add bidirectional follow/unfollow operations. Observe what happens if you form a cycle — use `Weak` to break it.
4. **Interior mutability trade-offs**: take an `Rc<RefCell<T>>`-heavy design and rewrite it in a single-owner way. Which is easier to read?
5. **Implement MyBox<T>**: a tuple struct `MyBox<T>(T)` with `impl Deref`. Show that you can use it with `&*b` and with deref coercion in a function accepting `&str`.

## `Drop`
6. Implement `struct Guard { name: String }` with a `Drop` impl that prints on destruction. Verify drop order for locals vs struct fields.
