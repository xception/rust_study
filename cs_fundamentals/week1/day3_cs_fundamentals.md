# CS Fundamentals: Day 3 -- The Call Stack and Stack Traces

> **Companion to**: LEARNING_PLAN.md Day 3 -- Functions and Control Flow
> **Rust concepts today**: `fn`, parameters, return values, statements vs expressions, `if/else`, `loop`, `while`, `for`
> **CS prerequisites unlocked**: The call stack, stack frames in detail, how function calls push/pop frames, reading a stack trace, expressions vs statements as evaluation model

---

## The Bridge (Why Now?)

Today you'll write multiple functions that call each other:

```rust
fn roll_attack(strength: u32) -> u32 { /* ... */ }
fn apply_damage(health: &mut i32, damage: u32) { /* ... */ }
fn main() {
    let damage = roll_attack(15);
    let mut hp = 100;
    apply_damage(&mut hp, damage);
}
```

When `main()` calls `roll_attack()`, and `roll_attack()` does its work and returns --
*what actually happens in memory?* Where do the parameters go? Where does the return value
come from? And when your program crashes inside a deeply nested function, why does the error
show a *stack trace* with a list of function names?

All of this is the **call stack** in action. Today we learn how it works.

---

## Core Concepts

---

### 1. The Call Stack

**One-Sentence Summary**: The call stack is a region of memory that tracks which functions
are currently executing and stores their local data.

**Mental Model**: Imagine a stack of sticky notes. Every time you call a function, you
write its name and variables on a new note and put it on top. When the function returns,
you peel off the top note. You always work with the note on top.

**How It Works**

From [Day 2](day2_cs_fundamentals.md), you know that local variables live on the stack.
Now we'll see that the stack isn't just a dumping ground for variables -- it's a structured
record of the entire call chain.

When your program starts:
1. The OS allocates a block of memory for the stack (typically 1-8 MB)
2. A **stack pointer** (a CPU register) tracks the "top" of the stack
3. `main()` gets a stack frame pushed onto the stack

Every function call pushes a new frame. Every return pops it.

**Before `main()` calls `roll_attack(15)`:**

```
  ┌─────────────────────────────────┐
  │  CALL STACK                     │
  ├─────────────────────────────────┤
  │  main()  ◄ active               │
  │    hp  = 100                    │
  │    dmg = ?                      │
  └─────────────────────────────────┘
```

**During `roll_attack(15)` execution:**

```
  ┌─────────────────────────────────┐
  │  CALL STACK                     │
  ├─────────────────────────────────┤
  │  roll_attack()  ◄ active        │
  │    strength = 15                │
  │    base     = 7                 │
  ├─────────────────────────────────┤
  │  main()                         │
  │    hp  = 100                    │
  │    dmg = ?                      │
  └─────────────────────────────────┘
```

**After `roll_attack` returns 10:**

```
  ┌─────────────────────────────────┐
  │  CALL STACK                     │
  ├─────────────────────────────────┤
  │  main()  ◄ active again         │
  │    hp  = 100                    │
  │    dmg = 10                     │
  └─────────────────────────────────┘
```

The stack frame for `roll_attack` is gone. Its memory is reclaimed. The variables
`strength` and `base` no longer exist.

**Rust Connection**: This is why you can't return a reference to a local variable in Rust:

```rust
fn broken() -> &i32 {
    let x = 42;
    &x  // ERROR: x lives on the stack frame of broken(), which is about to be destroyed
}
```

The compiler refuses because `x` will be gone the instant `broken()` returns. The
reference would point to freed memory. This is Rust preventing a *dangling pointer* --
a reference to memory that no longer holds what you think it holds.

**Common Misconceptions**:

- *"Function calls are free."* Each call has a cost: pushing a frame, copying parameters,
  storing the return address, popping the frame. It's small, but in a tight game loop
  running 60 times per second, it can matter.
- *"Recursive functions are just functions."* They are, but each recursive call pushes
  another frame. Call yourself 100,000 times and you'll run out of stack space
  ("stack overflow").

---

### 2. Anatomy of a Stack Frame

**One-Sentence Summary**: A stack frame contains everything a function needs to execute:
its parameters, local variables, and bookkeeping data like where to return to.

**Mental Model**: Think of a stack frame as a desk assigned to an employee. It has their
paperwork (parameters), their scratch paper (local variables), and a sticky note saying
"when done, bring results back to Room 305" (return address).

