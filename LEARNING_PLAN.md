# Rust Through Game Development - Complete Learning Plan

A 20-week, 70-day curriculum. Each session is 2-3 hours. Every Rust concept is introduced because a game needs it.

## How to Use This Plan

- [x] Read this entire document first to understand the arc
- Check off items as you complete them
- Each day has a **Rust Concept**, a **Game Application**, and **Exercises**
- "Why This Matters for Games" sections connect language features to real game dev problems
- References point you to deeper material for every topic
- Complete rustlings exercises when suggested -- they drill syntax so you can focus on design

---

# Phase 1: Rust Fundamentals Through Terminal Games (Weeks 1-5)

*Goal: Master core Rust by building text-based games. No graphics yet -- focus on the language.*

---

## Week 1: Setup, Variables, and Control Flow

### Day 1: Hello Rust & Cargo (2-3 hours)

- [x] **Chapter 1: Getting Started**
  - [x] Install Rust via [rustup](https://rustup.rs/)
  - [x] Set up your editor (VS Code + rust-analyzer recommended)
  - [x] `cargo new`, `cargo build`, `cargo run`, `cargo check`
  - [x] Create "Hello, World!" program
- [x] **Rustlings**: Complete `intro` exercises
- [x] **Game Exercise**: Build a number guessing game (Chapter 2 of the Rust Book)
  - Random number generation with `rand` crate
  - User input with `std::io`
  - Basic `loop`, `match`, and `parse`
- **Why This Matters for Games**: Cargo manages all your game dependencies (rendering libraries, physics engines, audio). Understanding it now means you can focus on game code later.

**References**:
- [The Rust Book - Ch 1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [The Rust Book - Ch 2: Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Day 2: Variables, Types, and Mutability (2-3 hours)

- [ ] **Chapter 3 (Part 1): Common Programming Concepts**
  - [ ] Variables and `mut` -- Rust defaults to immutable
  - [ ] Shadowing vs mutation
  - [ ] Scalar types: integers (`i32`, `u32`, `f64`), booleans, characters
  - [ ] Compound types: tuples and arrays
- [ ] **Rustlings**: Complete `variables`, `primitive_types` (part 1)
- [ ] **Game Exercise**: Build a **dice roller** CLI tool
  - Define dice with different types (`u8` for d6, `u16` for d100)
  - Roll multiple dice, sum results
  - Use shadowing to convert string input to numbers
- [ ] **Deep Dive**: Intentionally mutate an immutable variable -- read and understand the compiler error
- **Why This Matters for Games**: Game state (health, position, score) must be carefully managed. Immutability by default prevents accidental state corruption -- a huge class of game bugs.

**References**:
- [The Rust Book - Ch 3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Rust Book - Ch 3.2: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)

### Day 3: Functions and Control Flow (2-3 hours)

- [ ] **Chapter 3 (Part 2): Functions & Control Flow**
  - [ ] Functions, parameters, return values
  - [ ] Statements vs expressions (why `{ x + 1 }` returns a value)
  - [ ] `if`/`else`, `loop`, `while`, `for`
  - [ ] `loop` with `break` returning values
- [ ] **Rustlings**: Complete `functions`, `if`
- [ ] **Game Exercise**: Build a **text-based combat round**
  - `fn roll_attack(strength: u32) -> u32` -- calculate damage
  - `fn apply_damage(health: &mut i32, damage: u32)` -- modify health
  - Loop combat rounds until one side reaches 0 HP
  - Use `if`/`else` for hit/miss logic
- **Why This Matters for Games**: Game loops are literally loops. Combat, physics, AI -- all are functions called every frame. Understanding expressions means writing concise game logic.

**References**:
- [The Rust Book - Ch 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Book - Ch 3.5: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example - Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)

---

## Week 2: Ownership -- The Core Concept

*This is the most important week. Ownership is what makes Rust unique and what makes Rust games safe and fast.*

### Day 4: Ownership Fundamentals (3 hours)

- [ ] **Chapter 4 (Part 1): Understanding Ownership**
  - [ ] The three ownership rules
  - [ ] Stack vs heap memory
  - [ ] `String` vs string literals (`&str`)
  - [ ] Move semantics -- what happens when you assign a `String` to another variable
  - [ ] `Clone` for explicit deep copies
- [ ] **Rustlings**: Complete `move_semantics1`, `move_semantics2`
- [ ] **Game Exercise**: Build a **loot system**
  - Create a `String`-based item name
  - Transfer an item from a treasure chest to a player (ownership move)
  - Try to use the chest's item after transfer -- understand the compiler error
  - Use `clone()` when you need to duplicate an item
- [ ] **Reflection**: Draw a memory diagram of what happens when a `String` is moved
- **Why This Matters for Games**: Game objects (entities, textures, meshes) own their data. When a player picks up an item, ownership transfers. When an entity is destroyed, its memory is freed. Ownership models this perfectly without a garbage collector stalling your frame rate.

**References**:
- [The Rust Book - Ch 4.1: What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Visualizing Memory Layout of Rust's Data Types](https://cheats.rs/#memory-layout) (cheats.rs)
- [Rust by Example - Ownership and Moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)

### Day 5: References and Borrowing (3 hours)

- [ ] **Chapter 4 (Part 2): References & Borrowing**
  - [ ] Immutable references (`&T`) -- borrow without taking ownership
  - [ ] Mutable references (`&mut T`) -- borrow and modify
  - [ ] The borrowing rules: one `&mut` OR many `&`, never both
  - [ ] Why these rules prevent data races
- [ ] **Rustlings**: Complete `move_semantics3`, `move_semantics4`, `move_semantics5`
- [ ] **Game Exercise**: Build a **party system**
  - A `display_stats(player: &Player)` function that reads but doesn't own
  - A `heal(player: &mut Player, amount: u32)` function that modifies
  - Try to read and write simultaneously -- understand why the compiler stops you
- [ ] **Deep Dive**: Intentionally break borrowing rules in 3 different ways. Read every compiler error carefully.
- **Why This Matters for Games**: In a game, the renderer needs to *read* entity positions (immutable borrow) while the physics system *writes* them (mutable borrow). Rust's rules guarantee these don't conflict, eliminating an entire category of bugs that plague C++ game engines.

**References**:
- [The Rust Book - Ch 4.2: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust by Example - Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

### Day 6: Slices and Ownership Mastery (2-3 hours)

- [ ] **Chapter 4 (Part 3): The Slice Type**
  - [ ] String slices (`&str`) -- a view into a `String`
  - [ ] Array slices (`&[T]`)
  - [ ] Why slices are fat pointers (pointer + length)
- [ ] **Rustlings**: Complete `move_semantics6`, review all `move_semantics`
- [ ] **Game Exercise**: Build a **command parser** for a text adventure
  - Parse input like `"take sword"` into command + target using slices
  - `fn parse_command(input: &str) -> (&str, &str)` -- returns slices into the input
  - Handle edge cases: empty input, single word, multiple words
- [ ] **Practice**: Rewrite your Day 4 loot system using references instead of cloning where possible
- **Why This Matters for Games**: Slices let you reference parts of data without copying. Parsing player commands, reading subsets of a tilemap, accessing animation frame ranges -- all use slices.

**References**:
- [The Rust Book - Ch 4.3: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust by Example - Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)

---

## Week 3: Structs, Enums, and Pattern Matching

*Game entities are structs. Game states are enums. Pattern matching is how you handle them.*

### Day 7: Defining Structs (2-3 hours)

- [ ] **Chapter 5 (Part 1): Using Structs**
  - [ ] Defining and instantiating structs
  - [ ] Field init shorthand
  - [ ] Struct update syntax (`..other_player`)
  - [ ] Tuple structs and unit structs
  - [ ] Ownership of struct data
- [ ] **Rustlings**: Complete `structs1`, `structs2`
- [ ] **Game Exercise**: Define your **game entity types**
  - `struct Position { x: f64, y: f64 }`
  - `struct Player { name: String, health: i32, position: Position, inventory: Vec<String> }`
  - `struct Enemy { kind: String, health: i32, position: Position, damage: u32 }`
  - Create instances, print their fields
- **Why This Matters for Games**: Every game object -- player, enemy, projectile, tile -- is a struct. Struct composition (Position inside Player) is how game engines organize data.

**References**:
- [The Rust Book - Ch 5.1: Defining Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust by Example - Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

### Day 8: Methods and Associated Functions (2-3 hours)

- [ ] **Chapter 5 (Part 2): Method Syntax**
  - [ ] `impl` blocks
  - [ ] `&self`, `&mut self`, `self` -- choosing the right receiver
  - [ ] Associated functions (constructors like `Player::new()`)
  - [ ] Multiple `impl` blocks
- [ ] **Rustlings**: Complete `structs3`
- [ ] **Game Exercise**: Add behavior to your entities
  - `impl Player { fn new(name: &str) -> Self }`
  - `impl Player { fn take_damage(&mut self, amount: u32) }`
  - `impl Player { fn is_alive(&self) -> bool }`
  - `impl Player { fn distance_to(&self, other: &Position) -> f64 }`
  - `impl Enemy { fn attack(&self, target: &mut Player) }`
- **Why This Matters for Games**: Methods encapsulate game logic. `player.take_damage(10)` is readable, correct, and enforces access rules through `&self` vs `&mut self`.

**References**:
- [The Rust Book - Ch 5.3: Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust by Example - Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)

### Day 9: Enums and Option (3 hours)

- [ ] **Chapter 6 (Part 1): Enums**
  - [ ] Defining enums with variants
  - [ ] Enum variants with data (tuple and struct variants)
  - [ ] `Option<T>` -- Rust's replacement for null
  - [ ] Why Rust has no null: every "maybe" value is explicit
- [ ] **Rustlings**: Complete `enums1`, `enums2`, `enums3`
- [ ] **Game Exercise**: Model **game states and items**
  - `enum GameState { MainMenu, Playing, Paused, GameOver }`
  - `enum Item { Weapon { name: String, damage: u32 }, Potion { heal_amount: u32 }, Key(String) }`
  - `enum Direction { North, South, East, West }`
  - Use `Option<Item>` for a treasure chest that might be empty
- **Why This Matters for Games**: Game states (menu, playing, paused) are enums. Item types, enemy AI states, tile types -- all enums. `Option` handles "maybe" cases like "does this tile have an item?" without null pointer crashes.

**References**:
- [The Rust Book - Ch 6.1: Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The Rust Book - Ch 6.1: The Option Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)

### Day 10: Pattern Matching (2-3 hours)

- [ ] **Chapter 6 (Part 2): match and if let**
  - [ ] `match` -- exhaustive pattern matching
  - [ ] Patterns that bind to values
  - [ ] The `_` wildcard
  - [ ] `if let` for single-pattern checks
- [ ] **Rustlings**: Complete `options`, `match_expressions`, `if_let`
- [ ] **Game Exercise**: Build a **turn-based combat system**
  - `enum Action { Attack, Defend, UseItem(Item), Flee }`
  - `match action { Attack => ..., Defend => ..., UseItem(item) => ..., Flee => ... }`
  - Handle game state transitions with match: `match game_state { Playing => ..., ... }`
  - Use `if let Some(item) = chest.contents` to open chests
- [ ] **Weekend Project**: Combine Days 7-10 into a **mini text RPG**
  - Player with stats, inventory
  - Enemies with different types (enum)
  - Turn-based combat with match
  - Room navigation with Direction enum
- **Why This Matters for Games**: `match` is how you dispatch on game events, AI states, input actions, and collision results. Exhaustive matching means you can't forget to handle a case.

**References**:
- [The Rust Book - Ch 6.2: match](https://doc.rust-lang.org/book/ch06-02-match.html)
- [The Rust Book - Ch 6.3: if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [Rust by Example - match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

---

## Week 4: Modules and Collections

*Organize your game code. Build data structures games actually need.*

### Day 11: Modules and Project Structure (3 hours)

- [ ] **Chapter 7: Managing Growing Projects**
  - [ ] Packages and crates
  - [ ] Defining modules with `mod`
  - [ ] Visibility with `pub`
  - [ ] `use` keyword for bringing paths into scope
  - [ ] Separating modules into files
- [ ] **Rustlings**: Complete `modules1`, `modules2`, `modules3`
- [ ] **Game Exercise**: Restructure your text RPG into modules
  - `src/main.rs` -- entry point
  - `src/player.rs` -- Player struct and methods
  - `src/enemy.rs` -- Enemy types and AI
  - `src/combat.rs` -- combat logic
  - `src/world.rs` -- rooms, navigation
  - Use `pub` to expose only what other modules need
- **Why This Matters for Games**: Real games have thousands of lines of code. Modules keep player code, rendering code, physics code, and audio code separated and maintainable.

**References**:
- [The Rust Book - Ch 7: Packages, Crates, Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)

### Day 12: Vectors (2-3 hours)

- [ ] **Chapter 8 (Part 1): Vectors**
  - [ ] `Vec<T>` -- growable arrays
  - [ ] Creating, pushing, reading
  - [ ] Indexing vs `.get()` (panic vs Option)
  - [ ] Iterating over vectors
  - [ ] Vectors of enums for mixed types
- [ ] **Rustlings**: Complete `vecs1`, `vecs2`
- [ ] **Game Exercise**: Build an **inventory system**
  - `Vec<Item>` for player inventory
  - Add, remove, find items
  - `Vec<Enemy>` for enemies in a room
  - Filter enemies by type, find nearest enemy
  - Use `Vec<Box<dyn Entity>>` concept preview (we'll implement it later)
- **Why This Matters for Games**: Entity lists, particle systems, inventory, bullet pools, tile arrays -- games are full of dynamically-sized collections.

**References**:
- [The Rust Book - Ch 8.1: Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rust by Example - Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)

### Day 13: Strings (2-3 hours)

- [ ] **Chapter 8 (Part 2): Strings**
  - [ ] `String` vs `&str` -- owned vs borrowed
  - [ ] Creating and modifying strings
  - [ ] Why you can't index strings (UTF-8 encoding)
  - [ ] String slicing and iteration
  - [ ] `format!` macro
- [ ] **Rustlings**: Complete `strings1`, `strings2`, `strings3`, `strings4`
- [ ] **Game Exercise**: Build a **dialogue system**
  - NPC dialogue stored as `Vec<String>`
  - Template system: `format!("{} says: {}", npc.name, dialogue)`
  - Parse player text commands into actions
  - Display formatted combat log messages
- **Why This Matters for Games**: Dialogue, UI text, debug logs, save file names, asset paths -- strings are everywhere in games. Understanding `String` vs `&str` avoids unnecessary allocations in hot loops.

**References**:
- [The Rust Book - Ch 8.2: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust by Example - Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)

### Day 14: Hash Maps (2-3 hours)

- [ ] **Chapter 8 (Part 3): Hash Maps**
  - [ ] Creating and using `HashMap<K, V>`
  - [ ] Ownership rules with maps
  - [ ] `.entry()` API for insert-or-update
  - [ ] Iterating over maps
- [ ] **Rustlings**: Complete `hashmaps1`, `hashmaps2`, `hashmaps3`
- [ ] **Game Exercise**: Build a **game world map**
  - `HashMap<(i32, i32), Room>` -- coordinate-based world
  - `HashMap<String, u32>` -- player stats (str, dex, con, etc.)
  - `HashMap<String, Vec<Item>>` -- shop inventories
  - Implement `entry().or_insert()` for a scoring system
- [ ] **Weekend Project**: Expand your text RPG into a **multi-room adventure**
  - World map using HashMap
  - Full inventory system with Vec
  - Dialogue system with formatted strings
  - At least 5 rooms, 3 enemy types, 5 item types
- **Why This Matters for Games**: Tilemaps, entity registries, asset caches, keybinding maps, config settings -- HashMaps are the backbone of game data lookup.

**References**:
- [The Rust Book - Ch 8.3: Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust by Example - HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)

---

## Week 5: Error Handling, Generics, Traits, and Lifetimes

*Build robust game systems. Traits are the foundation of game engine architecture.*

### Day 15: Error Handling (3 hours)

- [ ] **Chapter 9: Error Handling**
  - [ ] `panic!` -- unrecoverable errors
  - [ ] `Result<T, E>` -- recoverable errors
  - [ ] `unwrap()`, `expect()`, and the `?` operator
  - [ ] Custom error types
  - [ ] When to panic vs when to return Result
- [ ] **Rustlings**: Complete `errors1` through `errors6`
- [ ] **Game Exercise**: Add error handling to your text RPG
  - `fn load_room(id: &str) -> Result<Room, GameError>` for missing rooms
  - `fn parse_command(input: &str) -> Result<Action, ParseError>` for invalid input
  - Use `?` to propagate errors up through game systems
  - Handle save/load with proper error management
- **Why This Matters for Games**: Games load assets (textures, sounds, levels) that can fail. Network calls can fail. Parse errors happen. `Result` forces you to handle every failure path, so your game doesn't crash unexpectedly.

**References**:
- [The Rust Book - Ch 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [thiserror crate](https://docs.rs/thiserror/) -- derive macro for custom errors
- [anyhow crate](https://docs.rs/anyhow/) -- flexible error handling for applications

### Day 16: Generics (2-3 hours)

- [ ] **Chapter 10 (Part 1): Generic Types**
  - [ ] Generic functions: `fn largest<T>(list: &[T]) -> &T`
  - [ ] Generic structs and enums
  - [ ] Generic method definitions
  - [ ] Monomorphization -- why generics have zero runtime cost
- [ ] **Rustlings**: Complete `generics1`, `generics2`
- [ ] **Game Exercise**: Build **generic game containers**
  - `struct Grid<T> { width: usize, height: usize, cells: Vec<T> }` -- works for tiles, entities, fog-of-war
  - `impl<T> Grid<T> { fn get(&self, x: usize, y: usize) -> Option<&T> }`
  - `fn find_nearest<T: HasPosition>(entities: &[T], target: &Position) -> Option<&T>`
- **Why This Matters for Games**: A `Grid<Tile>` for the map, a `Grid<bool>` for collision, a `Grid<f32>` for a heatmap -- same code, different types, zero overhead.

**References**:
- [The Rust Book - Ch 10.1: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)

### Day 17: Traits (3 hours)

- [ ] **Chapter 10 (Part 2): Traits**
  - [ ] Defining and implementing traits
  - [ ] Default implementations
  - [ ] Traits as function parameters (`impl Trait` syntax)
  - [ ] Trait bounds and `where` clauses
  - [ ] Returning types that implement traits
- [ ] **Rustlings**: Complete `traits1` through `traits5`
- [ ] **Game Exercise**: Define **game behavior traits**
  - `trait Drawable { fn draw(&self); fn z_order(&self) -> i32 { 0 } }`
  - `trait Updatable { fn update(&mut self, delta_time: f64); }`
  - `trait Collidable { fn bounding_box(&self) -> Rect; fn on_collision(&mut self, other: &dyn Collidable); }`
  - Implement these traits for Player and Enemy
  - Write `fn update_all(entities: &mut [&mut dyn Updatable], dt: f64)`
- **Why This Matters for Games**: Traits define what game objects *can do*. Anything `Drawable` can be rendered. Anything `Collidable` participates in physics. This is the foundation of game engine architecture (and exactly how Bevy's ECS works under the hood).

**References**:
- [The Rust Book - Ch 10.2: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)

### Day 18: Lifetimes (3 hours)

- [ ] **Chapter 10 (Part 3): Lifetimes**
  - [ ] Why lifetimes exist -- preventing dangling references
  - [ ] Lifetime annotation syntax (`'a`)
  - [ ] Lifetimes in function signatures
  - [ ] Lifetime elision rules (when you can skip annotations)
  - [ ] Lifetimes in structs
  - [ ] The `'static` lifetime
- [ ] **Rustlings**: Complete `lifetimes1`, `lifetimes2`, `lifetimes3`
- [ ] **Game Exercise**: Practice lifetimes in game contexts
  - `fn get_closest_enemy<'a>(enemies: &'a [Enemy], pos: &Position) -> Option<&'a Enemy>`
  - `struct GameView<'a> { player: &'a Player, visible_enemies: Vec<&'a Enemy> }` -- a temporary view of game state
  - Understand why game objects usually own their data (avoiding lifetime complexity)
- **Why This Matters for Games**: Lifetimes ensure references into game state don't outlive what they point to. In practice, game code often avoids complex lifetimes by owning data or using indices/IDs, but understanding lifetimes helps you know *why* those patterns exist.

**References**:
- [The Rust Book - Ch 10.3: Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust by Example - Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

### Day 19: Testing (2-3 hours)

- [ ] **Chapter 11: Writing Automated Tests**
  - [ ] `#[test]` attribute, `assert!`, `assert_eq!`, `assert_ne!`
  - [ ] `should_panic` tests
  - [ ] `Result<T, E>` in tests
  - [ ] `cargo test`, test organization (unit vs integration)
- [ ] **Rustlings**: Complete `tests1` through `tests4`
- [ ] **Game Exercise**: Test your game systems
  - Test combat: `assert_eq!(player.health, 80)` after taking 20 damage
  - Test inventory: add item, remove item, check contains
  - Test grid: get/set cells, bounds checking
  - Test command parser: valid and invalid inputs
- **Why This Matters for Games**: Game logic is complex. Tests catch regressions when you add features. Test your combat formulas, collision math, and state machines.

**References**:
- [The Rust Book - Ch 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)

### Day 20: Phase 1 Capstone -- Complete Text RPG (3 hours)

- [ ] **Combine everything from Weeks 1-5** into a polished text RPG:
  - [ ] Modular code structure (player, enemy, world, combat, items modules)
  - [ ] At least 10 rooms connected via HashMap world map
  - [ ] 3+ enemy types with different behaviors (enum + match)
  - [ ] Inventory system with Vec
  - [ ] Combat system with traits
  - [ ] Error handling for all user input
  - [ ] Tests for core game logic
  - [ ] Save/load game state to a file (using `std::fs` and `serde_json`)
- [ ] **Retrospective**: What Rust concepts clicked? Which need more review?

**References**:
- [serde_json crate](https://docs.rs/serde_json/) -- JSON serialization
- [serde crate](https://serde.rs/) -- Rust's serialization framework

---

# Phase 2: 2D Game Development with macroquad (Weeks 6-9)

*Time to put pixels on screen. macroquad is chosen for its simplicity -- minimal boilerplate, immediate-mode API, perfect for learning.*

---

## Week 6: Your First Graphical Game

### Day 21: macroquad Setup and Game Loop (3 hours)

- [ ] **Introduction to macroquad**
  - [ ] Add `macroquad` to `Cargo.toml`
  - [ ] The game loop: `loop { clear_background(); ... next_frame().await; }`
  - [ ] Coordinate system (origin top-left, y increases downward)
  - [ ] Drawing shapes: `draw_rectangle`, `draw_circle`, `draw_line`
  - [ ] Colors: `RED`, `GREEN`, `Color::new(r, g, b, a)`
  - [ ] `get_frame_time()` for delta time
- [ ] **Game Exercise**: Draw a moving rectangle controlled by arrow keys
  - Implement basic movement with `is_key_down()`
  - Apply delta time for frame-rate-independent movement
  - Keep the player within screen bounds
- **Why This Matters**: The game loop is the heartbeat of every game. Understanding delta time prevents your game from running at different speeds on different machines.

**References**:
- [macroquad getting started](https://macroquad.rs/articles/fish-tutorial/)
- [macroquad API docs](https://docs.rs/macroquad/)
- [macroquad examples](https://github.com/not-fl3/macroquad/tree/master/examples)

### Day 22: Input Handling and Game State (2-3 hours)

- [ ] **Input and State Management**
  - [ ] Keyboard input: `is_key_down`, `is_key_pressed` (held vs just pressed)
  - [ ] Mouse input: `mouse_position()`, `is_mouse_button_pressed()`
  - [ ] Game state machine using enums: `Menu -> Playing -> Paused -> GameOver`
  - [ ] Structuring game state in a `Game` struct
- [ ] **Game Exercise**: Build a **click-target game**
  - Random circles appear on screen
  - Click them to score points
  - Timer counts down
  - State transitions: Menu -> Playing -> GameOver -> Menu
  - Display score with `draw_text()`
- **Why This Matters**: Every game needs input handling and state management. The enum-based state machine pattern scales from simple to complex games.

**References**:
- [macroquad input API](https://docs.rs/macroquad/latest/macroquad/input/index.html)
- [Game Programming Patterns - State](https://gameprogrammingpatterns.com/state.html) (free online book)

### Day 23: Sprites and Textures (2-3 hours)

- [ ] **Loading and Drawing Images**
  - [ ] `load_texture("path.png").await`
  - [ ] `draw_texture()` and `draw_texture_ex()` for scale/rotation
  - [ ] Sprite sheets and `DrawTextureParams` for source rectangles
  - [ ] Basic animation: cycling through sprite sheet frames
  - [ ] `Texture2D` ownership and sharing
- [ ] **Game Exercise**: Animate a character
  - Load a sprite sheet (find free assets at [itch.io](https://itch.io/game-assets/free) or [OpenGameArt](https://opengameart.org/))
  - Implement frame-based animation (cycle every N milliseconds)
  - Move the character with arrow keys, flip sprite based on direction
- **Why This Matters**: Sprites are the visual building blocks of 2D games. Understanding sprite sheets and animation timing is fundamental.

**References**:
- [macroquad texture API](https://docs.rs/macroquad/latest/macroquad/texture/index.html)
- [OpenGameArt.org](https://opengameart.org/) -- free game assets
- [Kenney.nl](https://kenney.nl/assets) -- high-quality free assets
- [itch.io game assets](https://itch.io/game-assets/free)

### Day 24: Collision Detection (3 hours)

- [ ] **Basic 2D Collision**
  - [ ] AABB (Axis-Aligned Bounding Box) collision
  - [ ] Circle-circle collision
  - [ ] Circle-rectangle collision
  - [ ] Collision response (bouncing, stopping)
  - [ ] Spatial considerations (checking all pairs vs optimized approaches)
- [ ] **Game Exercise**: Build a **Pong clone**
  - Two paddles (rectangles), one ball (circle or rectangle)
  - Ball bounces off paddles and top/bottom walls
  - Score tracking when ball passes a paddle
  - Implement all collision detection from scratch
  - AI opponent: paddle follows ball y-position
- **Why This Matters**: Collision detection is the core of game physics. Understanding AABB and circle collision covers 90% of 2D game needs.

**References**:
- [2D Collision Detection (MDN)](https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection)
- [Game Programming Patterns - Game Loop](https://gameprogrammingpatterns.com/game-loop.html)

---

## Week 7: Building Game Systems

### Day 25: Particle Systems and Effects (2-3 hours)

- [ ] **Particles in Rust**
  - [ ] `struct Particle { position: Vec2, velocity: Vec2, lifetime: f32, color: Color }`
  - [ ] `Vec<Particle>` as a particle pool
  - [ ] Updating: move, age, remove dead particles
  - [ ] `Vec::retain()` for efficient removal
  - [ ] Spawning patterns: burst, continuous, directional
- [ ] **Game Exercise**: Add particle effects to Pong
  - Sparks on ball-paddle collision
  - Trail behind the ball
  - Explosion of particles on scoring
- **Why This Matters**: Particles make games feel alive. This exercise also practices Vec manipulation, struct design, and iterating over mutable collections -- core Rust skills.

**References**:
- [Game Programming Patterns - Object Pool](https://gameprogrammingpatterns.com/object-pool.html)
- [Particles tutorial concept](https://macroquad.rs/articles/particles/)

### Day 26: Tile Maps (3 hours)

- [ ] **Building a Tile-Based World**
  - [ ] `struct TileMap { width: usize, height: usize, tiles: Vec<TileType> }`
  - [ ] 2D indexing: `tiles[y * width + x]`
  - [ ] Loading tile maps from data (text file or embedded)
  - [ ] Camera offset for scrolling
  - [ ] Tile-based collision detection
- [ ] **Game Exercise**: Build a **scrolling tilemap renderer**
  - Define tile types: `enum TileType { Floor, Wall, Water, Lava }`
  - Load a map from a text file (`.` = floor, `#` = wall, etc.)
  - Render with camera following the player
  - Tile collision: player can't walk through walls
- **Why This Matters**: Tile maps are the foundation of platformers, RPGs, strategy games, and most 2D games. The `Vec` + index pattern is cache-friendly and fast.

**References**:
- [Tiled Map Editor](https://www.mapeditor.org/) -- popular tilemap editor
- [macroquad tiled example](https://github.com/not-fl3/macroquad/tree/master/examples)

### Day 27: Audio and UI (2-3 hours)

- [ ] **Sound and Text**
  - [ ] Loading and playing sounds with macroquad's audio API
  - [ ] Background music vs sound effects
  - [ ] Drawing text: `draw_text()`, font size, positioning
  - [ ] Basic UI: health bars, score display, menus
  - [ ] UI layout: centering, anchoring to screen edges
- [ ] **Game Exercise**: Add polish to your Pong clone
  - Sound effects for paddle hit, wall bounce, scoring
  - Score display centered at top
  - Health/lives bar
  - "Press SPACE to start" menu screen
  - "Game Over" screen with final score
- **Why This Matters**: Audio and UI transform a tech demo into a game. Players notice missing sound effects and bad UI more than they notice good code.

**References**:
- [macroquad audio API](https://docs.rs/macroquad/latest/macroquad/audio/index.html)
- [macroquad text/font API](https://docs.rs/macroquad/latest/macroquad/text/index.html)
- [freesound.org](https://freesound.org/) -- free sound effects

### Day 28: Game Architecture Patterns (3 hours)

- [ ] **Structuring a Larger Game**
  - [ ] The `Game` struct pattern: centralized state
  - [ ] State machine pattern for screens (Menu, Playing, Paused, GameOver)
  - [ ] Entity storage: `Vec<Entity>` with enum variants vs separate Vecs
  - [ ] Update-then-draw loop separation
  - [ ] Handling entity removal during iteration (collect indices, remove in reverse)
- [ ] **Game Exercise**: Refactor Pong into clean architecture
  - Separate game state from rendering
  - Implement proper state machine for game screens
  - Add pause functionality (press P to pause)
- [ ] **Deep Dive**: Read [Game Programming Patterns](https://gameprogrammingpatterns.com/) chapters on Game Loop, Update Method, Component
- **Why This Matters**: Good architecture makes games easier to extend. Adding a new enemy type should be easy. Adding a new screen should be trivial.

**References**:
- [Game Programming Patterns (free online)](https://gameprogrammingpatterns.com/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

---

## Week 8-9: Breakout Clone (Phase 2 Capstone)

### Day 29: Breakout -- Core Mechanics (3 hours)

- [ ] **Setup and Ball Physics**
  - [ ] Paddle at bottom, controlled by mouse or keyboard
  - [ ] Ball with velocity, bouncing off walls and paddle
  - [ ] Angle-based reflection off paddle (hit left = bounce left)
  - [ ] Speed increase over time
- [ ] **Implement**: Basic game loop with paddle and ball movement

### Day 30: Breakout -- Bricks and Collision (3 hours)

- [ ] **Brick Grid**
  - [ ] `Vec<Brick>` arranged in rows and columns
  - [ ] Different brick types: `enum BrickType { Normal, Hard(u8), Unbreakable }`
  - [ ] Ball-brick collision detection
  - [ ] Brick destruction and scoring
  - [ ] Collision response: which side was hit? (affects bounce direction)
- [ ] **Implement**: Full brick grid with destruction

### Day 31: Breakout -- Power-ups (3 hours)

- [ ] **Power-up System**
  - [ ] `enum PowerUp { MultiBall, WidePaddle, LaserPaddle, SlowBall, ExtraLife }`
  - [ ] Power-ups drop from destroyed bricks (random chance)
  - [ ] Power-up falls, collected by paddle
  - [ ] Timed effects (wide paddle lasts 10 seconds)
  - [ ] Multiple balls: `Vec<Ball>` instead of single Ball
- [ ] **Implement**: At least 3 power-up types

### Day 32: Breakout -- Levels and Polish (3 hours)

- [ ] **Level System**
  - [ ] Load level layouts from data
  - [ ] Multiple levels with increasing difficulty
  - [ ] Level transition screen
  - [ ] Lives system
- [ ] **Polish**
  - [ ] Particle effects on brick destruction
  - [ ] Sound effects for every event
  - [ ] Score display, lives display
  - [ ] Background music
  - [ ] Screen shake on ball loss

### Day 33: Breakout -- Final Polish and Review (3 hours)

- [ ] **Complete the Game**
  - [ ] High score tracking (save to file)
  - [ ] Main menu, pause screen, game over screen
  - [ ] At least 5 levels
  - [ ] Code review: clean module structure, good naming, tests for game logic
- [ ] **Retrospective**: Compare your Breakout code to your text RPG. Notice how much cleaner your Rust has gotten.

**References for the Breakout Project**:
- [Breakout game design](https://en.wikipedia.org/wiki/Breakout_(video_game))
- [macroquad Breakout example](https://macroquad.rs/articles/fish-tutorial/) (adapted)
- [Game feel and polish](https://www.youtube.com/watch?v=AJdEqssNZ-U) -- "Juice it or lose it" GDC talk

---

# Phase 3: Intermediate Rust & Bevy ECS (Weeks 10-14)

*Graduate to a real game engine. Learn intermediate Rust concepts motivated by ECS patterns.*

---

## Week 10: Introduction to Bevy

### Day 34: Bevy Fundamentals (3 hours)

- [ ] **What is ECS?**
  - [ ] Entity: just an ID
  - [ ] Component: data attached to entities (Position, Velocity, Sprite)
  - [ ] System: functions that operate on entities with specific components
  - [ ] Why ECS: cache-friendly, parallelizable, composable
- [ ] **Bevy Setup**
  - [ ] `cargo new bevy_game && cd bevy_game`
  - [ ] Add `bevy` to Cargo.toml (check [bevyengine.org](https://bevyengine.org/) for latest version)
  - [ ] Enable fast compiles: [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/)
  - [ ] `App::new().add_plugins(DefaultPlugins).run()`
- [ ] **Game Exercise**: Hello Bevy
  - [ ] Spawn a sprite (camera + sprite bundle)
  - [ ] Move it with keyboard input (system that queries `KeyboardInput`)
  - [ ] Understand Resources vs Components
- **Why This Matters**: Bevy is the most popular Rust game engine. Its ECS architecture is powerful but different from OOP. Understanding ECS early makes everything else click.

**References**:
- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/) -- essential companion
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- [ECS FAQ](https://github.com/SanderMertens/ecs-faq)

### Day 35: Components and Systems (3 hours)

- [ ] **Bevy ECS Deep Dive**
  - [ ] Defining components: `#[derive(Component)] struct Position(Vec2)`
  - [ ] Spawning entities: `commands.spawn((Position(...), Velocity(...), Sprite {...}))`
  - [ ] Writing systems: `fn movement(mut query: Query<(&mut Position, &Velocity)>)`
  - [ ] System ordering and stages
  - [ ] Resources: `Res<T>` and `ResMut<T>` for global state
- [ ] **Game Exercise**: Build a simple scene
  - [ ] Spawn player entity with Position, Velocity, and Sprite components
  - [ ] Spawn enemy entities
  - [ ] Movement system that applies Velocity to Position
  - [ ] Input system that modifies player Velocity
- **How This Connects to Rust**: Bevy systems use Rust's type system heavily. `Query<(&Position, &Velocity)>` is generic over the component types. The borrow checker ensures systems don't conflict.

**References**:
- [Bevy Cheat Book - ECS](https://bevy-cheatbook.github.io/programming/ecs-intro.html)
- [Bevy Cheat Book - Systems](https://bevy-cheatbook.github.io/programming/systems.html)

### Day 36: Events, States, and Resources (2-3 hours)

- [ ] **Bevy Patterns**
  - [ ] Events: `EventWriter<T>` and `EventReader<T>`
  - [ ] States: `States` enum for game screens
  - [ ] Resources: timer, score, configuration
  - [ ] Startup systems vs regular systems
  - [ ] System sets for ordering
- [ ] **Game Exercise**: Game state management in Bevy
  - [ ] `enum AppState { Menu, InGame, Paused, GameOver }`
  - [ ] Spawn/despawn entities on state transitions
  - [ ] Score resource that persists across states
  - [ ] Timer resource for gameplay mechanics

**References**:
- [Bevy Cheat Book - Events](https://bevy-cheatbook.github.io/programming/events.html)
- [Bevy Cheat Book - States](https://bevy-cheatbook.github.io/programming/states.html)
- [Bevy Cheat Book - Resources](https://bevy-cheatbook.github.io/programming/res.html)

---

## Week 11: Building a Game in Bevy

### Day 37: Bevy Sprites, Animation, and Input (3 hours)

- [ ] **2D Rendering in Bevy**
  - [ ] Sprite bundles and texture atlases
  - [ ] Sprite sheet animation with a timer
  - [ ] Camera setup: `Camera2d`
  - [ ] Input handling: `Res<ButtonInput<KeyCode>>`, `Res<ButtonInput<MouseButton>>`
- [ ] **Game Exercise**: Animated player character in Bevy
  - [ ] Load sprite sheet, define animation frames
  - [ ] Walk animation when moving, idle when still
  - [ ] Flip sprite based on direction
  - [ ] Camera follows player

**References**:
- [Bevy sprite example](https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_sheet.rs)
- [Bevy Cheat Book - 2D](https://bevy-cheatbook.github.io/2d/sprites.html)

### Day 38: Bevy Physics and Collision (3 hours)

- [ ] **Collision in Bevy**
  - [ ] Custom AABB collision system (reuse knowledge from macroquad)
  - [ ] Or use `bevy_rapier2d` / `avian2d` physics plugin
  - [ ] Collision events and responses
  - [ ] Layers and groups (what collides with what)
- [ ] **Game Exercise**: Implement collision for your Bevy game
  - [ ] Player-wall collision
  - [ ] Player-enemy collision (take damage)
  - [ ] Projectile-enemy collision (deal damage)
  - [ ] Collectible pickup

**References**:
- [avian physics engine](https://github.com/Jondolf/avian) -- Bevy physics plugin
- [Bevy collision example](https://github.com/bevyengine/bevy/blob/main/examples/2d/)

### Day 39: Bevy Audio and UI (2-3 hours)

- [ ] **Audio in Bevy**
  - [ ] Loading audio assets
  - [ ] Playing sound effects on events
  - [ ] Background music
- [ ] **UI in Bevy**
  - [ ] Bevy UI nodes (`Node`, `Text`, `Button`)
  - [ ] Flexbox-based layout
  - [ ] Button interactions
  - [ ] Health bars, score display
- [ ] **Game Exercise**: Build a complete HUD
  - [ ] Health bar that updates with player health
  - [ ] Score display
  - [ ] Menu with "Play" and "Quit" buttons
  - [ ] Sound effects for shooting, hitting, dying

**References**:
- [Bevy Cheat Book - Audio](https://bevy-cheatbook.github.io/audio.html)
- [Bevy Cheat Book - UI](https://bevy-cheatbook.github.io/programming/ui.html)
- [Bevy UI example](https://github.com/bevyengine/bevy/blob/main/examples/ui/)

---

## Week 12: Closures, Iterators, and Smart Pointers in Game Context

*These intermediate Rust concepts power Bevy's internals and your game systems.*

### Day 40: Closures (3 hours)

- [ ] **Chapter 13 (Part 1): Closures**
  - [ ] Closure syntax and type inference
  - [ ] Capturing environment: by reference, by mutable reference, by value
  - [ ] `Fn`, `FnMut`, `FnOnce` traits
  - [ ] `move` closures (used everywhere in Bevy)
- [ ] **Rustlings**: Complete `closures1`, `closures2`, `closures3`
- [ ] **Game Application**:
  - [ ] Bevy systems are closures: `app.add_systems(Update, |query: Query<&Position>| { ... })`
  - [ ] Event callbacks: `on_collision(|entity| { despawn(entity) })`
  - [ ] Sorting entities: `entities.sort_by(|a, b| a.z_order.cmp(&b.z_order))`

**References**:
- [The Rust Book - Ch 13.1: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)

### Day 41: Iterators (3 hours)

- [ ] **Chapter 13 (Part 2): Iterators**
  - [ ] The `Iterator` trait and `next()`
  - [ ] Consuming adaptors: `sum()`, `collect()`, `count()`
  - [ ] Iterator adaptors: `map()`, `filter()`, `zip()`, `enumerate()`
  - [ ] Chaining iterators
  - [ ] Why iterators are zero-cost abstractions
- [ ] **Rustlings**: Complete `iterators1` through `iterators5`
- [ ] **Game Application**:
  - [ ] Find all enemies within range: `enemies.iter().filter(|e| e.pos.distance(player.pos) < range)`
  - [ ] Calculate total score: `items.iter().map(|i| i.value).sum()`
  - [ ] Update all entities: `entities.iter_mut().for_each(|e| e.update(dt))`
  - [ ] Find nearest entity: `enemies.iter().min_by_key(|e| distance(e, player))`

**References**:
- [The Rust Book - Ch 13.2: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust by Example - Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)

### Day 42: Smart Pointers (3 hours)

- [ ] **Chapter 15: Smart Pointers**
  - [ ] `Box<T>` -- heap allocation, recursive types, trait objects
  - [ ] `Rc<T>` -- shared ownership (single-threaded)
  - [ ] `Arc<T>` -- shared ownership (thread-safe)
  - [ ] `RefCell<T>` -- interior mutability
  - [ ] `Deref` and `Drop` traits
  - [ ] When to use each
- [ ] **Rustlings**: Complete `box1`, `rc1`, `cow1`
- [ ] **Game Application**:
  - [ ] `Box<dyn State>` for game state trait objects
  - [ ] `Arc<Texture>` for shared textures across systems
  - [ ] `Rc<RefCell<EventQueue>>` for event systems (non-ECS pattern)
  - [ ] Why Bevy's ECS avoids most smart pointer needs (data lives in the World)
- **Why This Matters for Games**: Game engines share resources (textures, meshes) across many objects. Smart pointers manage this. But ECS architectures (Bevy) minimize the need by centralizing data.

**References**:
- [The Rust Book - Ch 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust by Example - Box](https://doc.rust-lang.org/rust-by-example/std/box.html)
- [Rust by Example - Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html)

---

## Week 13-14: Phase 3 Capstone -- Complete Bevy Game

*Build a top-down shooter OR platformer. Your choice based on what excites you.*

### Option A: Top-Down Shooter

### Day 43: Player and Movement (3 hours)
- [ ] Player sprite with WASD movement
- [ ] Mouse aiming (player faces cursor)
- [ ] Camera following player
- [ ] Smooth movement with acceleration/deceleration

### Day 44: Combat System (3 hours)
- [ ] Shooting projectiles toward mouse cursor
- [ ] Projectile lifetime and cleanup
- [ ] Enemy spawning (waves or continuous)
- [ ] Enemy AI: move toward player
- [ ] Projectile-enemy collision, damage, death

### Day 45: Level Design (3 hours)
- [ ] Tile-based level with walls
- [ ] Wall collision for player, enemies, and projectiles
- [ ] Multiple rooms or open arena
- [ ] Spawn points for enemies
- [ ] Collectibles (health, ammo, power-ups)

### Day 46: Polish (3 hours)
- [ ] Particle effects (muzzle flash, explosions, blood/sparks)
- [ ] Sound effects and music
- [ ] HUD: health, score, ammo, wave counter
- [ ] Screen shake on damage
- [ ] Enemy variety (fast/weak, slow/strong, ranged)

### Day 47: Final Polish (3 hours)
- [ ] Main menu, pause, game over screens
- [ ] High score persistence
- [ ] Difficulty scaling (enemies get tougher over time)
- [ ] Code review and cleanup
- [ ] Write tests for core game logic

### Option B: Platformer

### Day 43: Player and Physics (3 hours)
- [ ] Player with gravity, jump, and horizontal movement
- [ ] Coyote time (jump briefly after leaving platform)
- [ ] Variable jump height (hold to jump higher)
- [ ] Smooth acceleration and friction

### Day 44: Level and Collision (3 hours)
- [ ] Tile-based level with solid platforms
- [ ] Player-tile collision resolution
- [ ] One-way platforms (jump through from below)
- [ ] Moving platforms

### Day 45: Enemies and Hazards (3 hours)
- [ ] Walking enemies (patrol back and forth)
- [ ] Jump on enemies to defeat them
- [ ] Spikes, lava, falling hazards
- [ ] Health system or one-hit death with checkpoints

### Day 46: Collectibles and Progression (3 hours)
- [ ] Coins/gems to collect
- [ ] Door/key system for level progression
- [ ] Power-ups (double jump, dash)
- [ ] Multiple levels

### Day 47: Polish (3 hours)
- [ ] Animations: run, jump, fall, land, idle
- [ ] Particle effects: dust on landing, trail on dash
- [ ] Sound effects and music
- [ ] Menus, pause, transitions
- [ ] Code review and tests

**References for the Capstone**:
- [Bevy Game Template](https://github.com/NiklasEi/bevy_game_template)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/) -- your go-to reference
- [Game Feel (book)](https://www.game-feel.com/) -- making games feel good
- [Juice it or Lose it (GDC talk)](https://www.youtube.com/watch?v=Fy0aCDmgnxg)

---

# Phase 4: Advanced Rust for Games (Weeks 15-18)

*Push into advanced territory. These concepts unlock multiplayer, modding, procedural generation, and performance optimization.*

---

## Week 15: Concurrency for Games

### Day 48: Threads and Message Passing (3 hours)

- [ ] **Chapter 16 (Part 1-2): Threads and Channels**
  - [ ] `std::thread::spawn` and `join`
  - [ ] `move` closures with threads
  - [ ] Channels: `mpsc::channel()` for message passing
  - [ ] `Send` and `Sync` traits
- [ ] **Rustlings**: Complete `threads1`, `threads2`, `threads3`
- [ ] **Game Application**:
  - [ ] Background asset loading on a separate thread
  - [ ] AI computation off the main thread
  - [ ] Why Bevy handles parallelism for you (systems run in parallel when they don't conflict)

**References**:
- [The Rust Book - Ch 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust by Example - Threads](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)

### Day 49: Shared State and Async (3 hours)

- [ ] **Chapter 16 (Part 3): Shared State**
  - [ ] `Mutex<T>` for mutual exclusion
  - [ ] `Arc<Mutex<T>>` for thread-safe shared state
  - [ ] Deadlock awareness
- [ ] **Async Basics** (not in the Rust Book, but essential for games)
  - [ ] `async`/`await` syntax
  - [ ] `tokio` or `async-std` basics
  - [ ] Why macroquad uses `async fn main()`
- [ ] **Game Application**:
  - [ ] Async asset loading pipeline
  - [ ] Background save system
  - [ ] Chat system prototype for multiplayer

**References**:
- [The Rust Book - Ch 16.3: Shared State](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)

---

## Week 16: Unsafe, FFI, and Advanced Traits

### Day 50: Unsafe Rust (2-3 hours)

- [ ] **Chapter 19 (Part 1): Unsafe Rust**
  - [ ] What `unsafe` unlocks: raw pointers, unsafe functions, mutable statics
  - [ ] Creating safe abstractions over unsafe code
  - [ ] FFI: calling C code from Rust
- [ ] **Game Application**:
  - [ ] Why game engines use unsafe: GPU buffer manipulation, SIMD, FFI with C libraries
  - [ ] Reading Bevy source code that uses unsafe
  - [ ] When NOT to use unsafe in your game code

**References**:
- [The Rust Book - Ch 19.1: Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) -- advanced unsafe reference
- [Rust by Example - Unsafe](https://doc.rust-lang.org/rust-by-example/unsafe.html)

### Day 51: Advanced Traits and Patterns (3 hours)

- [ ] **Chapter 19 (Part 2-3): Advanced Traits & Types**
  - [ ] Associated types vs generics
  - [ ] Operator overloading (`Add`, `Mul` for Vec2 math)
  - [ ] Newtype pattern for type safety
  - [ ] Type aliases
  - [ ] The never type (`!`)
- [ ] **Chapter 18: Patterns and Matching**
  - [ ] Advanced pattern syntax: guards, bindings, nested patterns
  - [ ] Destructuring in function parameters
- [ ] **Game Application**:
  - [ ] Implement `Add`, `Sub`, `Mul` for a custom `Vec2` type
  - [ ] Newtype for units: `struct Pixels(f32)`, `struct Meters(f32)` -- prevent mixing coordinate systems
  - [ ] Advanced match for complex game event handling

**References**:
- [The Rust Book - Ch 19.2: Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [The Rust Book - Ch 18: Patterns](https://doc.rust-lang.org/book/ch18-00-patterns.html)

---

## Week 17: Networking and Multiplayer Basics

### Day 52: Networking Fundamentals (3 hours)

- [ ] **Rust Networking Basics**
  - [ ] `std::net::TcpStream` and `TcpListener`
  - [ ] Serialization with `serde` and `bincode`
  - [ ] Client-server architecture
  - [ ] Message protocols: define your packet types as enums
- [ ] **Game Exercise**: Build a **chat server and client**
  - [ ] Server accepts multiple connections
  - [ ] Clients send messages, server broadcasts to all
  - [ ] Serialize messages with serde

**References**:
- [Rust by Example - Networking (std::net)](https://doc.rust-lang.org/std/net/)
- [serde documentation](https://serde.rs/)
- [bincode crate](https://docs.rs/bincode/) -- fast binary serialization

### Day 53: Multiplayer Game Prototype (3 hours)

- [ ] **Multiplayer Architecture**
  - [ ] Client-server vs peer-to-peer
  - [ ] Game state synchronization
  - [ ] Handling latency: client-side prediction basics
  - [ ] Authoritative server model
- [ ] **Game Exercise**: Build a **simple multiplayer game**
  - [ ] Server manages game state
  - [ ] Two clients connect, each controls a player
  - [ ] Players see each other's position in real-time
  - [ ] Could be as simple as two dots moving on screen
- [ ] **Optional**: Explore `bevy_replicon` or `matchbox` for Bevy networking

**References**:
- [Gaffer On Games - Networking](https://gafferongames.com/) -- essential game networking articles
- [bevy_replicon](https://github.com/projectharmonia/bevy_replicon) -- Bevy multiplayer
- [matchbox](https://github.com/johanhelsing/matchbox) -- WebRTC peer-to-peer for Bevy

---

## Week 18: Macros and Procedural Generation

### Day 54: Macros (3 hours)

- [ ] **Chapter 19 (Part 5): Macros**
  - [ ] Declarative macros with `macro_rules!`
  - [ ] Repetition patterns (`$(...)*`)
  - [ ] Procedural macros overview (derive macros)
  - [ ] How Bevy uses derive macros (`#[derive(Component)]`, `#[derive(Resource)]`)
- [ ] **Rustlings**: Complete `macros` exercises
- [ ] **Game Exercise**: Write game-related macros
  - [ ] `spawn_enemy!(world, "goblin", 100, 5)` macro
  - [ ] `log_event!("Player {} took {} damage", name, amount)` with timestamp
  - [ ] Understand Bevy's derive macros by using them intentionally

**References**:
- [The Rust Book - Ch 19.5: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust by Example - Macros](https://doc.rust-lang.org/rust-by-example/macros.html)

### Day 55: Procedural Generation (3 hours)

- [ ] **Proc Gen for Games**
  - [ ] Random number generation with `rand` crate
  - [ ] Noise functions (Perlin/Simplex) with `noise` crate
  - [ ] Dungeon generation algorithms (BSP, random walk, cellular automata)
  - [ ] Seeded randomness for reproducible worlds
- [ ] **Game Exercise**: Build a **dungeon generator**
  - [ ] Generate a random dungeon with rooms and corridors
  - [ ] Place enemies and items randomly
  - [ ] Render the dungeon with macroquad or Bevy
  - [ ] Different seed = different dungeon
- **Why This Matters**: Procedural generation creates infinite replayability. Roguelikes, survival games, and open worlds all use it.

**References**:
- [rand crate](https://docs.rs/rand/)
- [noise crate](https://docs.rs/noise/)
- [Procedural Generation in Game Design (book)](https://www.routledge.com/Procedural-Generation-in-Game-Design/Short-Adams/p/book/9781498799195)
- [Roguelike Tutorial in Rust](https://bfnightly.bracketproductions.com/) -- Herbert Wolverson's excellent tutorial

---

# Phase 5: Capstone Project (Weeks 19-20)

*Design and build YOUR game. Apply everything you've learned.*

---

## Week 19: Design and Build

### Day 56-58: Game Design Document (Day 56) + Core Implementation (Days 57-58)

- [ ] **Day 56: Design Your Game** (3 hours)
  - [ ] Choose a genre you're excited about
  - [ ] Write a 1-page game design document:
    - Core mechanic (the one thing that makes it fun)
    - Player actions and goals
    - Art style (pixel art, simple shapes, etc.)
    - Scope: what's the minimum viable game?
  - [ ] Plan your ECS architecture: what components, systems, resources?
  - [ ] Create the Cargo project with Bevy

- [ ] **Day 57-58: Core Mechanics** (6 hours total)
  - [ ] Implement the core game loop
  - [ ] Player movement and primary action
  - [ ] Basic enemy/obstacle
  - [ ] Placeholder art (colored rectangles are fine)
  - [ ] Get the core mechanic feeling good

### Day 59-61: Systems and Content

- [ ] **Day 59: Game Systems** (3 hours)
  - [ ] Scoring/progression system
  - [ ] Health/damage system
  - [ ] Collision and physics
  - [ ] Audio integration

- [ ] **Day 60: Content** (3 hours)
  - [ ] Multiple enemy types or level variety
  - [ ] Power-ups or upgrades
  - [ ] At least 3 levels or escalating difficulty
  - [ ] Real sprite assets (from free asset sites or your own)

- [ ] **Day 61: UI and Menus** (3 hours)
  - [ ] Main menu
  - [ ] HUD (health, score, etc.)
  - [ ] Pause menu
  - [ ] Game over / victory screen

---

## Week 20: Polish and Ship

### Day 62-64: Polish

- [ ] **Day 62: Game Feel** (3 hours)
  - [ ] Screen shake
  - [ ] Particle effects
  - [ ] Hit freeze / slow motion on impact
  - [ ] Smooth camera
  - [ ] Tweened animations

- [ ] **Day 63: Audio and Visual Polish** (3 hours)
  - [ ] Complete sound effect coverage
  - [ ] Background music
  - [ ] Visual feedback for every player action
  - [ ] Consistent art style

- [ ] **Day 64: Bug Fixing and Testing** (3 hours)
  - [ ] Playtest and fix bugs
  - [ ] Edge cases: window resize, alt-tab, rapid input
  - [ ] Performance: check frame rate, optimize if needed
  - [ ] Write tests for critical game logic

### Day 65-66: Build and Share

- [ ] **Day 65: Distribution** (3 hours)
  - [ ] Build for your platform: `cargo build --release`
  - [ ] Optional: Build for web with WASM (`cargo build --target wasm32-unknown-unknown`)
  - [ ] Create a README for your game
  - [ ] Record a short gameplay GIF or video

- [ ] **Day 66: Retrospective** (2-3 hours)
  - [ ] What went well? What was hard?
  - [ ] Which Rust concepts do you now understand deeply?
  - [ ] Which need more work?
  - [ ] What would you do differently?
  - [ ] Plan your next game project

**References**:
- [Bevy WASM guide](https://bevy-cheatbook.github.io/platforms/wasm.html)
- [itch.io](https://itch.io/) -- publish your game for free
- [Rust GameDev Newsletter](https://gamedev.rs/newsletter/) -- stay connected

---

# Supplementary Materials

## Rustlings Progress Tracking

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
- [ ] Complete all `closures` exercises (1-3)
- [ ] Complete all `threads` exercises (1-3)
- [ ] Complete all `smart_pointers` exercises
- [ ] Complete all `macros` exercises
- [ ] Complete all remaining exercises

## Deep Understanding Checklist

### Rust Fundamentals
- [ ] I understand why Rust needs ownership and can explain it to someone else
- [ ] I can explain borrowing rules and why they prevent data races
- [ ] I know when to use `String` vs `&str` and why it matters for performance
- [ ] I understand stack vs heap and when each is used
- [ ] I can explain why Rust doesn't have null and how Option replaces it
- [ ] I understand trait objects vs generics and their performance trade-offs
- [ ] I know when to use `Box`, `Rc`, `Arc`, and `RefCell`
- [ ] I understand `Send` and `Sync` and why Rust's concurrency is safe
- [ ] I can explain lifetime annotations and when they're needed
- [ ] I understand zero-cost abstractions (generics, iterators) and monomorphization

### Game Development
- [ ] I can implement a game loop with fixed-timestep updates
- [ ] I understand delta time and frame-rate-independent movement
- [ ] I can implement AABB and circle collision detection
- [ ] I understand ECS architecture and why games use it
- [ ] I can build a complete game with menus, gameplay, and game over states
- [ ] I know how to structure game code with proper separation of concerns
- [ ] I understand sprite sheets, animation, and basic rendering
- [ ] I can implement basic game AI (follow, patrol, state machines)
- [ ] I understand client-server architecture for multiplayer
- [ ] I can use procedural generation to create game content

## Recommended Books

| Book | Focus | When to Read |
|------|-------|--------------|
| [The Rust Programming Language](https://doc.rust-lang.org/book/) | Rust fundamentals | Phase 1 (reference) |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | Practical Rust | Phase 1-2 (companion) |
| [Programming Rust, 2nd Ed](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) | Deep Rust | Phase 3+ |
| [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/) | Rust game dev | Phase 2-3 |
| [Game Programming Patterns](https://gameprogrammingpatterns.com/) | Game architecture | Phase 2+ (free online) |
| [Roguelike Tutorial in Rust](https://bfnightly.bracketproductions.com/) | Roguelike dev | Phase 3+ |

## Tips for Success

1. **Build games you want to play.** Motivation sustains you through hard concepts.
2. **Compiler errors are your teacher.** Read them carefully. Rust's error messages are the best in any language.
3. **Don't fight the borrow checker.** If you're struggling, your design might need rethinking. The compiler is usually right.
4. **Prototype with simple shapes first.** Get gameplay feeling good before adding art.
5. **Scope down ruthlessly.** A finished small game teaches more than an unfinished ambitious one.
6. **Join the community.** [r/rust_gamedev](https://reddit.com/r/rust_gamedev), Bevy Discord, and the Rust GameDev working group are welcoming.
7. **Play games analytically.** When you play a game, think about how it's implemented. "How did they do that particle effect?" "What's the collision system here?"

---

**Remember**: The goal is to learn Rust deeply while building things you're proud of. Game development provides endless motivation because you can *see and play* your progress. Every bouncing ball, every defeated enemy, every menu transition is proof that you're becoming a Rust developer.
