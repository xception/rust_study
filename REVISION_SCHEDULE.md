# Rust Game Dev - Revision Schedule

Spaced repetition tied to your game development milestones. Each review session connects Rust concepts back to the games you've built, reinforcing both language mastery and game dev skills.

## How to Use This Schedule

1. **Check off each session** as you complete it
2. **Use active recall**: try to explain concepts *before* checking notes
3. **Code from memory**: rebuild small game features without looking at your source
4. **Customize**: spend more time on topics you find difficult
5. **Don't skip reviews** -- they're when real learning solidifies

---

## Phase 1 Reviews: Rust Fundamentals (Weeks 1-5)

### Week 1 Review -- Variables, Types, Control Flow (1 hour)

- [ ] **Active Recall: Basics**
  - Write a dice roller from memory (variables, types, functions, loops)
  - Explain why Rust defaults to immutability -- how does this help in a game loop?
  - What's the difference between statements and expressions? Why does this matter for `match`?
- [ ] **Recode**: Rebuild your Day 3 combat round exercise from scratch without looking at old code
- [ ] **Rustlings Review**: Redo 3 exercises from `variables`, `functions`, or `if` without hints

### Week 2 Review -- Ownership & Borrowing (1.5 hours)

- [ ] **Active Recall: Ownership**
  - Recite the three ownership rules without looking
  - Draw a memory diagram: what happens when you assign a `String` to another variable?
  - Explain `&T` vs `&mut T` vs `T` -- when do you use each in game code?
  - Why can't you have `&T` and `&mut T` at the same time? Give a game example (renderer reading + physics writing).
- [ ] **Recode**: Rebuild your loot system and command parser from memory
- [ ] **Bug Hunt**: Intentionally write 3 programs that violate borrowing rules. Predict the error before compiling.
- [ ] **Rustlings Review**: Redo all `move_semantics` exercises

### Week 3 Review -- Structs, Enums, Matching (1.5 hours)

- [ ] **Active Recall: Types**
  - Define `Player`, `Enemy`, and `Item` structs from memory
  - Define a `GameState` enum with data variants
  - Write a `match` on `GameState` that handles every variant
  - Explain why `Option<T>` replaces null -- give a game example
- [ ] **Recode**: Rebuild your turn-based combat system from memory
- [ ] **Revisit Week 2**: Quick 15-minute ownership refresh
  - Can you still recite the three rules?
  - Draw one ownership diagram from memory
- [ ] **Rustlings Review**: Redo `structs`, `enums`, `match_expressions`

### Week 4 Review -- Modules & Collections (1.5 hours)

- [ ] **Active Recall: Collections**
  - When do you use `Vec<T>` vs `HashMap<K,V>` vs `[T; N]`?
  - Explain ownership rules with HashMap -- what happens when you insert a String?
  - Why can't you index a `String`? What do you do instead?
  - Sketch the module structure of your text RPG from memory
- [ ] **Recode**: Rebuild your inventory system (Vec) and world map (HashMap) from memory
- [ ] **Revisit Week 2**: Ownership & borrowing (15 minutes)
  - This is your first spaced repetition of the core concept
- [ ] **Rustlings Review**: Redo `vecs`, `hashmaps`, `strings`, `modules`

### Week 5 Review -- Error Handling, Generics, Traits, Lifetimes (2 hours)

- [ ] **Active Recall: Error Handling**
  - When should you `panic!` vs return `Result`?
  - Write a function using `?` from memory
  - Explain how `Result<T, E>` forces you to handle errors
- [ ] **Active Recall: Generics & Traits**
  - Write `struct Grid<T>` from memory with a `get` method
  - Define a `Drawable` trait and implement it for two types
  - Explain monomorphization -- why are generics zero-cost?
  - What's the difference between `impl Trait` and `dyn Trait`?
