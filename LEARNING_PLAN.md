# Rust Learning Plan - The Rust Programming Language Book

This learning plan is designed for **2-3 hours per day** of focused study. Each session emphasizes understanding **why** things work the way they do, not just memorizing syntax.

## How to Use This Plan
- [x] Read this entire document first to understand the approach
- Check off items as you complete them
- Each chapter has estimated time based on 2-3 hour sessions
- Focus on understanding concepts deeply, not rushing through
- Complete associated rustlings exercises when suggested
- Review the "Why This Matters" sections to build intuition

---

## Week 1: Getting Started & Basics

### Day 1: Introduction & Setup (2-3 hours)
- [x] **Chapter 1: Getting Started**
  - [x] Install Rust (`rustup`)
  - [x] Set up your development environment
  - [x] Create your first "Hello, World!" program
  - [x] Understand Cargo basics (`cargo new`, `cargo build`, `cargo run`)
  - [x] **Why This Matters**: Cargo is Rust's package manager and build system. Understanding it now saves time later.
- [x] **Rustlings**: Complete `intro` exercises

### Day 2: Programming Concepts (2-3 hours)
- [ ] **Chapter 3: Common Programming Concepts (Part 1)**
  - [ ] Variables and mutability (`let`, `mut`)
  - [ ] **Why `mut` is explicit**: Understand Rust's default immutability philosophy
  - [ ] Data types (scalar types: integers, floats, booleans, characters)
  - [ ] **Why explicit types matter**: Type safety prevents entire classes of bugs
- [ ] **Rustlings**: Complete `variables`, `primitive_types` (part 1)
- [ ] **Deep Dive**: Try to mutate an immutable variable and understand the compiler error

### Day 3: Functions & Control Flow (2-3 hours)
- [ ] **Chapter 3: Common Programming Concepts (Part 2)**
  - [ ] Functions and parameters
  - [ ] Statements vs expressions (why `let x = 5` is different from `5`)
  - [ ] **Why expressions matter**: Understanding expression-based programming
  - [ ] Control flow (`if`, `loop`, `while`, `for`)
  - [ ] **Why Rust has multiple loop types**: Each serves a specific purpose
- [ ] **Rustlings**: Complete `functions`, `if`
- [ ] **Practice**: Write a FizzBuzz program to solidify understanding

---

## Week 2: Ownership - The Core Concept

### Day 4: Understanding Ownership (3 hours)
- [ ] **Chapter 4: Understanding Ownership (Part 1)**
  - [ ] What is ownership and why does Rust need it?
  - [ ] **Deep Dive**: Memory management without garbage collection
  - [ ] The three ownership rules
  - [ ] Variable scope and memory allocation
  - [ ] The `String` type vs string literals
- [ ] **Rustlings**: Complete `move_semantics1`, `move_semantics2`
- [ ] **Reflection**: Why does ownership solve memory safety?

### Day 5: References & Borrowing (3 hours)
- [ ] **Chapter 4: Understanding Ownership (Part 2)**
  - [ ] References and borrowing (`&` and `&mut`)
  - [ ] **Why borrowing rules exist**: Preventing data races at compile time
  - [ ] Mutable vs immutable references
  - [ ] The borrowing rules (one mutable OR many immutable)
  - [ ] **Deep Dive**: Try to break the borrowing rules and understand compiler errors
- [ ] **Rustlings**: Complete `move_semantics3`, `move_semantics4`, `move_semantics5`
- [ ] **Practice**: Modify function signatures to use references instead of ownership

### Day 6: Slices & Ownership Mastery (2-3 hours)
- [ ] **Chapter 4: Understanding Ownership (Part 3)**
  - [ ] The slice type
  - [ ] String slices (`&str`)
  - [ ] **Why slices are references**: Understanding slice internals
  - [ ] Other slices (arrays, vectors)
- [ ] **Rustlings**: Complete `move_semantics6`, review all move_semantics
- [ ] **Deep Dive**: Draw memory diagrams of ownership transfers
- [ ] **Practice**: Write a function that returns the first word of a string using slices

---

## Week 3: Structs & Enums

