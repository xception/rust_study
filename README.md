# Rust Through Game Development - A Complete Learning Journey

A structured, self-taught path to learning Rust by building games. Every concept is motivated by a real game development problem, so you never learn syntax in a vacuum -- you learn it because your game needs it.

## Learning Philosophy

- **Build games, not toy examples.** Every Rust concept is introduced because a game project demands it.
- **Understanding over memorization.** Focus on *why* Rust works this way, not just *what* the syntax is.
- **Consistent daily practice.** 2-3 hours per day of focused work compounds fast.
- **Spaced repetition.** Regular revision sessions prevent forgetting.
- **Progressive complexity.** Start with terminal games, graduate to 2D engines, end with a polished Bevy project.

## Repository Structure

```
rust_study/
├── README.md                  # You are here
├── LEARNING_PLAN.md           # Complete 20-week roadmap with exercises & references
├── REVISION_SCHEDULE.md       # Spaced repetition schedule tied to game milestones
│
├── phase1_fundamentals/       # Weeks 1-5: Core Rust via terminal games
│   ├── week1/                 #   Setup, variables, control flow
│   ├── week2/                 #   Ownership & borrowing
│   ├── week3/                 #   Structs, enums, pattern matching
│   ├── week4/                 #   Modules & collections
│   └── week5/                 #   Error handling, generics, traits, lifetimes
│
├── phase2_game_foundations/   # Weeks 6-9: 2D game dev with macroquad
│   ├── week6/                 #   Game loops, rendering, input
│   ├── week7/                 #   Sprites, animation, collision
│   ├── week8/                 #   Audio, UI, game states
│   └── week9/                 #   Complete game: Breakout clone
│
├── phase3_intermediate/       # Weeks 10-14: Bevy ECS & systems
│   ├── week10/                #   Bevy intro, ECS fundamentals
│   ├── week11/                #   Sprites, input, physics in Bevy
│   ├── week12/                #   Audio, UI, scenes in Bevy
│   ├── week13/                #   Closures, iterators, smart pointers in game context
│   └── week14/                #   Complete game: top-down shooter or platformer
│
├── phase4_advanced/           # Weeks 15-18: Advanced Rust for games
│   ├── week15/                #   Concurrency & async for games
│   ├── week16/                #   Unsafe, FFI, advanced traits
│   ├── week17/                #   Networking & multiplayer basics
│   └── week18/                #   Macros, procedural generation
│
└── phase5_capstone/           # Weeks 19-20: Capstone game project
    ├── week19/                #   Design & build
    └── week20/                #   Polish, publish, retrospective
```

## Roadmap at a Glance

| Phase | Weeks | Focus | Milestone Game |
|-------|-------|-------|----------------|
| **1 - Fundamentals** | 1-5 | Core Rust through terminal games | Text RPG with inventory |
| **2 - Game Foundations** | 6-9 | 2D game dev with macroquad | Breakout clone |
| **3 - Intermediate** | 10-14 | Bevy ECS, intermediate Rust | Top-down shooter or platformer |
| **4 - Advanced** | 15-18 | Concurrency, networking, macros | Multiplayer prototype |
| **5 - Capstone** | 19-20 | Full game from scratch | Your own game |

## Key Technologies

| Tool | Purpose | When Introduced |
|------|---------|-----------------|
| [Rust & Cargo](https://www.rust-lang.org/) | Language & build system | Week 1 |
| [Rustlings](https://github.com/rust-lang/rustlings) | Bite-sized exercises | Weeks 1-5 |
| [macroquad](https://macroquad.rs/) | Simple 2D game framework | Week 6 |
| [Bevy](https://bevyengine.org/) | Full ECS game engine | Week 10 |
| [wgpu](https://wgpu.rs/) | Low-level GPU (reference) | Week 15+ |

## Getting Started

1. **Install Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` (or [rustup.rs](https://rustup.rs/) on Windows)
2. **Clone rustlings**: `cargo install rustlings` then `rustlings init`
3. **Open `LEARNING_PLAN.md`**: Start with Phase 1, Week 1, Day 1
4. **Follow `REVISION_SCHEDULE.md`**: Review regularly to lock in knowledge
5. **Build every project**: Type the code yourself, experiment, break things

## Progress Tracking

### Current Status
- **Phase**: __ of 5
- **Week**: __ of 20
- **Current Day**: __ of 70

### Milestones
- [ ] Phase 1 complete -- Core Rust fundamentals, text RPG built
- [ ] Phase 2 complete -- 2D game dev basics, Breakout clone shipped
- [ ] Phase 3 complete -- Bevy proficiency, intermediate game shipped
- [ ] Phase 4 complete -- Advanced Rust, multiplayer prototype
- [ ] Phase 5 complete -- Capstone game designed, built, and polished
- [ ] All rustlings exercises completed

## Resources

### Rust Language
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) -- the canonical reference
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) -- learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) -- small exercises
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [This Week in Rust](https://this-week-in-rust.org/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

### Game Development in Rust
- [Are We Game Yet?](https://arewegameyet.rs/) -- Rust gamedev ecosystem overview
- [Bevy Book](https://bevyengine.org/learn/book/introduction/) -- official Bevy guide
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/) -- unofficial but excellent
- [macroquad examples](https://macroquad.rs/examples/) -- learn by example
- [Rust GameDev Working Group](https://gamedev.rs/) -- news and resources
- [Rust GameDev Newsletter](https://gamedev.rs/newsletter/) -- monthly updates
- [Hands-on Rust (book)](https://pragprog.com/titles/hwrust/hands-on-rust/) -- Herbert Wolverson's bracket-lib guide

### Community
- [r/rust](https://reddit.com/r/rust) and [r/rust_gamedev](https://reddit.com/r/rust_gamedev)
- [Rust Discord](https://discord.gg/rust-lang) -- #gamedev channel
- [Bevy Discord](https://discord.gg/bevy)

---

**Start Date**: _______________
**Target Completion**: _______________ (approximately 20 weeks)