- [ ] **Active Recall: Lifetimes**
  - What do lifetime annotations mean?
  - Write `fn get_closest<'a>(list: &'a [Enemy], pos: &Position) -> &'a Enemy` from memory
  - Explain the lifetime elision rules
- [ ] **Revisit Week 3**: Structs & enums (15 minutes)
- [ ] **Rustlings Review**: Redo `errors`, `generics`, `traits`, `lifetimes`

---

## Phase 1 Comprehensive Review (End of Week 5, ~2.5 hours)

*You've completed the text RPG. This review covers all Rust fundamentals.*

- [ ] **Core Language** (45 minutes)
  - [ ] Ownership: explain with a diagram, give 2 game examples
  - [ ] Borrowing: explain the rules, give a game example of reading vs writing
  - [ ] Lifetimes: explain when they're needed and when elision handles them
  - [ ] Traits: define a game trait, implement it, use it as a parameter
- [ ] **Data Structures** (30 minutes)
  - [ ] Vec: create, push, iterate, filter -- game inventory example
  - [ ] HashMap: create, insert, lookup -- game world map example
  - [ ] String vs &str: explain with game dialogue example
- [ ] **Error Handling** (15 minutes)
  - [ ] Write a function that loads a game file and propagates errors with `?`
- [ ] **Game Design Review** (30 minutes)
  - [ ] Look at your text RPG code: what would you refactor now?
  - [ ] Identify places where you cloned unnecessarily
  - [ ] Identify places where you could use traits for abstraction
- [ ] **Self-Assessment** (30 minutes)
  - Rate yourself 1-5 on each topic (see tracking table below)
  - List topics that need more work before Phase 2
  - Plan extra practice for weak areas

---

## Phase 2 Reviews: macroquad & 2D Game Dev (Weeks 6-9)

### Week 6 Review -- Game Loop, Input, Sprites (1 hour)

- [ ] **Active Recall: Game Fundamentals**
  - Explain delta time and why it matters
  - Describe the game loop: input -> update -> draw
  - What's the difference between `is_key_down` and `is_key_pressed`?
  - Draw the coordinate system from memory (origin, axes)
- [ ] **Recode**: Rebuild the click-target game or basic movement demo from memory
- [ ] **Revisit Phase 1**: Ownership & borrowing (15 minutes -- should be getting automatic)

### Week 7 Review -- Collision, Tilemaps, Particles (1.5 hours)

- [ ] **Active Recall: Game Systems**
  - Implement AABB collision detection from memory
  - Implement circle-circle collision from memory
  - Explain 2D array indexing: `tiles[y * width + x]`
  - Describe the particle system lifecycle: spawn, update, remove dead
- [ ] **Recode**: Rebuild Pong's collision system from memory
- [ ] **Revisit Phase 1**: Traits & generics (15 minutes)

### Week 8-9 Review -- Breakout Clone Complete (2 hours)

- [ ] **Game Architecture Review**
  - [ ] Draw the module structure of your Breakout clone
  - [ ] List all structs and their responsibilities
  - [ ] List all enums (BrickType, PowerUp, GameState) and explain each
  - [ ] Identify the game loop structure: what runs in update vs draw?
- [ ] **Code Quality Review**
  - [ ] Are there unnecessary clones?
  - [ ] Are responsibilities well-separated?
  - [ ] Are error cases handled?
  - [ ] Could you add a new power-up type easily?
- [ ] **Revisit Phase 1**: All fundamentals (30-minute rapid review)
  - Ownership, borrowing, traits, error handling -- quick verbal review

---

## Phase 2 Comprehensive Review (End of Week 9, ~2 hours)

- [ ] **Rust Skills Check** (45 minutes)
  - [ ] Write a struct with methods from memory
  - [ ] Implement a trait for a custom type from memory
  - [ ] Write a function with proper error handling from memory
  - [ ] Use iterators to filter and transform a collection