### Day 7: Defining and Using Structs (2-3 hours)
- [ ] **Chapter 5: Using Structs (Part 1)**
  - [ ] Defining structs
  - [ ] Struct instantiation and field access
  - [ ] Tuple structs and unit-like structs
  - [ ] **Why different struct types**: When to use each
  - [ ] Ownership of struct data
- [ ] **Rustlings**: Complete `structs1`, `structs2`
- [ ] **Practice**: Create a User struct with owned and borrowed data

### Day 8: Method Syntax (2-3 hours)
- [ ] **Chapter 5: Using Structs (Part 2)**
  - [ ] Method syntax with `impl` blocks
  - [ ] `self`, `&self`, and `&mut self`
  - [ ] **Why different self types**: Understanding method receivers
  - [ ] Associated functions (like `String::from`)
  - [ ] Multiple `impl` blocks
- [ ] **Rustlings**: Complete `structs3`
- [ ] **Practice**: Implement methods for a Rectangle struct (area, can_hold)

### Day 9: Enums and Pattern Matching (3 hours)
- [ ] **Chapter 6: Enums and Pattern Matching (Part 1)**
  - [ ] Defining enums
  - [ ] Enum variants with data
  - [ ] **Why enums are powerful**: Tagged unions and type safety
  - [ ] The `Option<T>` enum
  - [ ] **Why Option instead of null**: Eliminating null pointer errors
- [ ] **Rustlings**: Complete `enums1`, `enums2`, `enums3`
- [ ] **Deep Dive**: Why doesn't Rust have null?

### Day 10: Match & Control Flow (2-3 hours)
- [ ] **Chapter 6: Enums and Pattern Matching (Part 2)**
  - [ ] The `match` control flow operator
  - [ ] Patterns that bind to values
  - [ ] **Why match is exhaustive**: Compiler-enforced completeness
  - [ ] The `_` placeholder
  - [ ] `if let` syntax
  - [ ] **When to use `if let` vs `match`**
- [ ] **Rustlings**: Complete `options`, `match`, `if_let`
- [ ] **Practice**: Write a program that processes an enum with match

---

## Week 4: Modules & Collections

### Day 11: Packages, Crates, and Modules (3 hours)
- [ ] **Chapter 7: Managing Growing Projects**
  - [ ] Packages and crates
  - [ ] Defining modules
  - [ ] Paths for referring to items
  - [ ] `pub` keyword and privacy
  - [ ] `use` keyword
  - [ ] **Why module system matters**: Organizing large projects
  - [ ] Separating modules into different files
- [ ] **Rustlings**: Complete `modules1`, `modules2`, `modules3`
- [ ] **Practice**: Refactor a program into multiple modules

### Day 12: Common Collections - Vectors (2-3 hours)
- [ ] **Chapter 8: Common Collections (Part 1)**
  - [ ] Vectors (`Vec<T>`)
  - [ ] Creating, updating, and reading vectors
  - [ ] **Why vector indexing can panic**: Understanding ownership with collections
  - [ ] Iterating over vectors
  - [ ] Using enums to store multiple types
- [ ] **Rustlings**: Complete `vec1`, `vec2`
- [ ] **Practice**: Build a dynamic list management program

