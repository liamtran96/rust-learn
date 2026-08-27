---
title: Fundamentals Journal - Strings And Memory
tags: [rust, journal, fundamentals]
---

# Strings And Memory

> Fundamentals topic journal. Return to [[../../journal|Journal index]].

## Entries

### 2026-08-12 — `&'static str`, owned `String`, and memory
**Working on:** Follow-up to Ch 1 Exercise #6 — `code/01-fundamentals/sign-expression/`
**What clicked:** `&'static str` is a reference to text valid for the rest of the program run; it is not the `static` item keyword and does not mean permanent disk storage. `String` owns runtime text, normally allocates heap memory, moves ownership to the caller when returned, and frees that memory when its owner is dropped.
**What didn't:** The word “static” first sounded like “saved on the computer forever,” and the distinction between reusing a literal and constructing an owned `String` needed several analogies.
**Questions asked this session:**
- **Q:** Why do we use `&'static str`?
  - **Technical answer:** `&str` is a borrowed view of UTF-8 text, and a lifetime tells Rust how long that borrow is valid. `sign` returns string literals rather than borrowing from an input, and literals are embedded in the program, so their references have the `'static` lifetime.
  - **Plain-English analogy / example:**
    ```rust
    let label: &'static str = "negative";
    let result = sign(-2);
    println!("{result}");
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`, `topics/rust/04-collections/strings.md`
- **Q:** Can you explain `&'static str` like I am five?
  - **Technical answer:** The reference `&` points to text owned somewhere else, while `'static` promises that the referenced text remains valid for the program run. The variable holding that reference can still go out of scope earlier; only the referenced literal has the long lifetime.
  - **Plain-English analogy / example:**
    ```text
    "negative" = a book kept in the program's library
    &           = a note pointing to that book
    'static     = the book stays until the library closes
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`
- **Q:** Why do we need the lifetime, and what is it for?
  - **Technical answer:** Rust uses lifetimes to reject dangling references—references that point to data already destroyed. Because `sign` has no borrowed input from which Rust could infer the output lifetime, `&'static str` explicitly states that the output points to program-lifetime data.
  - **Plain-English analogy / example:**
    ```rust
    fn valid() -> &'static str { "hello" }
    // A reference to a local String would be invalid:
    // the String would be dropped when the function ends.
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`
- **Q:** Does returning `String` mean those values are saved on my computer forever?
  - **Technical answer:** No. `String` is owned text, usually backed by heap memory allocated while the process runs; returning it moves ownership to the caller. Rust frees that allocation when the final owner leaves scope, and the operating system reclaims process memory when the program exits.
  - **Plain-English analogy / example:**
    ```rust
    {
        let label = String::from("negative");
        println!("{label}");
    } // label is dropped and its heap allocation is freed
    ```
  - **See also:** `topics/rust/04-collections/strings.md`, `topics/rust/02-ownership/ownership.md`
- **Q:** Can using `String` affect performance?
  - **Technical answer:** Constructing a `String` from a literal normally allocates heap memory, copies the bytes, and later deallocates them. Returning `&'static str` reuses an existing literal and returns only a reference, so the difference can matter in a hot loop but is negligible for one small call.
  - **Plain-English analogy / example:**
    ```rust
    for n in 0..1_000_000 {
        let label = sign(n); // reuse one of the existing literals
        std::hint::black_box(label);
    }
    ```
  - **See also:** `topics/rust/04-collections/strings.md`
- **Q:** What is a real-life example of the performance difference?
  - **Technical answer:** Reusing `&'static str` is like pointing to one of three permanent restaurant signs; creating a `String` is like printing a fresh paper sign for every customer and throwing it away afterward. The extra work is invisible for a few customers but wasteful for millions.
  - **Plain-English analogy / example:**
    ```text
    &'static str × 1,000,000 → point at existing signs
    String × 1,000,000       → print 1,000,000 new signs
    Same message; different amount of work.
    ```
  - **See also:** `topics/rust/04-collections/strings.md`
**Question to answer later:** How does Rust connect a returned `&str` lifetime to an input `&str` when the text is not a literal?
**Next:** Finish the iterator-`map` version of FizzBuzz; revisit named lifetimes during Phase 2 ownership.