**How It Works**

A typical stack frame contains:

```
  ┌────────────────────────────────────────────────┐
  │  Stack Frame for roll_attack()                 │
  ├────────────────────────────────────────────────┤
  │  Return Address                                │
  │    Resume at line X of main()                  │
  ├────────────────────────────────────────────────┤
  │  Saved Frame Pointer                           │
  │    Restore caller's frame                      │
  ├────────────────────────────────────────────────┤
  │  Parameter: strength = 15                      │
  │    Copied from caller                          │
  ├────────────────────────────────────────────────┤
  │  Local: base = 7                               │
  │    Created during execution                    │
  ├────────────────────────────────────────────────┤
  │  Return Value staging                          │
  │    Where result (10) is placed                 │
  └────────────────────────────────────────────────┘
```

When `main()` calls `roll_attack(15)`:

1. **Push the return address**: "After `roll_attack` finishes, continue at line X of `main`"
2. **Push the parameters**: Copy the value `15` into the new frame's `strength` slot
3. **Adjust the stack pointer**: Now the CPU knows the new "top" of the stack
4. **Execute the function body**: Create `base`, compute `base + 3`
5. **Place the return value**: Put `10` where the caller can find it
6. **Pop the frame**: Restore the stack pointer to where it was before the call
7. **Jump to the return address**: Continue executing `main()` after the call site

All of this happens in nanoseconds. The CPU has dedicated hardware support for it.

**Rust Connection**: When you write:

```rust
fn apply_damage(health: &mut i32, damage: u32) {
    *health -= damage as i32;
}
```

The parameter `health` is a reference (`&mut i32`). What's actually in the stack frame is
the *memory address* of the original `i32` -- not a copy of the value. We'll explore this
in depth on Day 5, but notice: the reference (8 bytes on a 64-bit system) lives on
`apply_damage`'s stack frame, while the actual health value lives on `main()`'s frame.

**Common Misconceptions**:

- *"Parameters are the original variables."* No, they're copies (for simple types) or
  copies of addresses (for references). Modifying a `u32` parameter inside a function
  doesn't affect the caller's variable.

---

### 3. Nested Calls and the Stack in Action

**One-Sentence Summary**: When functions call functions that call functions, the stack
grows deeper with each call and unwinds as each returns.

**How It Works**

Let's trace the full combat round from the Day 3 exercise:

```rust
fn roll_attack(strength: u32) -> u32 {
    let base = strength / 2;
    base + 3
}

fn apply_damage(health: &mut i32, damage: u32) {
    *health -= damage as i32;
}

fn combat_round(player_hp: &mut i32, player_str: u32) {
    let damage = roll_attack(player_str);
    apply_damage(player_hp, damage);
}

fn main() {
    let mut hp: i32 = 100;
    combat_round(&mut hp, 15);
}
```

**The stack at maximum depth** (when `roll_attack` is executing):

```
  ┌─────────────────────────────────────────────┐
  │  CALL STACK (max depth)                     │
  ├─────────────────────────────────────────────┤
  │  roll_attack()  ◄ current                   │
  │    strength: u32 = 15                       │
  │    base:     u32 = 7                        │
  │    return → combat_round                    │
  ├─────────────────────────────────────────────┤
  │  combat_round()                             │
  │    player_hp:  &mut i32 = addr of main.hp   │
  │    player_str: u32 = 15                     │
  │    damage:     u32 = (not yet assigned)     │
  │    return → main                            │
  ├─────────────────────────────────────────────┤
  │  main()                                     │
  │    hp: i32 = 100                            │
  └─────────────────────────────────────────────┘
```

**After `roll_attack` returns 10:**

```
  ┌─────────────────────────────────────────────┐
  │  CALL STACK                                 │
  ├─────────────────────────────────────────────┤
  │  combat_round()  ◄ current                  │
  │    player_hp:  &mut i32 = addr of main.hp   │
  │    damage:     u32 = 10                     │
  │    return → main                            │
  ├─────────────────────────────────────────────┤
  │  main()                                     │
  │    hp: i32 = 100                            │
  └─────────────────────────────────────────────┘
```

**After `apply_damage` runs and returns** (hp was modified via reference):

```
  ┌─────────────────────────────────────────────┐
  │  CALL STACK                                 │
  ├─────────────────────────────────────────────┤
  │  main()  ◄ current                          │
  │    hp: i32 = 90  ◄ modified!                │
  └─────────────────────────────────────────────┘
```