### Day 13: Strings (2-3 hours)
- [ ] **Chapter 8: Common Collections (Part 2)**
  - [ ] What is a String?
  - [ ] Creating and updating strings
  - [ ] **Why strings are complicated**: UTF-8 encoding
  - [ ] Indexing into strings (and why you can't)
  - [ ] **Deep Dive**: Bytes, scalar values, and grapheme clusters
  - [ ] Slicing strings safely
  - [ ] Iterating over strings
- [ ] **Rustlings**: Complete `strings1`, `strings2`, `strings3`, `strings4`
- [ ] **Reflection**: Why are String and &str different?

### Day 14: Hash Maps (2 hours)
- [ ] **Chapter 8: Common Collections (Part 3)**
  - [ ] Creating and using hash maps
  - [ ] Ownership and hash maps
  - [ ] Updating hash maps (overwrite, insert if empty, update based on old value)
  - [ ] **Why hash maps move ownership**: Understanding collection ownership
- [ ] **Rustlings**: Complete `hashmaps1`, `hashmaps2`, `hashmaps3`
- [ ] **Practice**: Build a word frequency counter

---

## Week 5: Error Handling & Generics

### Day 15: Unrecoverable Errors (2 hours)
- [ ] **Chapter 9: Error Handling (Part 1)**
  - [ ] `panic!` macro
  - [ ] When to panic
  - [ ] Unwinding vs aborting
  - [ ] **Why Rust makes errors explicit**: No hidden exceptions
- [ ] **Rustlings**: Complete `errors1`
- [ ] **Deep Dive**: What happens during a panic?

### Day 16: Recoverable Errors with Result (3 hours)
- [ ] **Chapter 9: Error Handling (Part 2)**
  - [ ] The `Result<T, E>` enum
  - [ ] Matching on Result
  - [ ] Shortcuts: `unwrap` and `expect`
  - [ ] **Why Result forces error handling**: Type-safe error propagation
  - [ ] Propagating errors with `?` operator
  - [ ] **Deep Dive**: How `?` works under the hood
- [ ] **Rustlings**: Complete `errors2`, `errors3`, `errors4`, `errors5`, `errors6`
- [ ] **Practice**: Write a function that reads a file and propagates errors

### Day 17: Generic Types (3 hours)
- [ ] **Chapter 10: Generic Types, Traits, and Lifetimes (Part 1)**
  - [ ] Generic data types in functions
  - [ ] Generic structs
  - [ ] Generic enums (revisit Option and Result)
  - [ ] Generic method definitions
  - [ ] **Why generics have no runtime cost**: Zero-cost abstractions
  - [ ] Monomorphization
- [ ] **Rustlings**: Complete `generics1`, `generics2`
- [ ] **Practice**: Create a generic Point<T> struct

### Day 18: Traits (3 hours)
- [ ] **Chapter 10: Generic Types, Traits, and Lifetimes (Part 2)**
  - [ ] Defining traits
  - [ ] Implementing traits on types
  - [ ] Default implementations
  - [ ] Traits as parameters (`impl Trait`)
  - [ ] Trait bounds
  - [ ] **Why traits are like interfaces**: But more powerful
  - [ ] `where` clauses
  - [ ] Returning types that implement traits
- [ ] **Rustlings**: Complete `traits1`, `traits2`, `traits3`, `traits4`, `traits5`
- [ ] **Deep Dive**: How trait objects enable polymorphism

### Day 19: Lifetimes (3 hours)
- [ ] **Chapter 10: Generic Types, Traits, and Lifetimes (Part 3)**
  - [ ] What are lifetimes?
  - [ ] **Why lifetimes exist**: Preventing dangling references
  - [ ] Lifetime annotation syntax
  - [ ] Lifetime annotations in function signatures
  - [ ] **Deep Dive**: How the borrow checker uses lifetimes
  - [ ] Lifetime elision rules
  - [ ] Lifetime annotations in structs
  - [ ] The static lifetime
- [ ] **Rustlings**: Complete `lifetimes1`, `lifetimes2`, `lifetimes3`
- [ ] **Practice**: Write functions with explicit lifetimes and understand the errors

---

## Week 6: Testing & I/O

### Day 20: Writing Tests (3 hours)
- [ ] **Chapter 11: Writing Automated Tests**
  - [ ] How to write tests
  - [ ] The `#[test]` attribute
  - [ ] Assertions: `assert!`, `assert_eq!`, `assert_ne!`
  - [ ] `should_panic` tests
  - [ ] Using `Result<T, E>` in tests
  - [ ] **Why testing is built into Rust**: Test-driven development
  - [ ] Running tests with `cargo test`
  - [ ] Test organization (unit tests, integration tests)
- [ ] **Rustlings**: Complete `tests1`, `tests2`, `tests3`, `tests4`
- [ ] **Practice**: Write tests for your previous exercises

### Day 21: I/O Project - Part 1 (3 hours)
- [ ] **Chapter 12: An I/O Project (Part 1)**
  - [ ] Accepting command line arguments
  - [ ] Reading a file
  - [ ] Refactoring to improve modularity
  - [ ] **Why separation of concerns matters**: Building maintainable code
  - [ ] Extracting logic from `main`
- [ ] **Practice**: Start building a minigrep clone
- [ ] **Rustlings**: Review `strings` and `vectors` exercises

### Day 22: I/O Project - Part 2 (3 hours)
- [ ] **Chapter 12: An I/O Project (Part 2)**
  - [ ] Error handling in the project
  - [ ] Extracting logic from `main`
  - [ ] Using `?` for error propagation
  - [ ] Splitting code into library and binary
  - [ ] **Why library crates are testable**: Separation of concerns
  - [ ] Writing tests for your library
- [ ] **Practice**: Complete the minigrep project
- [ ] **Deep Dive**: How does `cargo test` find and run tests?

---

## Week 7-8: Functional Programming & Advanced Concepts

### Day 23: Closures (3 hours)
- [ ] **Chapter 13: Functional Language Features (Part 1)**
  - [ ] Closure syntax
  - [ ] Capturing environment
  - [ ] **Why closures are powerful**: Inline anonymous functions
  - [ ] Closure type inference
  - [ ] `Fn`, `FnMut`, and `FnOnce` traits
  - [ ] **Deep Dive**: How closures capture variables
- [ ] **Rustlings**: Complete `closures1`, `closures2`, `closures3`
- [ ] **Practice**: Use closures with iterator methods

### Day 24: Iterators (3 hours)
- [ ] **Chapter 13: Functional Language Features (Part 2)**
  - [ ] The `Iterator` trait
  - [ ] Methods that consume iterators
  - [ ] Methods that produce iterators
  - [ ] **Why iterators are zero-cost**: Performance guarantees
  - [ ] Comparing loops vs iterators
  - [ ] Creating your own iterators
- [ ] **Rustlings**: Complete `iterators1`, `iterators2`, `iterators3`, `iterators4`, `iterators5`
- [ ] **Practice**: Refactor previous code to use iterators

### Day 25: Improving minigrep (2-3 hours)
- [ ] **Chapter 13: Improving Our I/O Project**
  - [ ] Refactoring with iterators
  - [ ] Making code more concise
  - [ ] Performance comparison
  - [ ] **Why functional style is idiomatic**: Rust best practices
- [ ] **Practice**: Refactor your minigrep to use iterators
- [ ] **Reflection**: Which style do you find more readable?

### Day 26: Cargo and Crates.io (2-3 hours)
- [ ] **Chapter 14: More About Cargo**
  - [ ] Release profiles
  - [ ] Publishing to crates.io
  - [ ] Cargo workspaces
  - [ ] **Why workspaces matter**: Managing multiple packages
  - [ ] Installing binaries with `cargo install`
  - [ ] Extending Cargo with custom commands
- [ ] **Practice**: Set up a workspace with multiple crates
- [ ] **Optional**: Create a simple crate structure for publishing

### Day 27: Smart Pointers - Box (2-3 hours)
- [ ] **Chapter 15: Smart Pointers (Part 1)**
  - [ ] What are smart pointers?
  - [ ] `Box<T>` for heap allocation
  - [ ] **Why Box is needed**: Recursive types and dynamic sizing
  - [ ] Enabling recursive types with Box
  - [ ] **Deep Dive**: Stack vs heap memory
- [ ] **Rustlings**: Complete `box1`
- [ ] **Practice**: Implement a cons list with Box

### Day 28: Deref and Drop Traits (2-3 hours)
- [ ] **Chapter 15: Smart Pointers (Part 2)**
  - [ ] The `Deref` trait
  - [ ] Deref coercion
  - [ ] **Why deref coercion is useful**: Automatic conversions
  - [ ] The `Drop` trait
  - [ ] **Why Drop ensures cleanup**: RAII pattern
  - [ ] Dropping values early with `drop`
- [ ] **Rustlings**: Complete `cow1` (Copy-on-Write)
- [ ] **Deep Dive**: How automatic dereferencing works

### Day 29: Rc and RefCell (3 hours)
- [ ] **Chapter 15: Smart Pointers (Part 3)**
  - [ ] `Rc<T>` for reference counting
  - [ ] **Why Rc enables shared ownership**: Multiple owners
  - [ ] `RefCell<T>` for interior mutability
  - [ ] **Why RefCell is needed**: Runtime borrow checking
  - [ ] Combining `Rc<T>` and `RefCell<T>`
  - [ ] **Deep Dive**: When to use each smart pointer
- [ ] **Rustlings**: Complete `rc1`
- [ ] **Practice**: Build a tree structure with shared ownership

### Day 30: Reference Cycles (2 hours)
- [ ] **Chapter 15: Smart Pointers (Part 4)**
  - [ ] Creating reference cycles
  - [ ] Preventing reference cycles with `Weak<T>`
  - [ ] **Why reference cycles are dangerous**: Memory leaks
  - [ ] **Deep Dive**: Weak vs strong references
- [ ] **Practice**: Fix a reference cycle using Weak
- [ ] **Reflection**: When would you use Rc vs Arc?

---

## Week 9: Concurrency

### Day 31: Threads (3 hours)
- [ ] **Chapter 16: Fearless Concurrency (Part 1)**
  - [ ] Creating threads with `spawn`
  - [ ] Waiting for threads with `join`
  - [ ] Using `move` closures with threads
  - [ ] **Why Rust's concurrency is safe**: No data races at compile time
  - [ ] **Deep Dive**: Send and Sync traits
- [ ] **Rustlings**: Complete `threads1`, `threads2`, `threads3`
- [ ] **Practice**: Spawn multiple threads to perform calculations

### Day 32: Message Passing (2-3 hours)
- [ ] **Chapter 16: Fearless Concurrency (Part 2)**
  - [ ] Channels for message passing
  - [ ] `mpsc` (multiple producer, single consumer)
  - [ ] Sending and receiving messages
  - [ ] **Why message passing prevents race conditions**: Ownership transfer
  - [ ] Multiple producers
  - [ ] **Deep Dive**: How channels use ownership
- [ ] **Practice**: Build a message passing system between threads

### Day 33: Shared State Concurrency (3 hours)
- [ ] **Chapter 16: Fearless Concurrency (Part 3)**
  - [ ] Using `Mutex<T>`
  - [ ] **Why mutexes are needed**: Shared mutable state
  - [ ] Multiple ownership with `Arc<T>`
  - [ ] Combining `Arc<Mutex<T>>`
  - [ ] **Deep Dive**: Deadlocks and how to avoid them
  - [ ] `Send` and `Sync` traits in depth
- [ ] **Practice**: Build a counter shared between multiple threads
- [ ] **Reflection**: Message passing vs shared state - when to use each?

---

## Week 10: Object-Oriented & Patterns

### Day 34: OOP in Rust (2-3 hours)
- [ ] **Chapter 17: Object-Oriented Features**
  - [ ] Characteristics of OOP
  - [ ] Encapsulation with `pub`
  - [ ] Inheritance vs composition
  - [ ] **Why Rust doesn't have inheritance**: Composition over inheritance
  - [ ] Polymorphism with traits
  - [ ] **Deep Dive**: Trait objects vs generics
- [ ] **Practice**: Refactor an OOP design to Rust patterns

### Day 35: Trait Objects (3 hours)
- [ ] **Chapter 17: Object-Oriented Features (Part 2)**
  - [ ] Trait objects with `dyn`
  - [ ] Object safety
  - [ ] **Why some traits can't be objects**: Compiler requirements
  - [ ] Dynamic dispatch vs static dispatch
  - [ ] **Performance implications**: When to use each approach
- [ ] **Practice**: Implement a GUI library example with trait objects
- [ ] **Deep Dive**: What is the cost of dynamic dispatch?

### Day 36: Patterns (3 hours)
- [ ] **Chapter 18: Patterns and Matching (Part 1)**
  - [ ] Where patterns can be used
  - [ ] Refutability: irrefutable vs refutable patterns
  - [ ] **Why pattern types matter**: Compiler guarantees
  - [ ] Pattern syntax: literals, variables, wildcards
  - [ ] Multiple patterns with `|`
  - [ ] Matching ranges
  - [ ] Destructuring structs, enums, tuples
- [ ] **Rustlings**: Review `enums` and `match` exercises
- [ ] **Practice**: Use pattern matching extensively

### Day 37: Advanced Patterns (2-3 hours)
- [ ] **Chapter 18: Patterns and Matching (Part 2)**
  - [ ] Ignoring values with `_` and `..`
  - [ ] Match guards
  - [ ] `@` bindings
  - [ ] **Why advanced patterns are powerful**: Expressive matching
- [ ] **Practice**: Refactor previous code with advanced patterns
- [ ] **Reflection**: How do patterns improve code readability?

---

## Week 11-12: Advanced Features

### Day 38: Unsafe Rust (3 hours)
- [ ] **Chapter 19: Advanced Features (Part 1)**
  - [ ] When and why to use `unsafe`
  - [ ] Dereferencing raw pointers
  - [ ] Calling unsafe functions
  - [ ] Creating safe abstractions over unsafe code
  - [ ] **Why unsafe is necessary**: FFI and low-level operations
  - [ ] Using `extern` functions
- [ ] **Deep Dive**: What guarantees does unsafe give up?
- [ ] **Practice**: Write a safe wrapper around an unsafe operation

### Day 39: Advanced Traits (3 hours)
- [ ] **Chapter 19: Advanced Features (Part 2)**
  - [ ] Associated types
  - [ ] **Why associated types vs generics**: When to use each
  - [ ] Default generic type parameters
  - [ ] Operator overloading
  - [ ] Fully qualified syntax
  - [ ] Supertraits
  - [ ] Newtype pattern
- [ ] **Practice**: Implement operator overloading for a custom type
- [ ] **Deep Dive**: How does operator overloading work in Rust?

### Day 40: Advanced Types (2-3 hours)
- [ ] **Chapter 19: Advanced Features (Part 3)**
  - [ ] Type aliases with `type`
  - [ ] The never type `!`
  - [ ] **Why the never type exists**: Diverging functions
  - [ ] Dynamically sized types (DST)
  - [ ] `Sized` trait
  - [ ] **Deep Dive**: How does Rust handle unsized types?
- [ ] **Practice**: Use type aliases to simplify complex types

### Day 41: Advanced Functions and Closures (2-3 hours)
- [ ] **Chapter 19: Advanced Features (Part 4)**
  - [ ] Function pointers
  - [ ] Returning closures
  - [ ] **Why closures and functions are different**: Trait objects
  - [ ] **Deep Dive**: Closure implementation details
- [ ] **Practice**: Pass functions and closures as parameters

### Day 42: Macros (3 hours)
- [ ] **Chapter 19: Advanced Features (Part 5)**
  - [ ] Declarative macros with `macro_rules!`
  - [ ] **Why macros are different from functions**: Metaprogramming
  - [ ] Macro patterns and syntax
  - [ ] Procedural macros (overview)
  - [ ] **Deep Dive**: How macros expand at compile time
- [ ] **Rustlings**: Complete `macros` exercises
- [ ] **Practice**: Write a simple declarative macro

---

## Week 13: Final Project & Advanced Topics

### Day 43: Final Project - Web Server Setup (3 hours)
- [ ] **Chapter 20: Final Project - Web Server (Part 1a)**
  - [ ] Building a single-threaded web server
  - [ ] Listening to TCP connections
  - [ ] Reading HTTP requests
- [ ] **Practice**: Start the basic web server

### Day 44: Complete Single-Threaded Web Server (3 hours)
- [ ] **Chapter 20: Final Project - Web Server (Part 1b)**
  - [ ] Writing HTTP responses
  - [ ] Handling multiple requests
  - [ ] **Why building from scratch teaches fundamentals**
- [ ] **Practice**: Complete the basic web server

### Day 45: Design Thread Pool (3 hours)
- [ ] **Chapter 20: Final Project - Web Server (Part 2a)**
  - [ ] Planning the multithreaded design
  - [ ] Designing a thread pool
  - [ ] **Deep Dive**: Worker pattern and job queues
- [ ] **Practice**: Design the thread pool architecture

### Day 46: Implement Thread Pool (3 hours)
- [ ] **Chapter 20: Final Project - Web Server (Part 2b)**
  - [ ] Implementing the thread pool
  - [ ] Turning single-threaded into multithreaded
- [ ] **Practice**: Build the thread pool

### Day 47: Complete Multithreaded Web Server (3 hours)
- [ ] **Chapter 20: Final Project - Web Server (Part 2c)**
  - [ ] Graceful shutdown
  - [ ] Testing the multithreaded server
  - [ ] Performance comparison
- [ ] **Practice**: Complete and test the multithreaded web server
- [ ] **Reflection**: What concurrency patterns did you use?

### Day 48: Review and Next Steps (2-3 hours)
- [ ] Review all rustlings exercises
- [ ] Identify weak areas for revision
- [ ] Plan personal projects to reinforce learning
- [ ] **Reflection**: What patterns do you understand best? Which need more work?
- [ ] Update REVISION_SCHEDULE.md with personalized focus areas

---

## Supplementary Resources

### Rustlings Progress Tracking
- [ ] Complete all `intro` exercises
- [ ] Complete all `variables` exercises
- [ ] Complete all `functions` exercises
- [ ] Complete all `if` exercises
- [ ] Complete all `primitive_types` exercises
- [ ] Complete all `move_semantics` exercises (1-6)
- [ ] Complete all `structs` exercises (1-3)
- [ ] Complete all `enums` exercises (1-3)
- [ ] Complete all `modules` exercises (1-3)
- [ ] Complete all `collections` exercises (vectors, hashmaps)
- [ ] Complete all `strings` exercises (1-4)
- [ ] Complete all `errors` exercises (1-6)
- [ ] Complete all `generics` exercises (1-2)
- [ ] Complete all `traits` exercises (1-5)
- [ ] Complete all `lifetimes` exercises (1-3)
- [ ] Complete all `tests` exercises (1-4)
- [ ] Complete all `iterators` exercises (1-5)
- [ ] Complete all `threads` exercises (1-3)
- [ ] Complete all `macros` exercises
- [ ] Complete all remaining exercises

### Additional Practice Ideas
- [ ] Build a command-line todo app
- [ ] Create a simple HTTP client
- [ ] Implement common data structures (linked list, binary tree)
- [ ] Write a basic JSON parser
- [ ] Build a chat server with async/await (after learning async)
- [ ] Contribute to an open-source Rust project

### Deep Understanding Checklist
- [ ] I understand why Rust needs ownership
- [ ] I can explain borrowing rules to someone else
- [ ] I know when to use `String` vs `&str`
- [ ] I understand the difference between the stack and heap
- [ ] I can explain why Rust doesn't have null
- [ ] I understand trait objects vs generics
- [ ] I know when to use `Box`, `Rc`, `Arc`, `RefCell`
- [ ] I understand the difference between `Send` and `Sync`
- [ ] I can explain why Rust's concurrency is safe
- [ ] I understand lifetime annotations and when they're needed

---

## Tips for Success

1. **Focus on "Why" Not "What"**
   - Don't just memorize syntax
   - Understand the reasoning behind design decisions
   - Ask yourself: "Why does Rust force me to do this?"

2. **Struggle is Learning**
   - Compiler errors are teaching moments
   - Read error messages carefully - they're very helpful
   - Try to fix errors before looking up solutions

3. **Consistent Daily Practice**
   - 2-3 hours daily is better than 20 hours once a week
   - Take breaks when stuck
   - Sleep on difficult concepts

4. **Write Code, Don't Just Read**
   - Type every example yourself
   - Modify examples to see what breaks
   - Experiment with variations

5. **Use the Compiler as a Teacher**
   - Intentionally write incorrect code to see errors
   - Understand what the compiler is protecting you from
   - Learn to read and interpret error messages

6. **Track Your Progress**
   - Check off items as you complete them
   - Review checked items periodically
   - Update this document with your own notes

7. **Join the Community**
   - Read the Rust forum and r/rust
   - Ask questions on Discord/Zulip
   - Learn from others' mistakes and questions

---

**Remember**: Understanding Rust deeply takes time. The ownership system is different from most languages, and that's okay. Take your time to build strong mental models. The effort you invest in understanding fundamentals will pay off exponentially as you build more complex programs.