- [ ] **Game Dev Skills Check** (45 minutes)
  - [ ] Explain the game loop and delta time
  - [ ] Implement AABB collision from scratch
  - [ ] Describe how to structure a game state machine
  - [ ] Explain camera/viewport concepts
- [ ] **Compare**: Look at your text RPG code (Phase 1) and your Breakout code (Phase 2). How has your Rust style improved?
- [ ] **Self-Assessment**: Update the tracking table below

---

## Phase 3 Reviews: Bevy & Intermediate Rust (Weeks 10-14)

### Week 10 Review -- ECS Fundamentals (1.5 hours)

- [ ] **Active Recall: ECS**
  - Explain Entity, Component, System in your own words
  - Why is ECS good for games? (cache-friendly, parallel, composable)
  - Write a Bevy system from memory: `fn movement(query: Query<(&mut Transform, &Velocity)>)`
  - Explain Resources vs Components
  - What are Events and when do you use them?
- [ ] **Recode**: Spawn an entity with components and write a movement system from memory
- [ ] **Revisit Phase 1**: Ownership (10 minutes -- should be second nature)

### Week 11 Review -- Bevy Rendering and Physics (1.5 hours)

- [ ] **Active Recall: Bevy 2D**
  - Explain sprite bundles and texture atlases
  - How do you handle input in Bevy? (`Res<ButtonInput<KeyCode>>`)
  - Describe how collision works in your Bevy game
  - How do you handle entity despawning safely?
- [ ] **Recode**: Build a simple "player moves, collects items" scene from memory
- [ ] **Revisit Phase 2**: Collision detection (15 minutes)

### Week 12 Review -- Closures, Iterators, Smart Pointers (2 hours)

- [ ] **Active Recall: Intermediate Rust**
  - [ ] Explain `Fn`, `FnMut`, `FnOnce` -- when is each used?
  - [ ] Chain 3 iterator methods to solve a game problem from memory
  - [ ] Explain `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>` -- when to use each?
  - [ ] Why does Bevy's ECS reduce the need for smart pointers?
- [ ] **Game Application**: Write game code from memory using these features
  - [ ] Use a closure to sort entities by distance
  - [ ] Use iterators to find all enemies in range
  - [ ] Explain where `Box<dyn Trait>` is used in game code
- [ ] **Revisit Phase 1**: Traits & lifetimes (15 minutes)
- [ ] **Rustlings Review**: Redo `closures`, `iterators`, `smart_pointers`

### Week 13-14 Review -- Capstone Game (2 hours)

- [ ] **Architecture Review**
  - [ ] Draw the ECS architecture: all components, systems, resources
  - [ ] List all state transitions in your game
  - [ ] Identify the most complex system -- could it be simplified?
- [ ] **Code Quality**
  - [ ] Are systems focused on a single responsibility?
  - [ ] Are components minimal (data only, no logic)?
  - [ ] Is the code well-organized into plugins/modules?
- [ ] **Retrospective**: What Rust concepts are now automatic? Which still require thought?

---

## Phase 3 Comprehensive Review (End of Week 14, ~2.5 hours)

- [ ] **Rust Mastery Check** (60 minutes)
  - [ ] Ownership, borrowing, lifetimes -- explain all three to an imaginary beginner
  - [ ] Generics, traits, trait objects -- write examples from memory
  - [ ] Closures, iterators -- chain operations fluently
  - [ ] Smart pointers -- explain trade-offs
  - [ ] Error handling -- design a custom error type
- [ ] **Game Dev Mastery Check** (60 minutes)
  - [ ] ECS: explain the pattern and its benefits
  - [ ] Implement a simple Bevy system from memory
  - [ ] Describe your game's architecture at a high level
  - [ ] Explain collision detection approaches
  - [ ] Discuss game state management patterns
- [ ] **Self-Assessment**: Update the tracking table

---

## Phase 4 Reviews: Advanced Rust (Weeks 15-18)

### Week 15 Review -- Concurrency (1.5 hours)

