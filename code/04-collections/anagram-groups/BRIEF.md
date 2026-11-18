# Anagram groups — Brief

> Liam writes this himself. Claude only sets up the runway.
> When it compiles, runs, and you say "done", run `/journal` to log it.

## Spec
**Anagram groups** — given `Vec<String>`, return `Vec<Vec<String>>` grouping anagrams.

Example: `["eat", "tea", "tan", "ate", "nat", "bat"]` → `[["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]` (group order and word order inside a group may vary).

## Concepts in play
- Two words are anagrams when they contain the same letters. Find a **key** that is identical for every anagram of a word (think: what do "eat" and "tea" look like once their letters are put in order?).
- `HashMap<K, Vec<String>>` grouping: `entry(key).or_insert(...)` hands back a `&mut Vec<String>` you can `push` onto — the same `entry` API as median-mode, with a Vec instead of a counter.
- Turning a `&str` into its letters with `.chars()`, and collecting letters back into a `String` or `Vec<char>`.
- Getting the groups out of the map at the end (look at what `HashMap` offers besides `iter()`).

## Watch out for
`.clone()` spam (pitfalls.md #4): the function receives the `Vec<String>` by value, so each `String` can be **moved** into its group — you should not need to clone the words themselves.

## References (read first if stuck)
- Chapter: `topics/rust/04-collections/index.md`
- Specific notes: `topics/rust/04-collections/hashmap.md` (entry API), `topics/rust/04-collections/strings.md` (`.chars()`)
- Cheatsheet: `topics/rust/cheatsheets/iterator-methods.md`

## Checklist
- [ ] Read the chapter notes above (or skim if already read)
- [ ] `cargo run` compiles and prints something — even a stub
- [ ] Implement the spec as `fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>>`
- [ ] Try the example above plus an empty `Vec` and a single word
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Tell Claude "done" → `/journal` logs it and ticks the roadmap

## Run
```
cd code/04-collections/anagram-groups
cargo run
```
