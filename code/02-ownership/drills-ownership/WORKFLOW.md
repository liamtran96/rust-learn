# Drill Workflow — step by step

> The full loop for working the ownership drills. `BRIEF.md` has the rules; this is the routine.
> Core loop: **predict → run → read the error → minimal fix → explain why → next.**

## One drill session

1. **Open the crate.**

   ```text
   cd code/02-ownership/drills-ownership
   ```

   Skim `BRIEF.md` once, then open the next drill file (start: `tests/d01_move.rs`).

2. **Predict — before running anything.**
   Fill in the `PREDICT:` line at the top of the file in your own words, e.g.
   *"won't compile — `s` was moved into `t`, so `{s}` is use-after-move."*
   Writing it down is the point; a guess in your head doesn't count.

3. **Run only that drill.**

   ```text
   cargo test --test d01_move
   ```

   Or keep `bacon drills-ownership-test` running in a second terminal and just save the file.

4. **Read the compiler error against your prediction.**
   Were you right? The error names the rule — read the whole thing, including the `help:` lines, before touching the code.

5. **Fix it minimally.**
   Change the least code that makes it green. No `.clone()` unless you can defend it in the next step. Re-run until it passes.

6. **Write the `WHY:` line before opening the next drill.**
   One sentence — the rule your fix relied on, e.g.
   *"a `&` borrow lets two names read the same value without a second owner."*
   Green with an empty WHY = not done.

7. **Repeat**: d01 → d02 → … → d12, ordered easy → hard.
   Stop wherever the timer ends and note which drill is next — that's tomorrow's first move.

## When all 12 are green

1. Run the finish line:

   ```text
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt
   ```

2. Checkpoint from `topics/rust/exercises/ch02-ownership.md`: explain the four listed compiler errors **without notes** — out loud or in writing.
3. Say **"done"** so `$journal` logs the session, the questions asked, and the roadmap tick.

## The rhythm across days

- One 25–60 min session per day; 2–4 drills per session is a good pace. Don't binge all 12 — spacing is what makes it stick.
- Stuck more than ~10 minutes? That's not failure — ask about the *error message* and read it with the agent, rather than being handed the fix.
- Open the chapter notes (`topics/rust/02-ownership/index.md`) only to unblock, not before starting — code first, read to unblock.

## Drill order

| Drill | Kind | Teaches |
|---|---|---|
| `d01_move` | fix-it | use-after-move (E0382) |
| `d02_copy_vs_move` | fix-it | `Copy` vs move semantics |
| `d03_borrow_then_mutate` | fix-it | shared borrow vs mutation (E0502) |
| `d04_two_mut_borrows` | fix-it | one `&mut` at a time (E0499) |
| `d05_for_consumes` | fix-it | `for x in v` consumes `v` |
| `d06_str_params` | fix-it | `&str` over `&String` |
| `d07_dangling` | fix-it | no references to locals (E0106) |
| `d08_lifetime_elision` | fix-it | explicit `<'a>` when elision fails |
| `d09_slice_window` | implement | return a borrowed slice |
| `d10_mut_through_ref` | implement | mutate through `&mut [T]` |
| `d11_scoped_return` | fix-it | trailing `;` returns `()` |
| `d12_scanner_peek` | implement | struct holding `&'a str`, `&self` vs `&mut self` |
