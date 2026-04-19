# CS Fundamentals: Day 2 -- Variables, Types, and Memory

> **Companion to**: LEARNING_PLAN.md Day 2 -- Variables, Types, and Mutability
> **Rust concepts today**: `let`, `mut`, scalar types (`i32`, `u32`, `f64`, `bool`, `char`), tuples, arrays
> **CS prerequisites unlocked**: Variables as named memory locations, types as byte-width contracts, stack memory for locals, memory addresses

---

## The Bridge (Why Now?)

Today you'll write `let x: i32 = 5;` and `let mut health: i32 = 100;`. These look simple.
But when Rust says `x` is `i32`, it's making a precise contract: `x` occupies exactly
4 bytes on the stack, those bytes are interpreted as a two's complement signed integer, and
the value 5 is stored as the byte pattern `05 00 00 00`.

When you add `mut`, you're not changing where or how `health` is stored -- you're telling
the compiler that the value at that memory location is allowed to change. Without `mut`,
the compiler *refuses to let you modify it*, even though the hardware wouldn't care.

Understanding what variables really are -- named slots of memory with a specific size and
interpretation -- is necessary to understand Rust's ownership system later.

---

## Core Concepts

---

### 1. A Variable Is a Named Memory Location

**One-Sentence Summary**: A variable is a human-friendly name that refers to a specific
location in the computer's memory.

**Mental Model**: Think of memory as a massive row of numbered mailboxes. A variable is a
label you stick on one (or several) of those mailboxes so you can find it again.

**How It Works**

When you write:

```rust
let x: i32 = 5;
```

Three things happen:

1. The compiler reserves 4 bytes of space on the **stack** (because `i32` = 4 bytes)
2. The value `5` is written into those 4 bytes
3. The name `x` is associated with that location (this association exists only at compile
   time -- the running binary doesn't know the name "x")

The key insight: **the name `x` does not exist at runtime**. It's a compiler convenience.
The binary just says "read 4 bytes from stack offset 8" or similar. Variable names are for
humans, not CPUs.

```
  Source Code                          Stack Memory
  ─────────────                        ─────────────────────────────
  let x: i32 = 5;   ── compiles to ──► 0x7FF4: 05 00 00 00  (x, 4 bytes)
  let y: i32 = 10;  ── compiles to ──► 0x7FF0: 0A 00 00 00  (y, 4 bytes)
```

(Note: On x86, integers are stored in "little-endian" byte order, so `5` is stored as
`05 00 00 00`, not `00 00 00 05`. This detail isn't critical now but explains what you'll
see in a debugger.)

**Rust Connection**: When Rust says a variable is *immutable by default* (`let x = 5;`),
it means the compiler won't generate instructions that write to that memory location after
initialization. The memory location itself is just bytes -- immutability is enforced by the
compiler, not the hardware.

**Common Misconceptions**:

- *"Variables are like boxes that hold values."* Close, but misleading. A variable isn't
  a container -- it *is* a region of memory. The value isn't "inside" it; the value *is*
  the pattern of bytes at that location.
- *"Variable names exist at runtime."* In a compiled language like Rust, they don't
  (unless you compile with debug symbols, which is a separate topic).

---

### 2. Types Are Byte-Width Contracts

**One-Sentence Summary**: A type tells the compiler how many bytes to reserve and how to
interpret the bit pattern stored there.

**Mental Model**: If memory is a row of mailboxes, a type is the label on the mailbox that
says "this contains 4 letters, read them as an integer" vs. "this contains 8 letters, read
them as a decimal number."

**How It Works**

Every type in Rust has a known, fixed size (for types that live on the stack). Here are the
scalar types you'll use today:

| Type | Bytes | Range / Values |
|------|-------|----------------|
| `i8` | 1 | -128 to 127 |
| `i16` | 2 | -32,768 to 32,767 |
| `i32` | 4 | -2,147,483,648 to 2,147,483,647 |
| `i64` | 8 | much larger |
| `u8` | 1 | 0 to 255 |
| `u16` | 2 | 0 to 65,535 |
| `u32` | 4 | 0 to 4,294,967,295 |
| `u64` | 8 | much larger |
| `f32` | 4 | ~7 decimal digits precision |
| `f64` | 8 | ~15 decimal digits precision |
| `bool` | 1 | true (1) or false (0) |
| `char` | 4 | any Unicode scalar value |

The `i` prefix means signed (can be negative), `u` means unsigned (zero or positive). The
number is the bit width: `i32` = 32 bits = 4 bytes.

Why does this matter? Because when the compiler sees `let x: i32 = 5`, it knows:
- Reserve exactly 4 bytes on the stack
- The addition instruction to use is "signed 32-bit add"
- If you try to store 3 billion in it, that's an overflow (won't fit in 4 bytes as signed)