- [ ] **Active Recall: Threads and Async**
  - Explain `Send` and `Sync` traits
  - When would you use threads vs async in a game?
  - Explain `Arc<Mutex<T>>` pattern
  - Why is Bevy's concurrency safe by default?
- [ ] **Revisit**: Closures (closures + move are key for threads)
- [ ] **Rustlings Review**: Redo `threads`

### Week 16 Review -- Unsafe and Advanced Traits (1 hour)

- [ ] **Active Recall**
  - What does `unsafe` unlock? When is it appropriate?
  - Implement `Add` for a custom Vec2 from memory
  - Explain the newtype pattern and give a game example
  - What are associated types and how do they differ from generics?
- [ ] **Revisit**: Traits (15 minutes -- review the hierarchy from basic to advanced)

### Week 17 Review -- Networking (1.5 hours)

- [ ] **Active Recall: Multiplayer**
  - Explain client-server architecture for games
  - How do you serialize game state for network transmission?
  - What is an authoritative server?
  - Describe client-side prediction at a high level
- [ ] **Recode**: Rebuild the chat server from memory
- [ ] **Revisit**: Error handling (network code is full of Result)

### Week 18 Review -- Macros and Proc Gen (1 hour)

- [ ] **Active Recall**
  - Write a simple `macro_rules!` macro from memory
  - Explain how Bevy's derive macros work (at a high level)
  - Describe one procedural generation algorithm
  - Explain seeded randomness and why it matters
- [ ] **Revisit**: Iterators (proc gen uses them heavily)

---

## Phase 4 Comprehensive Review (End of Week 18, ~2 hours)

- [ ] **Advanced Rust Check** (60 minutes)
  - [ ] Concurrency: threads, channels, Arc, Mutex
  - [ ] Unsafe: when and why, safe abstractions
  - [ ] Advanced traits: associated types, operator overloading, newtype
  - [ ] Macros: write a declarative macro from memory
- [ ] **Advanced Game Dev Check** (60 minutes)
  - [ ] Networking: client-server, serialization
  - [ ] Proc gen: describe an algorithm, explain seeded randomness
  - [ ] Performance: where to optimize, profiling concepts
- [ ] **Self-Assessment**: Update the tracking table

---

## Phase 5 Review: Capstone (End of Week 20, ~2 hours)

- [ ] **Full Retrospective**
  - [ ] Compare your first code (Day 1 hello_world) to your capstone
  - [ ] List every Rust concept you used in your capstone game
  - [ ] What was the hardest bug you encountered? How did you fix it?
  - [ ] What would you build differently if starting over?
- [ ] **Final Self-Assessment**: Update all tracking table columns
- [ ] **Plan Next Steps**:
  - [ ] What kind of games do you want to build next?
  - [ ] Which Rust areas need continued study?
  - [ ] Are there Bevy plugins or crates you want to explore?
  - [ ] Consider contributing to Bevy or another open-source Rust game project

---

## Weekly Quick Reviews (15-20 minutes each)

Starting Week 2, do these **every Sunday**. The questions evolve as you progress.

### Weeks 2-5: Rust Fundamentals Focus

- [ ] Date: _______________
  - [ ] Recite the three ownership rules
  - [ ] Explain one Rust concept from this week using a game example
  - [ ] Redo one rustlings exercise from memory
  - [ ] Identify one thing you struggled with this week

- [ ] Date: _______________
  - [ ] Recite the three ownership rules
  - [ ] Explain one Rust concept from this week using a game example
  - [ ] Redo one rustlings exercise from memory
  - [ ] Identify one thing you struggled with this week

- [ ] Date: _______________
  - [ ] Recite the three ownership rules
  - [ ] Explain one Rust concept from this week using a game example
  - [ ] Redo one rustlings exercise from memory
  - [ ] Identify one thing you struggled with this week

