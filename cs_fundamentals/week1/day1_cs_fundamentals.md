# CS Fundamentals: Day 1 -- What Is a Program?

> **Companion to**: LEARNING_PLAN.md Day 1 -- Hello Rust & Cargo
> **Rust concepts today**: `cargo new`, `cargo build`, `cargo run`, `fn main()`, `println!`
> **CS prerequisites unlocked**: Compiled vs. interpreted languages, the compilation pipeline, entry points, process lifecycle, memory regions of a running program

---

## The Bridge (Why Now?)

Today you'll type `cargo new hello_world`, write `fn main() { println!("Hello, world!"); }`,
and run it with `cargo run`. It works. But what actually happened between you pressing Enter
and seeing text on your screen?

You wrote *source code*. Cargo invoked the Rust *compiler*. The compiler produced a *binary
executable*. Your operating system loaded that binary into *memory* as a *process*. The
process executed `main()`, which called into the standard library to print text, and then
the process exited.

Every one of those italicized words is a CS concept. Here's what each one means.

---

## Core Concepts

---

### 1. Source Code vs. Machine Code

**One-Sentence Summary**: Source code is what you write; machine code is what the CPU runs.

**Mental Model**: Think of source code as a recipe written in English. The CPU is a chef
who only speaks binary. The compiler is the translator.

**How It Works**

Your `.rs` file is a text file. It contains Rust syntax that is meaningful to you but
meaningless to a CPU. The CPU understands only *machine instructions* -- sequences of
bytes that map to operations like "load this value from memory" or "add these two numbers."

The gap between your Rust code and machine code is large. The Rust compiler (`rustc`,
invoked by `cargo build`) bridges it in several stages:

```
  main.rs          Parsing       Type        Borrow       Optimization     Code Gen       hello_world
 (text file) ───►  & Lexing  ──► Checking ──► Checking ──►  (LLVM)     ──► (machine  ──► (binary
                                                                           code)         executable)
```

<details>
<summary>C pipeline comparison (Clang / GCC)</summary>

```
  main.c           Preprocess    Parsing       Type         Optimization     Code Gen       a.out
 (text file) ───►  (cpp)      ──► & Lexing ──► Checking ──►  (LLVM/GCC) ──► (machine  ──► (binary
                                                                           code)         executable)
```

**How this differs from Rust**

- **Preprocessor**: C expands `#include`, `#define`, `#if`, … *before* the parser sees the file; Rust has no separate C-style preprocessor (macros work differently).
- **Borrow checker**: Only `rustc` has this stage; C compilers do not — memory correctness is your responsibility (bugs may be undefined behavior).
- **LLVM vs GCC**: **Clang** lowers through LLVM like `rustc`, so optimize/codegen can look similar; **GCC** uses its own optimizer and instruction selectors instead of LLVM.

</details>

After `cargo build`, look in `target/debug/`. There's a file with your project name. That
file contains machine code your CPU can execute directly. No interpreter needed at runtime.

**Rust Connection**: When `cargo build` succeeds, the binary in `target/debug/` is a
standalone program. It doesn't need Rust installed to run. This is what "compiled language"
means -- all translation happens before execution, not during.

**Common Misconceptions**:

- *"Compiled means fast, interpreted means slow."* Not exactly. Compilation allows
  optimizations that are hard to do at runtime, but the real distinction is *when*
  translation happens: before execution (compiled) or during (interpreted).
- *"`cargo run` is like an interpreter."* No. `cargo run` = `cargo build` + execute
  the binary. It's a convenience shortcut, not interpretation.

---

### 2. What Is `main()`?

**One-Sentence Summary**: `main()` is the agreed-upon starting point -- where the operating
system hands control to your code.

**Mental Model**: Imagine a theater production. The OS is the stage manager. Your program is
the script. `main()` is the opening line -- the stage manager always starts there, regardless
of what other scenes (functions) exist in the script.

**How It Works**

Every executable program needs an *entry point* -- a single function where execution begins.
In Rust (and C, C++, Java, Go, and many others), that entry point is a function called `main`.

When you write:

```rust
fn main() {
    println!("Hello, world!");
}
```