The same 4 bytes `FF FF FF FF` mean different things depending on the type.
**The bits are identical. The type determines the interpretation.**

```
              Memory: FF FF FF FF
              (same 4 bytes)
                    │
         ┌──────────┼──────────┐
         ▼          ▼          ▼
   interpret    interpret   interpret
    as i32       as u32      as f32
         │          │          │
         ▼          ▼          ▼
        -1     4,294,967,295  NaN
  (two's comp.)  (unsigned)  (IEEE 754)
```

**Rust Connection**: This is why Rust won't let you assign an `i32` to a `u32` without
an explicit cast. They're not the same type even though they're the same size. The compiler
protects you from misinterpreting the bytes.

**Common Misconceptions**:

- *"An `i32` and a `u32` are basically the same thing."* They're the same *size* but
  different *interpretations*. This distinction prevents subtle bugs.
- *"`bool` should only need 1 bit."* Logically yes, but memory is addressed by the byte.
  The smallest addressable unit is 1 byte, so `bool` uses 1 byte.

---

### 3. The Stack: Where Local Variables Live

**One-Sentence Summary**: The stack is a region of memory where local variables are stored
automatically, in a last-in-first-out order.

**Mental Model**: Imagine a cafeteria tray dispenser. You can only add or remove trays from
the top. When you call a function, you push a tray (stack frame) for its local variables.
When the function returns, you pop the tray off. You can never pull a tray from the middle.

**How It Works**

From [Day 1](day1_cs_fundamentals.md), you know the process memory has a stack region. Now
let's zoom in on what lives there.

When `main()` starts executing, the program allocates a *stack frame* for it. The stack
frame is a block of memory that holds all of `main()`'s local variables.

```rust
fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    let sum: i32 = x + y;
}
```

```
  ┌──────────────────────────────────────────┐
  │  main()'s Stack Frame — 12 bytes total   │
  ├──────────────────────────────────────────┤
  │  x:   i32 = 5       4 bytes  05 00 00 00│
  ├──────────────────────────────────────────┤
  │  y:   i32 = 10      4 bytes  0A 00 00 00│
  ├──────────────────────────────────────────┤
  │  sum: i32 = 15      4 bytes  0F 00 00 00│
  └──────────────────────────────────────────┘
```

Key properties of the stack:

| Property | Implication |
|----------|-------------|
| **Fixed-size entries** | The compiler knows the exact byte size of every local variable at compile time |
| **Automatic cleanup** | When a function returns, its stack frame is "popped" -- no manual cleanup needed |
| **Extremely fast** | Allocation = move a pointer. No searching for free space. |
| **Limited size** | Typically 1-8 MB. You can't put 1 million game entities on the stack. |
| **LIFO order** | You can't free a variable in the middle. The last frame pushed is the first popped. |

**Rust Connection**: When Rust says a variable "goes out of scope" at the closing `}`, it
means the stack frame is about to be popped. The variable's memory is reclaimed. This is why:

```rust
{
    let temp = 42;
}
// temp is gone here -- its stack space was reclaimed when } was reached
```

This scoping rule is the foundation of Rust's ownership and lifetime system.

**Common Misconceptions**:

- *"Variables are stored in some abstract 'variable table'."* No. Local variables are
  literally contiguous bytes on the stack, addressed by offset from the frame pointer.
- *"The stack is slow because it's a data structure."* The call stack isn't a linked-list
  or array -- it's a raw region of memory with a pointer to the top. "Push" just means
  subtracting from the stack pointer. It's the fastest allocation possible.

<details>
<summary><strong>🔍 Zoom Out: Where Is the Stack Inside the Computer's RAM?</strong></summary>

In [Day 1](day1_cs_fundamentals.md) we saw that your process is one of many tenants in
physical RAM, alongside the OS kernel and other programs. Now let's zoom into your process
and see exactly where the stack sits:

```
  ┌──────────────────────────────────────────────────────────┐
  │  PHYSICAL RAM                                            │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  OS Kernel                                         │  │
  │  └────────────────────────────────────────────────────┘  │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  Other Processes (browser, editor, etc.)           │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  YOUR RUST PROCESS                                 │  │
  │  │  ┌──────────────────────────────────────────────┐  │  │
  │  │  │  Stack (1-8 MB) ◄ YOU ARE HERE               │  │  │
  │  │  │  ┌──────────────────────────────────────┐    │  │  │
  │  │  │  │  main()                              │    │  │  │
  │  │  │  │  x = 5, y = 10, sum = 15             │    │  │  │
  │  │  │  └──────────────────────────────────────┘    │  │  │
  │  │  ├──────────────────────────────────────────────┤  │  │
  │  │  │  (free space)                                │  │  │
  │  │  ├──────────────────────────────────────────────┤  │  │
  │  │  │  Heap (empty today — Day 4)                  │  │  │
  │  │  ├──────────────────────────────────────────────┤  │  │
  │  │  │  Static Data  "Hello, world!"                │  │  │
  │  │  ├──────────────────────────────────────────────┤  │  │
  │  │  │  Program Code  compiled fn main()            │  │  │
  │  │  └──────────────────────────────────────────────┘  │  │
  │  └────────────────────────────────────────────────────┘  │
  │                                                          │
  │  ┌────────────────────────────────────────────────────┐  │
  │  │  Free RAM                                          │  │
  │  └────────────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────────────┘
```

**Scale perspective:** Your `let x: i32 = 5` occupies 4 bytes. Your stack is ~8 MB.
Your process might use ~50 KB total for a hello-world. Your system has 16 GB of RAM.
That's like placing a single grain of rice on a football field.

But those 4 bytes are *yours*. They're at a specific address in the stack region of your
process's virtual memory. The OS maps that virtual address to real hardware. And when your
function returns and the stack frame is popped, those 4 bytes become available for the next
function call -- no OS involvement needed, just moving the stack pointer.

**And recall the memory hierarchy** ([Day 1](day1_cs_fundamentals.md)): your stack doesn't
just live in RAM -- the top of the stack almost certainly lives in **L1 cache** (~1ns access)
because your code touches it so frequently. Stack allocation is fast for two reasons:
moving a pointer is cheap, and the data you just pushed is likely already in the fastest
memory your CPU has.

```
  Memory Hierarchy — Where the Stack Actually Lives
  ─────────────────────────────────────────────────────────
  Registers          value of x during  x + y
  L1 Cache           Your current stack frame
                     (x=5, y=10, sum=15) — almost certainly here
  L2/L3 Cache        Recent frames from other calls
  RAM                Full stack region (1-8 MB reserved)
  Disk               (only if RAM full and OS swaps — rare)
```

</details>

---

### 4. Memory Addresses (Conceptual)

**One-Sentence Summary**: Every byte in memory has a numeric address, and the CPU uses
these addresses to read and write data.

**Mental Model**: Think of memory as a hotel with numbered rooms. Room 1000, Room 1001,
Room 1002. When you say `let x: i32 = 5`, you're checking into rooms 1000-1003 (4 bytes)
and putting the value 5 in those rooms.

**How It Works**

Memory is a flat sequence of bytes. Each byte has an address -- a number that identifies it.
On a 64-bit system, addresses are 64-bit numbers (like `0x00007FF4DE3A1B20`).

When you write `let x: i32 = 5`:
- The compiler assigns `x` a stack offset (say, 8 bytes from the frame base)
- At runtime, the frame base might be address `0x7FF4_0000`
- So `x` lives at address `0x7FF4_0008` through `0x7FF4_000B` (4 bytes)

You don't need to think about specific addresses. The important concept is:
**every variable has a location in memory, and that location has a numeric address.**

This matters on Day 4-5 when we cover references and pointers -- a reference
is just a variable that stores someone else's address.

```
  Stack Memory
  ───────────────────────────────────
  Address       Bytes          Name
  ───────────────────────────────────
  0x7FF4_0008   05 00 00 00    x: i32
  0x7FF4_000C   0A 00 00 00    y: i32
  0x7FF4_0010   0F 00 00 00    sum: i32
  ───────────────────────────────────
```

**Rust Connection**: You'll rarely work with raw addresses in Rust (that's `unsafe` territory).
But when you see `&x` (a reference to x), what Rust is really giving you is x's memory
address wrapped in safety guarantees. Understanding that addresses exist under the hood
removes the mystery from references.

**Common Misconceptions**:

- *"I need to manage memory addresses myself."* Not in Rust. The compiler handles this. But
  understanding that addresses exist under the hood removes the mystery from references.

---

## Putting It Together

Let's trace the Day 2 dice roller exercise through memory:

```rust
fn main() {
    let sides: u8 = 6;            // A d6
    let roll: u8 = 4;             // Rolled a 4 (pretend it's random)
    let mut total: u32 = 0;       // Running total
    total = total + roll as u32;  // Add roll to total

    let big_sides: u16 = 100;    // A d100
    let big_roll: u16 = 73;      // Rolled a 73
    total = total + big_roll as u32;
}
```

**Step 1-3: Variables are created on the stack**

```
  ┌──────────────────────────────────────┐
  │  main()'s Stack Frame               │
  ├──────────────────────────────────────┤
  │  sides: u8  = 6       1 byte        │
  ├──────────────────────────────────────┤
  │  roll:  u8  = 4       1 byte        │
  ├──────────────────────────────────────┤
  │  total: u32 = 0       4 bytes (mut) │
  └──────────────────────────────────────┘
```

**Step 4: `total = total + roll as u32;`**

`total` is `mut`, so the compiler allows overwriting its memory location.
The bytes at total's address change from `00 00 00 00` to `04 00 00 00`. The address
didn't change. The variable name still refers to the same location -- only the contents
changed.

**Step 5: After all variables are set**

```
  ┌──────────────────────────────────────────┐
  │  main()'s Stack Frame — ~12 bytes        │
  ├──────────────────────────────────────────┤
  │  sides:     u8  = 6        1 byte        │
  ├──────────────────────────────────────────┤
  │  roll:      u8  = 4        1 byte        │
  ├──────────────────────────────────────────┤
  │  total:     u32 = 77       4 bytes       │
  ├──────────────────────────────────────────┤
  │  big_sides: u16 = 100      2 bytes       │
  ├──────────────────────────────────────────┤
  │  big_roll:  u16 = 73       2 bytes       │
  └──────────────────────────────────────────┘
```

**Step 6: `main()` returns**

The entire stack frame is popped. All bytes are reclaimed instantly. No cleanup code,
no garbage collector. The stack pointer just moves back.

---

## Check Your Understanding

**Q1**: How many bytes does `let health: i32 = 100;` occupy on the stack?

**A1**: 4 bytes. `i32` = 32 bits = 4 bytes, regardless of the value stored.

---

**Q2**: If you write `let x: u8 = 300;`, what happens and why?

**A2**: It won't compile. `u8` can only hold values 0-255 (1 byte, unsigned). The compiler
knows 300 doesn't fit and rejects it. This is the type-as-contract in action.

---

**Q3**: After `let x: i32 = 5;`, does the running program know that the variable is named
"x"?

**A3**: No. The name `x` exists only in source code and during compilation. At runtime,
the binary references a stack offset (like "4 bytes at offset 8 from the frame base").

---

**Q4**: Why is `let mut total = 0;` necessary if you want to write `total = total + 1;`
later? The memory *can* be overwritten -- what stops it?

**A4**: The compiler stops it. Immutability in Rust is a compile-time rule, not a hardware
restriction. Without `mut`, the compiler refuses to generate the write instruction. This
catches accidental mutations at compile time, which prevents a class of bugs.

---

**Q5**: A `bool` only needs one bit of information (true/false). Why does it use a full byte?

**A5**: Memory is byte-addressable -- you can't address a single bit. The smallest unit
the CPU can read or write at an address is one byte (8 bits). So `bool` uses 1 byte, with
7 bits effectively unused.

---

## Vocabulary Added

| Term | Definition | First Seen |
|------|------------|------------|
| **Variable** | A named memory location; the name exists at compile time, the memory at runtime | Day 2 |
| **Type** | A contract specifying how many bytes a value occupies and how to interpret them | Day 2 |
| **Byte** | 8 bits; the smallest addressable unit of memory | Day 2 |
| **Stack frame** | A block of stack memory holding one function's local variables (details Day 3) | Day 2 |
| **Memory address** | A numeric identifier for a specific byte location in memory | Day 2 |
| **Immutability** | A compile-time rule preventing modification of a variable's value after initialization | Day 2 |
| **Little-endian** | Byte ordering where the least significant byte is stored at the lowest address | Day 2 |
| **Scalar type** | A type representing a single value: integer, float, boolean, or character | Day 2 |