- [ ] Date: _______________
  - [ ] Recite the three ownership rules
  - [ ] Explain one Rust concept from this week using a game example
  - [ ] Redo one rustlings exercise from memory
  - [ ] Identify one thing you struggled with this week

### Weeks 6-9: Game Dev Fundamentals Focus

- [ ] Date: _______________
  - [ ] Explain delta time and the game loop
  - [ ] Describe one game system you built this week
  - [ ] Identify a Rust pattern you used in game code
  - [ ] What would you refactor in this week's code?

- [ ] Date: _______________
  - [ ] Explain delta time and the game loop
  - [ ] Describe one game system you built this week
  - [ ] Identify a Rust pattern you used in game code
  - [ ] What would you refactor in this week's code?

- [ ] Date: _______________
  - [ ] Explain delta time and the game loop
  - [ ] Describe one game system you built this week
  - [ ] Identify a Rust pattern you used in game code
  - [ ] What would you refactor in this week's code?

- [ ] Date: _______________
  - [ ] Explain delta time and the game loop
  - [ ] Describe one game system you built this week
  - [ ] Identify a Rust pattern you used in game code
  - [ ] What would you refactor in this week's code?

### Weeks 10-14: Bevy & Intermediate Rust Focus

- [ ] Date: _______________
  - [ ] Explain an ECS concept (entity, component, system, resource, event)
  - [ ] Write a simple Bevy system from memory
  - [ ] Identify one intermediate Rust feature you used this week
  - [ ] What was the most useful compiler error you got?

- [ ] Date: _______________
  - [ ] Explain an ECS concept
  - [ ] Write a simple Bevy system from memory
  - [ ] Identify one intermediate Rust feature you used this week
  - [ ] What was the most useful compiler error you got?

- [ ] Date: _______________
  - [ ] Explain an ECS concept
  - [ ] Write a simple Bevy system from memory
  - [ ] Identify one intermediate Rust feature you used this week
  - [ ] What was the most useful compiler error you got?

- [ ] Date: _______________
  - [ ] Explain an ECS concept
  - [ ] Write a simple Bevy system from memory
  - [ ] Identify one intermediate Rust feature you used this week
  - [ ] What was the most useful compiler error you got?

- [ ] Date: _______________
  - [ ] Explain an ECS concept
  - [ ] Write a simple Bevy system from memory
  - [ ] Identify one intermediate Rust feature you used this week
  - [ ] What was the most useful compiler error you got?

### Weeks 15-20: Advanced & Capstone Focus

- [ ] Date: _______________
  - [ ] Explain one advanced Rust feature (concurrency, unsafe, macros)
  - [ ] How did this feature apply to your game?
  - [ ] What's the most elegant code you wrote this week?
  - [ ] What's the most important lesson from this week?

- [ ] Date: _______________
  - [ ] Same as above

- [ ] Date: _______________
  - [ ] Same as above

- [ ] Date: _______________
  - [ ] Same as above

- [ ] Date: _______________
  - [ ] Same as above

- [ ] Date: _______________
  - [ ] Same as above

_(Add more dates as you progress)_

---

## Topic-Specific Deep Dives (As Needed)

Use these when you identify a weak area. Each is a focused 1-hour session.