### 2026-08-12 — Measuring `&'static str` versus `String`
**Working on:** Performance follow-up — `code/01-fundamentals/sign-expression/examples/performance.rs`, `benches/sign_performance.rs`, and `borrowed-vs-owned.html`
**What clicked:** A fair microbenchmark measures many calls in an optimized build, hides predictable inputs and unused outputs from the optimizer, and reports an average or statistical estimate rather than timing one tiny call. Returning `String` includes allocation, copying, ownership, and eventual deallocation; returning a literal reference reuses existing program data.
**What didn't:** The first handwritten median selected the upper-middle result for an even number of samples; it was corrected to average the two middle durations. The exact speed ratio initially sounded universal, but it is local to the machine, compiler, benchmark design, and current system load.
**Questions asked this session:**
- **Q:** Can we calculate the performance of both return types and see and run the code?
  - **Technical answer:** A separate release-mode example now calls both implementations repeatedly and measures elapsed wall-clock time with `Instant`. It uses the same changing inputs for both cases and reports total duration, nanoseconds per call, and their ratio without altering the finished exercise binary.
  - **Plain-English analogy / example:**
    ```text
    cargo run --release --example performance
    &'static str → point to an existing sign
    String       → make and discard a new paper sign
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** How do you measure it?
  - **Technical answer:** `Instant::now()` records the starting clock point and `start.elapsed()` returns a `Duration`, an amount of elapsed time. The benchmark measures the whole loop and divides its total nanoseconds by the number of calls; it does not time each call separately.
  - **Plain-English analogy / example:**
    ```rust
    let start = Instant::now();
    for _ in 0..10_000_000 { do_work(); }
    let total = start.elapsed();
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** What are `black_box`, `Instant`, and `Duration`?
  - **Technical answer:** `black_box` is an optimizer hint that makes inputs look unpredictable and outputs potentially useful, discouraging removal of the measured work. `Instant` is a monotonic stopwatch point, while `Duration` stores the amount of time between two points.
  - **Plain-English analogy / example:**
    ```rust
    let start = Instant::now();       // press stopwatch start
    black_box(sign(black_box(-2)));   // make Rust really do the work
    let elapsed: Duration = start.elapsed();
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** How do `median` and `nanos_per_call` work, and are you sure?
  - **Technical answer:** The median sorts repeated durations and selects the center, reducing the effect of unusually slow noisy samples. For an even count, the correct implementation averages the two center values; `nanos_per_call` converts the total to nanoseconds and divides by the iteration count to calculate average throughput.
  - **Plain-English analogy / example:**
    ```text
    sorted: 9ms, 10ms, 11ms, 12ms
    median: (10ms + 11ms) / 2 = 10.5ms
    ns/call: total nanoseconds / number of calls
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** Is there another way to measure this?
  - **Technical answer:** Criterion is a statistics-driven microbenchmarking library that automatically warms up, chooses iteration counts, collects samples, estimates a confidence interval, and identifies outliers. Allocation counters and profilers answer different questions, such as how many heap allocations occurred or which CPU instructions were expensive.
  - **Plain-English analogy / example:**
    ```text
    cargo bench --bench sign_performance
    manual Instant → homemade stopwatch
    Criterion      → repeated lab measurement
    ```
  - **See also:** `code/01-fundamentals/sign-expression/benches/sign_performance.rs`
- **Q:** Is the Criterion comparison definitely measuring the right thing?
  - **Technical answer:** Yes, for the question “what does obtaining and then discarding each result cost?” Criterion's `iter` loop includes destruction of the returned value. That fairly includes allocation, copying, and freeing for `String`, while dropping a borrowed reference has essentially no cleanup work; the measured ratio remains a local estimate rather than a universal constant.
  - **Plain-English analogy / example:**
    ```text
    borrowed result: choose → point → discard pointer
    owned result:    allocate → copy → own → free
    both paths include their complete cleanup
    ```
  - **See also:** `code/01-fundamentals/sign-expression/benches/sign_performance.rs`
- **Q:** Can you show both cases visually so a five-year-old can understand?
  - **Technical answer:** The interactive comparison shows borrowing as pointing to one reusable program sign and ownership as producing a new paper sign for each call. It can switch between one and one million calls and step through preparation, return, and cleanup.
  - **Plain-English analogy / example:**
    ```text
    Borrow: “Look at the sign already on the wall.”
    Own:    “Print a new sign, give it away, then recycle it.”
    ```
  - **See also:** `code/01-fundamentals/sign-expression/borrowed-vs-owned.html`
- **Q:** What is the heap, how does it work, and can it be added visually?
  - **Technical answer:** The heap is a region of runtime memory used for dynamically sized or long-lived allocations. A `String` owns a small handle containing a pointer, length, and capacity; its pointer leads to bytes on the heap, and when the `String` is dropped Rust returns that space to the allocator for reuse.
  - **Plain-English analogy / example:**
    ```text
    allocate → reserve an empty shelf
    use      → String owns the shelf address
    drop     → empty the shelf so it can be reused
    ```
  - **See also:** `code/01-fundamentals/sign-expression/borrowed-vs-owned.html`, `topics/rust/02-ownership/ownership.md`
**Question to answer later:** Where does the `String` handle itself live, and how are its pointer, length, and capacity represented?
**Next:** Finish the iterator-`map` version of FizzBuzz; revisit stack-versus-heap and ownership in Phase 2.

