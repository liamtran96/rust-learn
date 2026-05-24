// Scratch crate: feel the difference between local mut, const, static mut, and static + Mutex.
// Read the comments — that's the lesson.

use std::sync::Mutex;

// ─── Demo 1 ────────────────────────────────────────────────────────────────
// Local `mut` resets every call. Counter is useless across calls.
fn greet_local() {
    let mut counter: u32 = 0;
    counter += 1;
    println!("[local]      hello #{counter}");
}

// ─── Demo 2 ────────────────────────────────────────────────────────────────
// `const` is a VALUE, not a variable. You cannot assign to it.
// Uncomment the line inside greet_const() to see the compile error.
const _CONST_COUNTER: u32 = 0;
fn greet_const() {
    // _CONST_COUNTER += 1;  // ❌ error[E0070]: invalid left-hand side of assignment
    println!("[const]      const can't be mutated (try uncommenting the line)");
}

// ─── Demo 3 ────────────────────────────────────────────────────────────────
// `static mut` works but every access requires `unsafe` because two threads
// could race. Don't write this in real code — shown for contrast only.
static mut UNSAFE_COUNTER: u32 = 0;
fn greet_static_mut() {
    // Edition 2024 forbids implicit refs to static mut, so we read into a
    // local through a raw pointer. This is exactly the kind of awkward
    // ceremony that makes `static + Mutex` (Demo 4) the better answer.
    unsafe {
        let ptr = &raw mut UNSAFE_COUNTER;
        *ptr += 1;
        let snapshot = *ptr;
        println!("[static mut] hello #{snapshot}  (needed unsafe block + raw ptr)");
    }
}

// ─── Demo 4 ────────────────────────────────────────────────────────────────
// THE IDIOMATIC PATTERN.
// `static` itself is immutable (we never reassign COUNTER). The mutability
// lives INSIDE the Mutex. lock() blocks if anyone else holds it; the lock
// auto-releases when `count` goes out of scope at the end of the function.
static COUNTER: Mutex<u32> = Mutex::new(0);

fn greet() {
    let mut count = COUNTER.lock().unwrap();
    *count += 1;
    println!("[mutex]      hello #{count}");
}

fn addresses() {
    const C: u32 = 42;
    static S: u32 = 42;

    println!("&C first  = {:p}", &C);
    println!("&C second = {:p}", &C);   // may differ from the first!
    println!("&S first  = {:p}", &S);
    println!("&S second = {:p}", &S);   // always equal to the first
}

fn main() {
    println!("--- Demo 1: local mut (broken — always prints 1) ---");
    greet_local();
    greet_local();
    greet_local();

    println!("\n--- Demo 2: const (won't mutate at all) ---");
    greet_const();

    println!("\n--- Demo 3: static mut (works, but unsafe) ---");
    greet_static_mut();
    greet_static_mut();
    greet_static_mut();

    println!("\n--- Demo 4: static + Mutex (the right way) ---");
    greet();
    greet();
    greet();

    fn addresses() {
    const C: u32 = 42;
    static S: u32 = 42;

    println!("&C first  = {:p}", &C);
    println!("&C second = {:p}", &C);   // may differ from the first!
    println!("&S first  = {:p}", &S);
    println!("&S second = {:p}", &S);   // always equal to the first
}

    println!("\n--- Demo 5: addresses of const vs static ---");
    addresses();
}