### Ownership Deep Dive
- [ ] Date: _______________
  - [ ] Reread [Ch 4 of the Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
  - [ ] Draw memory diagrams for 5 game scenarios (item transfer, entity creation, component access, function call, return value)
  - [ ] Explain ownership to an imaginary person using game analogies
  - [ ] Redo all `move_semantics` rustlings

### Lifetimes Deep Dive
- [ ] Date: _______________
  - [ ] Reread [lifetime sections](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
  - [ ] Write 5 functions with explicit lifetimes in game context
  - [ ] Explain the 3 lifetime elision rules
  - [ ] Read [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

### Traits Deep Dive
- [ ] Date: _______________
  - [ ] Define 5 game-related traits (Drawable, Updatable, Collidable, Serializable, Damageable)
  - [ ] Implement each for 2 different types
  - [ ] Compare `impl Trait` (static dispatch) vs `dyn Trait` (dynamic dispatch) -- benchmark both
  - [ ] Redo all `traits` rustlings

### ECS Deep Dive
- [ ] Date: _______________
  - [ ] Reread [Bevy Cheat Book ECS section](https://bevy-cheatbook.github.io/programming/ecs-intro.html)
  - [ ] Build a minimal ECS from scratch (just HashMap<EntityId, ComponentStorage>)
  - [ ] Explain why ECS is cache-friendly (data-oriented design)
  - [ ] Read the [ECS FAQ](https://github.com/SanderMertens/ecs-faq)

### Collision Deep Dive
- [ ] Date: _______________
  - [ ] Implement AABB, circle-circle, circle-AABB, and SAT collision from scratch
  - [ ] Explain collision response (reflection, penetration resolution)
  - [ ] Implement a spatial hash or quadtree for broad-phase collision
  - [ ] Read [Real-Time Collision Detection concepts](https://realtimecollisiondetection.net/)

### Concurrency Deep Dive
- [ ] Date: _______________
  - [ ] Reread [Ch 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
  - [ ] Implement both channel and mutex examples
  - [ ] Explain `Send` and `Sync` in your own words
  - [ ] Write a parallel asset loader using threads + channels

---

## Active Recall Questions (Use for Any Review Session)

### Rust: Ownership & Borrowing
- What are the three ownership rules?
- Why can you have multiple `&T` but only one `&mut T`?
- What happens to heap data when the owner goes out of scope?
- When is data copied vs moved?
- How does this apply to game entity management?

### Rust: Types & Traits
- What's the difference between `String` and `&str`?
- When should you use `Option<T>`? Give a game example.
- What makes a trait object-safe?
- How do generics achieve zero-cost abstraction?
- How do traits enable Bevy's ECS?

### Rust: Error Handling
- When should you `panic!` vs return `Result`?
- How does `?` work? Write an example from memory.
- How do you handle errors when loading game assets?

### Rust: Smart Pointers
- When should you use `Box<T>`?
- `Rc<T>` vs `Arc<T>` -- what's the difference?
- When would you use `RefCell<T>`?
- Why does Bevy's ECS reduce the need for these?

### Rust: Concurrency
- What are `Send` and `Sync`?
- Channels vs mutexes -- when to use each?
- Why is Rust's concurrency memory-safe?
- How does Bevy parallelize systems automatically?

### Game Dev: Fundamentals
- Explain the game loop (input -> update -> draw)
- What is delta time and why is it critical?
- Describe AABB collision detection
- What is a state machine in game context?

### Game Dev: ECS
- Explain Entity, Component, System
- Why is ECS cache-friendly?
- How do you handle cross-system communication in Bevy?
- When would you use Resources vs Components?

### Game Dev: Architecture
- How do you structure a game into modules?
- How do you handle entity removal during iteration?
- What is "game feel" and how do you implement it?
- How do you serialize game state for save/load?

---

## Progress Tracking

### Understanding Levels

Rate yourself on each topic. Update at the end of each phase.

**Rating Scale**:
- 1 = Don't understand at all
- 2 = Vaguely familiar, need to look up
- 3 = Understand basics, can use with documentation
- 4 = Comfortable, can use without documentation
- 5 = Deep understanding, can teach others

| Topic | Phase 1 | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|-------|---------|---------|---------|---------|---------|
| **Rust: Ownership** | __ | __ | __ | __ | __ |
| **Rust: Borrowing** | __ | __ | __ | __ | __ |
| **Rust: Lifetimes** | __ | __ | __ | __ | __ |
| **Rust: Structs & Enums** | __ | __ | __ | __ | __ |
| **Rust: Pattern Matching** | __ | __ | __ | __ | __ |
| **Rust: Error Handling** | __ | __ | __ | __ | __ |
| **Rust: Collections** | __ | __ | __ | __ | __ |
| **Rust: Generics** | __ | __ | __ | __ | __ |
| **Rust: Traits** | __ | __ | __ | __ | __ |
| **Rust: Closures** | __ | __ | __ | __ | __ |
| **Rust: Iterators** | __ | __ | __ | __ | __ |
| **Rust: Smart Pointers** | __ | __ | __ | __ | __ |
| **Rust: Concurrency** | __ | __ | __ | __ | __ |
| **Rust: Macros** | __ | __ | __ | __ | __ |
| **Game: Game Loop & Timing** | __ | __ | __ | __ | __ |
| **Game: Input Handling** | __ | __ | __ | __ | __ |
| **Game: Collision Detection** | __ | __ | __ | __ | __ |
| **Game: Sprites & Animation** | __ | __ | __ | __ | __ |
| **Game: Audio** | __ | __ | __ | __ | __ |
| **Game: UI/HUD** | __ | __ | __ | __ | __ |
| **Game: ECS (Bevy)** | __ | __ | __ | __ | __ |
| **Game: State Management** | __ | __ | __ | __ | __ |
| **Game: Networking** | __ | __ | __ | __ | __ |
| **Game: Proc Generation** | __ | __ | __ | __ | __ |

### Weak Areas to Focus On

Update this list as you identify areas needing more work:

1. _________________________________ (Date identified: _______)
2. _________________________________ (Date identified: _______)
3. _________________________________ (Date identified: _______)
4. _________________________________ (Date identified: _______)
5. _________________________________ (Date identified: _______)

---

## Post-Completion: Maintenance Phase

### Monthly Reviews (After Week 20)

#### Month 6 Review (30-60 minutes)
- [ ] Quick review of Rust fundamentals (should be instant now)
- [ ] Play your capstone game -- any bugs to fix?
- [ ] Read 2-3 posts from [Rust GameDev Newsletter](https://gamedev.rs/newsletter/)
- [ ] Explore a new Bevy plugin or Rust crate
- [ ] Start a new small game project

#### Month 7 Review (30-60 minutes)
- [ ] Same as Month 6
- [ ] Focus on one advanced topic you want to deepen
- [ ] Consider contributing to Bevy or another Rust game project

#### Month 8+ Reviews (30 minutes each)
- [ ] Review Rust concepts you use less often
- [ ] Follow Rust and Bevy release notes
- [ ] Build, build, build -- each game teaches more than any review session

### Continuous Learning
- [ ] Follow [Rust GameDev Newsletter](https://gamedev.rs/newsletter/)
- [ ] Follow [This Week in Rust](https://this-week-in-rust.org/)
- [ ] Join [Bevy Discord](https://discord.gg/bevy) and participate
- [ ] Watch [GDC talks](https://www.youtube.com/channel/UC0JB7TSe49lg56u6qH8y_MQ) for game design inspiration
- [ ] Play games analytically -- "how would I build this in Bevy?"

---

## Tips for Effective Revision

1. **Active Recall > Passive Reading**: Try to remember before checking notes. Rebuild game features from memory.
2. **Spaced Repetition**: Review just before you forget. The schedule is designed for this.
3. **Connect Rust to Games**: Every Rust concept has a game application. Always think "how does this help my game?"
4. **Code, Don't Just Read**: Rebuild exercises from memory. Type out systems without looking at old code.
5. **Explain Out Loud**: If you can teach it, you understand it. Explain to a rubber duck.
6. **Track Honestly**: The self-assessment table only helps if you're honest about weak areas.
7. **Don't Skip Reviews**: Even when you feel confident. That's when spaced repetition is most effective.

---

**Remember**: Every game you build compounds your understanding. Your 5th game will be dramatically better than your 1st, not because you memorized more syntax, but because you *think* in Rust. Trust the process, stay consistent, and have fun building games.