You're defining the one function the operating system will call when it starts your program.
If you define other functions (`fn greet()`, `fn calculate()`), they only run if `main()`
calls them directly or indirectly.

This isn't arbitrary convention. The operating system's program loader is built to look for
a specific symbol in your binary. Rust's compiler and linker ensure that your `fn main()`
becomes that symbol.

**Rust Connection**: Rust's `main()` has a specific signature: no parameters, returns `()` by
default (or `Result` for error handling later). The compiler enforces this. You cannot name
it `start()` or `run()` and expect it to work.

**Common Misconceptions**:

- *"The first line of the file runs first."* No. Only `main()` runs. You could put
  `fn helper()` above `fn main()` in the file -- `helper` won't run unless `main` calls it.

---

### 3. What Is a Process?

**One-Sentence Summary**: A process is a running instance of your program, with its own
isolated memory space managed by the operating system.

**Mental Model**: Your binary on disk is a blueprint. A process is a building constructed
from that blueprint. You can construct multiple buildings (run the program multiple times),
and each is independent.

**How It Works**

When you run `cargo run` (or execute the binary directly), the operating system:

1. **Reads the binary** from disk
2. **Allocates memory** for the new process
3. **Loads the program's code and data** into that memory
4. **Sets up the stack** (we'll define this on Day 2-3)
5. **Jumps to `main()`** -- your code starts executing
6. **When `main()` returns**, the OS reclaims all the process's memory

The process is isolated. It cannot read another process's memory. It gets its own address
space -- a range of memory addresses that belong only to it.

```
                     ┌──────────────────┐
                ┌───►│ Process 1        │
                │    │ (own memory)     │
  hello_world   │    └──────────────────┘
  (file on   ───┤    ┌──────────────────┐
   disk)        ├───►│ Process 2        │    Each process is isolated.
                │    │ (own memory)     │    They cannot read each
                │    └──────────────────┘    other's memory.
                │    ┌──────────────────┐
                └───►│ Process 3        │
                     │ (own memory)     │
                     └──────────────────┘
```

**Rust Connection**: When your Rust program panics (crashes), the process terminates and the
OS cleans up all its memory. This is why Rust panics are "safe" even though they're
undesirable -- no memory is truly leaked to the system, because the OS reclaims everything.

**Common Misconceptions**:

- *"A process is the same as a program."* A program is a file on disk. A process is that
  program *running*. One program can have multiple processes (multiple instances running).

---

### 4. Memory Regions of a Running Process

**One-Sentence Summary**: When your program runs, the OS carves its memory into distinct
regions, each with a different purpose.

**Mental Model**: Think of a process's memory like a building with designated floors. You
don't store accounting files in the cafeteria. Similarly, code goes in one region, local
variables in another, and dynamically allocated data in yet another.

**How It Works**

Every process gets memory organized roughly like this (high addresses at top):

```
  ┌─────────────────────────────────────────────┐  High addresses
  │              STACK (grows ↓)                 │
  │  Local variables, function call info         │
  │  [Day 2-3: zoom in here]                    │
  ├─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┤
  │                                             │
  │         (unused space -- stack and           │
  │          heap grow toward each other)        │
  │                                             │
  ├─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┤
  │              HEAP (grows ↑)                  │
  │  Dynamic allocations: String, Vec, etc.      │
  │  [Day 4: zoom in here]                      │
  ├─────────────────────────────────────────────┤
  │         STATIC / GLOBAL DATA                 │
  │  Constants, string literals, static vars     │
  ├─────────────────────────────────────────────┤
  │         PROGRAM CODE (TEXT)                   │
  │  Your compiled machine instructions          │
  └─────────────────────────────────────────────┘  Low addresses
```

Today, you only need the big picture:

- **Code (Text)**: The compiled machine instructions from your `.rs` files live here.
  Read-only -- the program doesn't modify its own instructions.
- **Static/Global Data**: String literals like `"Hello, world!"` live here. When you
  write `println!("Hello, world!")`, that string is embedded in this region at compile time.
- **Stack**: Where local variables and function call bookkeeping live. Fast, automatic,
  limited in size. (We'll explore this deeply on Days 2 and 3.)
- **Heap**: Where dynamically-sized data lives. Flexible, requires explicit management.
  (We'll explore this deeply on Day 4.)

**Rust Connection**: When you write `println!("Hello, world!")`, the string `"Hello, world!"`
is stored in the Static Data region of your binary. It's baked in at compile time. The
program doesn't allocate memory for it at runtime -- it's already there.

**Common Misconceptions**:

- *"All memory is the same."* It's not. The stack and heap have fundamentally different
  performance characteristics and usage patterns. Treating them as interchangeable is
  how you end up confused by Rust's ownership system later.
- *"The program allocates all its memory at startup."* The stack and heap grow and shrink
  during execution. Only the code and static data are fixed.

<details>
<summary><strong>🔍 Zoom Out: Where Does Your Process Live in the Whole Computer?</strong></summary>

The diagram above shows the memory layout *inside* your process. But your process isn't the
only thing in RAM. Here's the full picture -- your computer's physical memory, from boot to
running your Rust program:

```
  ┌──────────────────────────────────────────────────────────┐
  │                   PHYSICAL RAM (e.g. 16 GB)              │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  OS KERNEL (always resident)                       │  │
  │  │  ├─ Kernel code: scheduler, memory manager         │  │
  │  │  ├─ Device drivers: GPU, disk, network, audio      │  │
  │  │  └─ Kernel data: process table, file system cache  │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  PowerShell (your terminal)                        │  │
  │  │  Code | Static | Heap | Stack                      │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  VS Code / Cursor                                  │  │
  │  │  Code | Static | Heap | Stack                      │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  ★ YOUR RUST PROGRAM ◄ THIS IS YOU                 │  │
  │  │  Code | Static | Heap | Stack                      │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  Background apps: Spotify, Browser, Antivirus ...  │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  FREE MEMORY                                       │  │
  │  │  (available for new processes or growth)            │  │
  │  └────────────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────────────┘
```

**Key things to notice:**

1. **The OS kernel is always in memory.** It's the first thing loaded when you boot. It
   manages everything: which processes run, how memory is divided, how hardware is accessed.
   Your program never touches kernel memory directly.

2. **Every process gets its own isolated chunk.** Your Rust program, your editor, your
   terminal -- each is a separate process with its own Code/Static/Heap/Stack layout.
   They **cannot** read each other's memory. The OS enforces this isolation.

3. **Your Rust program is just one small tenant in a busy building.** A typical system has
   dozens to hundreds of processes running. Your `hello_world` binary might use a few
   kilobytes. Chrome alone might use gigabytes. They all share the same physical RAM.

4. **The OS is the landlord.** It decides:
   - Which process gets CPU time (scheduling)
   - Which memory addresses each process can use (virtual memory)
   - When to reclaim memory from terminated processes
   - When to swap memory to disk if RAM is full

5. **Virtual memory: the illusion of privacy.** Each process *thinks* it has the entire
   address space to itself. The OS and CPU translate these "virtual" addresses to real
   physical RAM locations.

```
   Your Process Sees                  Physical RAM
  ┌──────────────────┐              ┌──────────────────┐
  │ 0x0000: Code     │──── MMU ───►│ 0x1A000          │
  │ 0x1000: Static   │──── maps ──►│ 0x3F000          │
  │ 0x5000: Heap     │──── to ────►│ 0x82000          │
  │ 0xF000: Stack    │────────────►│ 0xC1000          │
  └──────────────────┘              └──────────────────┘
```

Chrome's `0x0000` maps to a completely different physical address -- no collision.

**Why this matters for Rust**: When Rust's ownership system frees memory (via `Drop`), it's
releasing memory back to the OS (or the process's allocator) so other parts of your program
-- or eventually other processes -- can use it. When you have a memory leak, you're holding
onto RAM that the rest of the system could use. On a game console with 8 GB of shared RAM,
every wasted byte is a byte your game can't use for textures or audio.

</details>

<details>
<summary><strong>🔍 Zoom Out: The Memory Hierarchy -- From CPU Registers to Disk</strong></summary>

The diagrams above show where data lives inside RAM. But RAM itself is just one level in a
**hierarchy of storage** that your computer uses. Understanding this hierarchy explains *why*
some operations are fast and others are slow -- which matters when you're choosing
between stack and heap (Day 4) or optimizing a game loop.

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  CPU REGISTERS          ~100 bytes        < 1 ns               │
  │  The CPU works directly with data here                         │
  ├─────────────────────────────────────────────────────────────────┤
  │  L1 CACHE (per core)    32-64 KB          ~1 ns    (4 cycles)  │
  │  (32KB data + 32KB instructions)                               │
  ├─────────────────────────────────────────────────────────────────┤
  │  L2 CACHE (per core)    256 KB - 1 MB     ~3-5 ns  (12 cyc.)  │
  ├─────────────────────────────────────────────────────────────────┤
  │  L3 CACHE (shared)      8-64 MB           ~10-15 ns (40 cyc.) │
  ├─────────────────────────────────────────────────────────────────┤
  │  RAM (main memory)      8-64 GB           ~50-100 ns (200 c.) │
  │  Your process lives here (stack, heap, code)                   │
  ├─────────────────────────────────────────────────────────────────┤
  │  SSD / NVMe             256 GB - 4 TB     ~10-100 μs          │
  │  Your binary lives here before loading       (100× RAM)        │
  ├─────────────────────────────────────────────────────────────────┤
  │  HDD (if any)           1-10 TB           ~1-10 ms             │
  │  Mechanical: spinning platters               (10,000× RAM)     │
  └─────────────────────────────────────────────────────────────────┘
        ▲ FAST, SMALL, EXPENSIVE                SLOW, LARGE, CHEAP ▼
```

**The speed gaps are staggering.** To put this in human terms:

```
  If a CPU register access takes     1 second    (picking up a book on your desk)
  Then L1 cache takes                3 seconds   (opening a desk drawer)
  Then L2 cache takes               10 seconds   (walking to a bookshelf)
  Then L3 cache takes               30 seconds   (walking to the next room)
  Then RAM takes                     3 minutes   (driving to the library)
  Then SSD takes                     3 hours     (ordering the book online)
  Then HDD takes                     3 days      (requesting it via inter-library loan)
```

**How the hierarchy works in practice:**

When the CPU needs to read `let x: i32 = 5`:

1. It checks the **registers** -- is `x` already loaded? (instant)
2. If not, it checks **L1 cache** -- was `x` accessed recently? (~1 ns)
3. If not, **L2** → **L3** → **RAM** -- each level slower, each level larger
4. When found, the data is **copied upward** through each level (so next access is faster)

This is automatic and invisible to your code. You don't choose which cache level to use.
But the *patterns* of your memory access determine whether the CPU finds data in cache
(a **cache hit**, fast) or has to go all the way to RAM (a **cache miss**, slow).

**Where each part of your Rust program lives in the hierarchy:**

```
  Hierarchy Level       What Lives There
  ─────────────────     ──────────────────────────────────────────────
  Registers             Active variables during  x + y
  L1-L3 Cache           Recently accessed data. Stack frame almost
                        always in L1/L2.
  RAM                   Full process memory: code, static, stack, heap
  Disk (SSD)            Your binary before loading. Swap space if
                        RAM is full.
```

**Why this matters for the rest of your Rust journey:**

- **Day 2-3 (Stack):** Stack variables are accessed in a tight, predictable pattern (push,
  use, pop). This pattern is *cache-friendly* -- the top of the stack almost always sits in
  L1 cache. That's a big reason why stack access is fast.

- **Day 4 (Heap):** Heap allocations can be scattered across memory. Following a pointer
  from the stack to the heap might jump to an address far away, causing a **cache miss**
  (the CPU has to fetch from RAM instead of cache). This is why "pointer chasing" is slower.

- **Games (later):** A game running at 60 FPS has 16ms per frame. A cache miss costs ~100ns.
  If your game loop accesses 10,000 scattered heap objects, that's potentially 1ms of cache
  misses alone -- 6% of your frame budget, just from poor memory layout. This is why game
  engines use data-oriented design. Rust's ownership model encourages contiguous,
  cache-friendly data, which aligns with this -- though it wasn't designed for that reason.

</details>

---

## Putting It Together

Let's trace exactly what happens when you do `cargo run` on your Day 1 project:

**Step 1: You write the code**

```rust
fn main() {
    println!("Hello, world!");
}
```

This is a text file: `src/main.rs`.

**Step 2: `cargo build` compiles it**

```
  main.rs ──► rustc ──► parsing ──► type check ──► LLVM ──► target/debug/hello_world
```

The string `"Hello, world!"` is embedded in the binary's static data section.

**Step 3: The OS loads the binary as a process**

```
  ┌─────────────────────────────────────────┐
  │  Stack                                  │
  │  main()'s frame — almost empty          │
  ├─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┤
  │  (free space)                           │
  ├─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┤
  │  Heap — empty (no dynamic allocations)  │
  ├─────────────────────────────────────────┤
  │  Static Data                            │
  │  contains: "Hello, world!"              │
  ├─────────────────────────────────────────┤
  │  Program Code                           │
  │  main() + println! instructions         │
  └─────────────────────────────────────────┘
```

**Step 4: Execution enters `main()`**

The OS jumps to `main()`. The `println!` macro expands to code that writes the string
literal to standard output (your terminal). `main()` returns. The OS destroys the process
and reclaims all memory.

**Step 5: You see "Hello, world!" on the screen**

Total lifetime of this process: a few milliseconds.

---

## Check Your Understanding

**Q1**: After `cargo build`, could you copy the binary to another machine (same OS and
architecture) and run it without Rust installed?

**A1**: Yes. The binary is standalone machine code. It doesn't need the Rust toolchain
at runtime. This is a property of compiled languages.

---

**Q2**: If you write `fn greet() { println!("Hi!"); }` above `fn main()` but never call
`greet()` from `main()`, does "Hi!" get printed?

**A2**: No. Only `main()` is called by the OS. Other functions only execute if called
(directly or indirectly) from `main()`.

---

**Q3**: Where does the string `"Hello, world!"` physically reside when your program is
running?

**A3**: In the static/global data region of the process's memory. It was embedded in the
binary at compile time, not allocated at runtime.

---

**Q4**: What's the difference between your program (the binary file on disk) and the
process that runs when you execute it?

**A4**: The program is a static file containing machine code. A process is a running
instance of that program, with its own memory space, managed by the OS. You can run the
same program multiple times, creating multiple independent processes.

---

## Vocabulary Added

| Term | Definition | First Seen |
|------|------------|------------|
| **Source code** | Human-readable text (`.rs` files) that defines a program | Day 1 |
| **Machine code** | Binary instructions the CPU executes directly | Day 1 |
| **Compiler** | A tool that translates source code into machine code | Day 1 |
| **Binary / Executable** | A file containing machine code, ready to run | Day 1 |
| **Entry point** | The function where execution begins (`main()`) | Day 1 |
| **Process** | A running instance of a program, with its own memory space | Day 1 |
| **Memory region** | A designated area of a process's memory (code, static, stack, heap) | Day 1 |
| **Stack** | Memory region for local variables and call tracking (details Day 2-3) | Day 1 |
| **Heap** | Memory region for dynamic allocations (details Day 4) | Day 1 |
| **Static data** | Data embedded in the binary at compile time (string literals, etc.) | Day 1 |
| **Memory hierarchy** | The layered system of storage from CPU registers (fastest) through caches and RAM to disk (slowest) | Day 1 |
| **CPU register** | A tiny, ultra-fast storage slot inside the CPU itself (~100 bytes total) | Day 1 |
| **CPU cache (L1/L2/L3)** | Small, fast memory between the CPU and RAM that stores recently accessed data | Day 1 |
| **Cache hit** | When the CPU finds requested data in cache (fast) | Day 1 |
| **Cache miss** | When the CPU must fetch data from RAM because it's not in cache (slow) | Day 1 |
| **Virtual memory** | The OS's illusion giving each process its own private address space, mapped to physical RAM | Day 1 |
