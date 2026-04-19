---
title: Ch 1 — Fundamentals Exercises
tags: [rust, exercises, fundamentals]
---

# Ch 1 — Fundamentals Exercises

## Warm-up
1. **FizzBuzz** — print 1..100, replacing multiples of 3 with `Fizz`, 5 with `Buzz`, both with `FizzBuzz`. Solve it three ways: with `if`, with `match`, and with an iterator `map`.
2. **Temperature converter** — CLI that reads an argument like `25C` or `77F` and prints the converted value.
3. **Guessing game** — the classic from the Rust book. Generate a number, prompt the user, compare, loop until right.

## Type system warm-up
4. Without running, predict what each of these prints:
   ```rust
   let x = 5;
   let x = x + 1;
   let x = x * 2;
   println!("{x}");

   let y = 5;
   {
       let y = y + 1;
       println!("{y}");
   }
   println!("{y}");
   ```

5. Why does this fail to compile, and what's the minimal fix?
   ```rust
   let a = 100_i32;
   let b = 200_i64;
   let c = a + b;
   ```

## Expression practice
6. Rewrite the following to eliminate `return`:
   ```rust
   fn sign(n: i32) -> &'static str {
       if n > 0 { return "positive"; }
       else if n < 0 { return "negative"; }
       return "zero";
   }
   ```

7. Write `fn count_digits(n: u32) -> u32` using a `loop` with `break value`.

## Checkpoint
You're done with Ch 1 when you can do all of the above without consulting docs for syntax.
