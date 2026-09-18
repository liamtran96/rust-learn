# Rust Recall Lab prototype

This throwaway UI prototype tests one question: **which layout makes replaying Liam's real Rust learning-session questions easiest to return to?**

Run it from the repository root:

```powershell
cargo run --manifest-path practice-site-prototype/Cargo.toml
```

Then open <http://127.0.0.1:4173>. Use the floating arrows (or keyboard left/right arrows) to compare:

- `?variant=A` - one-question focus session
- `?variant=B` - dense review dashboard
- `?variant=C` - chapter-based learning trail

The server reads `topics/rust/progress.md`, `topics/rust/study-plan.md`, every topic journal under `topics/rust/journal/`, and the supporting `mistakes.md` files. The browser checks for updates every 2.5 seconds, so normal journal/progress updates appear without a rebuild.

Each exercise replays a context-complete `Questions asked this session` entry with its session title, `Working on` context, original prompt, pre-answer code, Liam's old answer, technical answer, example or analogy, and related notes. Type a new answer, reveal the journal answer to compare them side by side, then rate the recall.

Historical questions without an explicit `Prompt context` remain in the ordered queue with a visible `historical` label and a context-incomplete notice. The journal workflow now records `Prompt context`, `Prompt code`, and `Liam's answer` for every new question, so future cards are context-complete automatically. Corrected or completed code stays behind Reveal; the prompt shows only the original code Liam was asked to reason about.

Typed answers and ratings are deliberately kept only in browser memory. Once a layout wins, record the decision below, replace this prototype with the real site, and remove the losing variants and switcher.

## Verdict

Pending Liam's review.
