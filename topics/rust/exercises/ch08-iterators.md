---
title: Ch 8 — Iterators Exercises
tags: [rust, exercises, iterators]
---

# Ch 8 — Iterators Exercises

**Constraint: no explicit `for` loops. Use iterator chains.**

1. Sum the squares of even numbers from 1 to 100.
2. Given `Vec<&str>`, return `Vec<String>` uppercased.
3. Given `Vec<i32>`, return `(min, max, mean, median)` as a struct.
4. FizzBuzz as an iterator chain producing `Vec<String>`.
5. Given `Vec<Vec<i32>>`, flatten and return the top 5 distinct values.
6. Given `&str`, return the length of the longest run of the same character.
7. Compute the **dot product** of two `Vec<f64>` using `zip` + `map` + `sum`.

## Writing iterators

8. Implement `struct Fib { a: u64, b: u64 }` that is an iterator yielding Fibonacci numbers. Iterate and collect the first 20.
9. Implement `fn batches<I: Iterator>(iter: I, n: usize) -> impl Iterator<Item = Vec<I::Item>>` — chunks the input iterator into Vecs of size n.

## Short-circuiting
10. Using `try_fold`, parse a `&str` containing space-separated integers into `Result<Vec<i32>, _>`, returning the **first** parse error.