Notice: `hp` in `main()`'s frame was changed even though `apply_damage`'s frame is gone.
That's because `apply_damage` received a *reference* (the address of `hp`), not a copy.
It reached through the reference to modify the original value.

---

### 4. Stack Traces: Reading a Crash Report

**One-Sentence Summary**: A stack trace is a snapshot of the call stack at the moment
something went wrong, listing every function that was active from innermost to outermost.

**Mental Model**: If the call stack is a stack of sticky notes, a stack trace is a
photograph of the entire stack taken at the moment of failure. It tells you not just
*where* things went wrong, but the *chain of calls that led there*.

**How It Works**

When a Rust program panics, you see something like:

```
thread 'main' panicked at 'attempt to subtract with overflow', src/main.rs:8:5
stack backtrace:
   0: std::panicking::begin_panic
   1: game::apply_damage
             at ./src/main.rs:8:5
   2: game::combat_round
             at ./src/main.rs:13:5
   3: game::main
             at ./src/main.rs:18:5
```

Reading this bottom-to-top:

```
  Frame 3:  main (line 18)              ← called combat_round
  Frame 2:  combat_round (line 13)      ← called apply_damage
  Frame 1:  apply_damage (line 8)       ← the function with the bug
  Frame 0:  std::panicking::begin_panic ← where the panic fired
```

The trace tells you the full story:

- `main` called `combat_round` at line 18
- `combat_round` called `apply_damage` at line 13
- `apply_damage` tried to subtract with overflow at line 8
- The subtraction failed, triggering `begin_panic`

**How to read a stack trace (workflow)**:

1. **Start at the top** -- the immediate cause of the error
2. **Find your code** -- skip standard library frames (`std::*`)
3. **Read the file and line number** -- go to that exact line
4. **Work down the trace** -- understand *why* that function was called with those inputs
5. **The bug is usually 1-2 levels above the crash site** -- the crasher was given bad data

**Rust Connection**: Rust includes an environment variable `RUST_BACKTRACE=1` that enables
full stack traces. When you panic without a trace, set it:

```
RUST_BACKTRACE=1 cargo run
```

**Common Misconceptions**:

- *"The error is always at the top of the stack trace."* The *symptom* is at the top, but
  the *cause* is often a few frames down, where bad data was created and passed along.
- *"Stack traces are only for crashes."* You can capture them at any time for debugging
  (though in Rust this requires the `backtrace` crate or the nightly `std::backtrace`).

---

### 5. Expressions vs. Statements

**One-Sentence Summary**: An expression evaluates to a value; a statement performs an
action but produces no value.

**Mental Model**: An expression is a question that has an answer ("what is 2 + 3?" → 5).
A statement is a command ("set x to 5"). Questions produce results. Commands produce effects.

**How It Works**

This distinction matters in Rust more than most languages because Rust is
expression-oriented. Almost everything is an expression:

| Expressions (produce a value) | Statements (produce no value) |
|-------------------------------|-------------------------------|
| `5` | `let x = 5;` |
| `x + 3` | (that's basically it in Rust) |
| `{ let a = 1; a + 2 }` | |
| `if condition { 10 } else { 20 }` | |
| `match x { 1 => "one", _ => "other" }` | |

The curly-brace block `{ let a = 1; a + 2 }` is an expression that evaluates to `3`.
The `if` expression evaluates to either `10` or `20`.

Why this matters at the CS level: the CPU is always computing values (arithmetic, comparisons,
memory reads). An expression-oriented language maps more directly to what the CPU does.
Statements are the exception, not the rule.

**Rust Connection**: This is why Rust functions return the last expression without `return`:

```rust
fn double(x: i32) -> i32 {
    x * 2  // no semicolon = this is the expression the block evaluates to
}
```

Adding a semicolon turns an expression into a statement:

```rust
fn broken(x: i32) -> i32 {
    x * 2;  // semicolon makes this a statement that returns ()
    // ERROR: expected i32, found ()
}
```

The semicolon discards the expression's value. Rust's compiler error "expected `i32`, found
`()`" is saying: "the last thing in this block is a statement (which produces `()`), but
you promised to return an `i32`."

**Common Misconceptions**:

- *"`return` is the only way to return a value."* In Rust, the last expression in a
  function body is implicitly the return value. Explicit `return` is for early exits.

---

## Putting It Together

Let's trace the Day 3 combat exercise through the call stack:

```rust
fn roll_attack(strength: u32) -> u32 {
    let base = strength / 2;
    base + 3
}

fn apply_damage(health: &mut i32, damage: u32) {
    *health -= damage as i32;
}

fn main() {
    let mut hp: i32 = 100;
    let strength: u32 = 15;
    let damage = roll_attack(strength);
    apply_damage(&mut hp, damage);
    // hp is now 90
}
```

**Step-by-step execution trace:**

```
  Step 1: OS calls main()
  ─────────────────────────────────────────────────
  Stack: [ main | hp=100, strength=15 ]

  Step 2: main() calls roll_attack(15)
  ─────────────────────────────────────────────────
  Stack: [ roll_attack | strength=15 ]
         [ main        | hp=100, strength=15 ]

  Step 3: roll_attack() computes base=7, returns 10
  ─────────────────────────────────────────────────
  Stack: [ main | hp=100, strength=15, damage=10 ]
         (roll_attack's frame is gone)

  Step 4: main() calls apply_damage(&mut hp, 10)
  ─────────────────────────────────────────────────
  Stack: [ apply_damage | health=&hp, damage=10 ]
         [ main         | hp=100, strength=15, damage=10 ]

  Step 5: apply_damage() writes *health -= 10
  ─────────────────────────────────────────────────
  Stack: [ apply_damage | health=&hp, damage=10 ]
         [ main         | hp=90 ← modified via reference ]

  Step 6: apply_damage() returns
  ─────────────────────────────────────────────────
  Stack: [ main | hp=90, strength=15, damage=10 ]

  Step 7: main() returns → process exits
  ─────────────────────────────────────────────────
  Stack: (empty — OS reclaims all memory)
```

At step 5, if `hp` were `0` and we subtracted `10`, an integer underflow would occur. In
debug mode, Rust panics, and you'd see a stack trace showing: `apply_damage` ← `main`.

---

## Check Your Understanding

**Q1**: How many stack frames exist simultaneously at the deepest point of this program?

```rust
fn a() { b(); }
fn b() { c(); }
fn c() { println!("deep"); }
fn main() { a(); }
```

**A1**: Four: `main` → `a` → `b` → `c`. (Plus internal frames for `println!`, but those
are in the standard library.)

---

**Q2**: In the stack trace below, where should you look first to find the bug?

```
   0: core::panicking::panic
   1: game::calculate_xp
             at ./src/main.rs:22:9
   2: game::defeat_enemy
             at ./src/main.rs:15:5
   3: game::main
             at ./src/main.rs:8:5
```

**A2**: Start at frame 1 (`calculate_xp`, line 22) -- that's where the panic occurred in
your code. Then check frame 2 (`defeat_enemy`, line 15) to see what inputs were passed. The
root cause is likely in how `defeat_enemy` called `calculate_xp`.

---

**Q3**: Why does this function fail to compile?

```rust
fn add_one(x: i32) -> i32 {
    x + 1;
}
```

**A3**: The semicolon after `x + 1` turns the expression into a statement, which produces
`()`. The function signature promises `i32`, so the compiler reports a type mismatch.
Remove the semicolon and `x + 1` becomes the return expression.

---

**Q4**: After `roll_attack` returns, can any code access `roll_attack`'s `base` variable?

**A4**: No. `base` was on `roll_attack`'s stack frame, which was popped when the function
returned. The memory is gone. This is why Rust prevents returning references to local
variables.

---

## Vocabulary Added

| Term | Definition | First Seen |
|------|------------|------------|
| **Call stack** | The stack-structured memory region tracking active function calls | Day 3 |
| **Stack frame** | The block of memory for one function's parameters, locals, and return address | Day 3 |
| **Return address** | The instruction to resume at when a function finishes | Day 3 |
| **Stack pointer** | A CPU register pointing to the current top of the stack | Day 3 |
| **Stack trace / backtrace** | A snapshot of all active stack frames at a given moment (usually at a crash) | Day 3 |
| **Stack overflow** | When the call stack exceeds its size limit (often from infinite recursion) | Day 3 |
| **Expression** | Code that evaluates to a value | Day 3 |
| **Statement** | Code that performs an action but produces no value (`let x = ...;`) | Day 3 |
| **Dangling pointer** | A reference to memory that has been freed (Rust prevents this at compile time) | Day 3 |
