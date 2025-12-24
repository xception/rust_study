# Rust Learning Revision Schedule

This revision schedule uses **spaced repetition** to help you remember concepts better. Research shows that reviewing material at increasing intervals significantly improves long-term retention.

## How to Use This Schedule

1. **Check off each revision session** as you complete it
2. **Focus on understanding, not memorizing** - try to explain concepts in your own words
3. **Customize based on your weak areas** - spend more time on topics you find difficult
4. **Be honest with yourself** - if you can't explain a concept, review it again
5. **Use active recall** - test yourself before looking at notes

---

## Week 1: Foundation Building (Days 1-7)

### Week 1 - Day 7 Review (End of Week)
- [ ] **Quick Review**: Variables & Mutability
  - Can you explain why Rust defaults to immutability?
  - Write a program with mutable and immutable variables from memory
- [ ] **Quick Review**: Ownership Rules
  - Recite the three ownership rules without looking
  - Explain what happens when a variable goes out of scope
- [ ] **Quick Review**: References & Borrowing
  - Draw a diagram of borrowing
  - Explain the borrowing rules to an imaginary beginner
- [ ] **Practice**: Solve 3 rustlings exercises from Week 1 again without hints

**Time**: 1 hour

---

## Week 2: Ownership Consolidation (Days 8-14)

### Week 2 - Mid-week Check (Day 11)
- [ ] **Concept Review**: Ownership & Borrowing
  - Explain when to use `&T` vs `&mut T` vs `T`
  - What problems does ownership solve?
- [ ] **Code Review**: Look at code from Days 4-6
  - Can you refactor it to be more idiomatic?
  - Identify any unnecessary clones

**Time**: 45 minutes

### Week 2 - Day 14 Review (End of Week)
- [ ] **Comprehensive Review**: Weeks 1-2 Material
  - [ ] Ownership (all three aspects)
  - [ ] Structs and methods
  - [ ] Enums and pattern matching
  - [ ] Option and match exhaustiveness
- [ ] **Deep Questions**:
  - Why can't you have multiple mutable references?
  - What's the difference between String and &str?
  - When would you use enums vs structs?
- [ ] **Practice**: Build a mini-project combining these concepts
  - Example: A library system with books (struct) and availability status (enum)

**Time**: 2 hours

---

## Week 3: Collections & Modules (Days 15-21)

### Week 3 - Day 21 Review (End of Week)
- [ ] **Review**: All Collection Types
  - [ ] Vec<T>: When to use, common operations
  - [ ] String vs &str: memory representation
  - [ ] HashMap: Ownership rules with collections
- [ ] **Review**: Module System
  - [ ] Explain visibility rules
  - [ ] When to use `pub` vs `pub(crate)` vs private
- [ ] **Revisit Week 1-2**: Quick 30-minute review
  - Go through checklist of Week 1-2 concepts
  - Identify anything you've forgotten
- [ ] **Practice**: Refactor previous code with better module organization

**Time**: 1.5 hours

---

## Week 4: Error Handling & Generics (Days 22-28)

### Week 4 - Day 28 Review (End of Week)
- [ ] **Review**: Error Handling Philosophy
  - [ ] When to use `panic!` vs `Result`
  - [ ] How does `?` operator work?
  - [ ] Explain error propagation
- [ ] **Review**: Generics & Traits
  - [ ] Write a generic function from scratch
  - [ ] Explain trait bounds
  - [ ] What is monomorphization?
- [ ] **Revisit Week 2**: Ownership & Borrowing (30 min)
  - This is your first spaced repetition of core concepts
- [ ] **Practice**: Implement a generic data structure with trait bounds

**Time**: 2 hours

---

## Month 1 Comprehensive Review (Day 30)

### Major Review Session
- [ ] **Fundamental Concepts** (45 minutes)
  - [ ] Ownership rules (explain with diagrams)
  - [ ] Borrowing rules (provide examples)
  - [ ] Lifetimes (basic understanding)
- [ ] **Type System** (30 minutes)
  - [ ] Structs, enums, and pattern matching
  - [ ] Generics and traits
  - [ ] Type inference
- [ ] **Collections & Strings** (20 minutes)
  - [ ] When to use each collection type
  - [ ] UTF-8 and string handling
- [ ] **Error Handling** (15 minutes)
  - [ ] Result and Option
  - [ ] Error propagation patterns
- [ ] **Self-Assessment**:
  - [ ] Rate your understanding of each topic (1-5)
  - [ ] List topics that need more work
  - [ ] Plan extra practice for weak areas

**Time**: 2 hours

---

## Week 5-6: Smart Pointers & Concurrency (Days 31-42)

### Week 5 - Day 35 Review
- [ ] **Review**: Smart Pointers Comparison
  - [ ] Box<T>: When and why?
  - [ ] Rc<T> vs Arc<T>
  - [ ] RefCell<T>: Interior mutability
  - [ ] When to use each?
- [ ] **Revisit Week 3**: Collections (30 min)
- [ ] **Practice**: Create a reference-counting example

**Time**: 1.5 hours

### Week 6 - Day 42 Review
- [ ] **Review**: Concurrency Concepts
  - [ ] Threads and message passing
  - [ ] Shared state concurrency
  - [ ] Send and Sync traits
- [ ] **Review**: Thread Safety
  - [ ] Why is Rust's concurrency safe?
  - [ ] Compare channels vs mutexes
- [ ] **Revisit Week 4**: Generics & Traits (30 min)
- [ ] **Practice**: Write a concurrent program using both patterns

**Time**: 2 hours

---

## Month 2 Comprehensive Review (Day 60)

### Major Review Session
- [ ] **Core Concepts** (30 minutes)
  - [ ] Ownership (should be second nature now)
  - [ ] Borrowing (explain advanced cases)
  - [ ] Lifetimes (deeper understanding)
- [ ] **Advanced Types** (30 minutes)
  - [ ] Smart pointers (all types)
  - [ ] Trait objects
  - [ ] Advanced trait features
- [ ] **Concurrency** (30 minutes)
  - [ ] Thread safety guarantees
  - [ ] Different concurrency patterns
  - [ ] When to use each approach
- [ ] **Patterns & Best Practices** (30 minutes)
  - [ ] Idiomatic Rust patterns
  - [ ] Common pitfalls and solutions
  - [ ] Performance considerations
- [ ] **Self-Assessment**:
  - [ ] Can you explain Rust concepts to others?
  - [ ] Are you comfortable reading Rust code?
  - [ ] Can you debug compiler errors efficiently?

**Time**: 2 hours

---

## Ongoing Monthly Reviews (Month 3+)

### Month 3 Review
**Date**: _______________

- [ ] **Complete Ownership System Review** (30 min)
  - Review all ownership, borrowing, and lifetime concepts
  - Ensure these are completely internalized
- [ ] **Traits and Generics Deep Dive** (30 min)
  - Advanced trait usage
  - Associated types
  - Trait objects vs generics performance
- [ ] **Concurrency Patterns** (30 min)
  - Review Send and Sync
  - Arc<Mutex<T>> patterns
  - Channel patterns
- [ ] **Macros and Metaprogramming** (30 min)
  - Declarative macros
  - When to use macros
- [ ] **Practice Project**: Build something complex using multiple concepts

**Time**: 2 hours + project time

### Month 4 Review
**Date**: _______________

- [ ] **Everything from Month 3** (condensed: 45 min)
- [ ] **Advanced Features Review** (45 min)
  - Unsafe Rust
  - Advanced type system features
  - FFI concepts
- [ ] **Performance and Optimization** (30 min)
  - Zero-cost abstractions
  - When to optimize
  - Profiling basics
- [ ] **Practice**: Contribute to an open-source Rust project

**Time**: 2 hours + project time

### Month 5 Review
**Date**: _______________

- [ ] **Comprehensive Language Review** (60 min)
  - All core concepts (should be quick now)
  - Focus on areas you use less
- [ ] **Advanced Patterns** (30 min)
  - Design patterns in Rust
  - Common crate patterns
- [ ] **Ecosystem Exploration** (30 min)
  - Review common crates you've used
  - Explore new crates for your domain
- [ ] **Practice**: Start a significant personal project

**Time**: 2 hours + project time

### Month 6 Review
**Date**: _______________

- [ ] **Mastery Check** (2 hours)
  - [ ] Can you implement complex algorithms in Rust?
  - [ ] Are you comfortable with async/await? (if learned)
  - [ ] Can you design APIs following Rust conventions?
  - [ ] Do you understand performance implications of your choices?
  - [ ] Can you mentor others in Rust basics?
- [ ] **Self-Assessment**: Identify remaining gaps
- [ ] **Plan**: Decide on specialization areas (async, embedded, systems, web, etc.)

**Time**: 2 hours

---

## Weekly Quick Reviews (15-20 minutes each)

Starting Week 2, do these **every Sunday**:

### Week 2+ Sunday Review (15-20 min)
- [ ] Date: _______________
  - [ ] Recite ownership rules
  - [ ] Explain one concept you learned this week to yourself
  - [ ] Review one rustlings section
  - [ ] Identify one thing you struggled with

- [ ] Date: _______________
  - [ ] Recite ownership rules
  - [ ] Explain one concept you learned this week to yourself
  - [ ] Review one rustlings section
  - [ ] Identify one thing you struggled with

- [ ] Date: _______________
  - [ ] Recite ownership rules
  - [ ] Explain one concept you learned this week to yourself
  - [ ] Review one rustlings section
  - [ ] Identify one thing you struggled with

_(Continue this pattern - add more dates as you progress)_

---

## Topic-Specific Deep Dives (As Needed)

Use these when you identify weak areas:

### Ownership Deep Dive (1 hour)
- [ ] Date: _______________
  - [ ] Reread Chapter 4 of the book
  - [ ] Draw memory diagrams for 5 different scenarios
  - [ ] Explain ownership to an imaginary person
  - [ ] Complete all move_semantics rustlings again

### Lifetimes Deep Dive (1 hour)
- [ ] Date: _______________
  - [ ] Reread lifetime sections
  - [ ] Write 10 functions with explicit lifetimes
  - [ ] Explain lifetime elision rules
  - [ ] Complete all lifetime rustlings again

### Traits Deep Dive (1 hour)
- [ ] Date: _______________
  - [ ] Reread trait sections
  - [ ] Implement 5 different traits for custom types
  - [ ] Compare static vs dynamic dispatch
  - [ ] Complete all trait rustlings again

### Concurrency Deep Dive (1 hour)
- [ ] Date: _______________
  - [ ] Reread Chapter 16
  - [ ] Implement both channel and mutex examples
  - [ ] Explain Send and Sync to yourself
  - [ ] Complete all thread rustlings again

### Error Handling Deep Dive (1 hour)
- [ ] Date: _______________
  - [ ] Reread Chapter 9
  - [ ] Write 5 functions with proper error handling
  - [ ] Create custom error types
  - [ ] Complete all error rustlings again

---

## Active Recall Questions (Use for Any Review Session)

### Ownership & Borrowing
- [ ] What are the three ownership rules?
- [ ] Why can you have multiple immutable references but only one mutable?
- [ ] What happens to heap data when the owner goes out of scope?
- [ ] What's the difference between moving and copying?
- [ ] When is data copied vs moved?

### References & Lifetimes
- [ ] What does a lifetime annotation mean?
- [ ] Why do we need lifetime annotations?
- [ ] What are the lifetime elision rules?
- [ ] What is the 'static lifetime?
- [ ] How does the borrow checker use lifetimes?

### Types & Traits
- [ ] What's the difference between String and &str?
- [ ] When should you use Option<T>?
- [ ] What makes a trait object-safe?
- [ ] What's the difference between trait bounds and where clauses?
- [ ] How do associated types differ from generic type parameters?

### Collections
- [ ] When should you use Vec vs array?
- [ ] Why can't you index into a String directly?
- [ ] What's the ownership behavior of HashMap?
- [ ] How do you iterate over a collection without consuming it?

### Error Handling
- [ ] When should you use panic! vs Result?
- [ ] How does the ? operator work?
- [ ] What's the difference between unwrap() and expect()?
- [ ] How do you propagate errors up the call stack?

### Smart Pointers
- [ ] When should you use Box<T>?
- [ ] What's the difference between Rc<T> and Arc<T>?
- [ ] When would you use RefCell<T>?
- [ ] What are the dangers of reference cycles?
- [ ] How does Weak<T> prevent memory leaks?

### Concurrency
- [ ] What are Send and Sync?
- [ ] Why is Rust's concurrency memory-safe?
- [ ] When should you use channels vs mutexes?
- [ ] What's the difference between thread safety and memory safety?
- [ ] How does Arc<Mutex<T>> enable shared mutable state?

---

## Progress Tracking

### Understanding Levels
Rate yourself on each topic (update monthly):

**Rating Scale**:
- 1 = Don't understand at all
- 2 = Vaguely familiar, need to look up
- 3 = Understand basics, can use with documentation
- 4 = Comfortable, can use without documentation
- 5 = Deep understanding, can teach others

| Topic | Month 1 | Month 2 | Month 3 | Month 4 | Month 5 | Month 6 |
|-------|---------|---------|---------|---------|---------|---------|
| Ownership | __ | __ | __ | __ | __ | __ |
| Borrowing | __ | __ | __ | __ | __ | __ |
| Lifetimes | __ | __ | __ | __ | __ | __ |
| Structs & Enums | __ | __ | __ | __ | __ | __ |
| Pattern Matching | __ | __ | __ | __ | __ | __ |
| Error Handling | __ | __ | __ | __ | __ | __ |
| Collections | __ | __ | __ | __ | __ | __ |
| Generics | __ | __ | __ | __ | __ | __ |
| Traits | __ | __ | __ | __ | __ | __ |
| Smart Pointers | __ | __ | __ | __ | __ | __ |
| Concurrency | __ | __ | __ | __ | __ | __ |
| Iterators | __ | __ | __ | __ | __ | __ |
| Closures | __ | __ | __ | __ | __ | __ |
| Macros | __ | __ | __ | __ | __ | __ |

### Weak Areas to Focus On
Update this list as you identify areas needing more work:

1. _________________________________ (Date identified: _______)
2. _________________________________ (Date identified: _______)
3. _________________________________ (Date identified: _______)
4. _________________________________ (Date identified: _______)
5. _________________________________ (Date identified: _______)

---

## Tips for Effective Revision

1. **Active Recall > Passive Reading**
   - Try to remember before checking your notes
   - Explain concepts out loud
   - Write code from memory

2. **Spaced Repetition Works**
   - Review just before you're about to forget
   - Increase intervals for concepts you know well
   - Decrease intervals for difficult topics

3. **Test Your Understanding**
   - Can you teach it to someone else?
   - Can you write code without looking at examples?
   - Can you explain why, not just what?

4. **Connect Concepts**
   - How does ownership relate to concurrency?
   - How do traits enable polymorphism?
   - How do lifetimes ensure memory safety?

5. **Use Multiple Methods**
   - Read code
   - Write code
   - Explain concepts
   - Draw diagrams
   - Debug problems

6. **Track Your Progress**
   - Be honest about what you don't understand
   - Celebrate improvements
   - Don't skip review sessions

7. **Customize This Schedule**
   - Spend more time on topics you find difficult
   - Add extra review sessions for weak areas
   - Adjust timing based on your pace

---

## Maintenance Phase (After Month 6)

Once you've completed the initial learning period:

### Monthly Maintenance (30 min)
- [ ] Quick review of core concepts
- [ ] Read Rust blog posts or RFCs
- [ ] Try a new crate or pattern
- [ ] Contribute to open source

### Quarterly Deep Review (2 hours)
- [ ] Review all fundamental concepts
- [ ] Update your knowledge with language changes
- [ ] Explore advanced topics you haven't used
- [ ] Assess areas for deeper study

### Continuous Learning
- [ ] Follow Rust release notes
- [ ] Read "This Week in Rust"
- [ ] Participate in Rust community
- [ ] Teach others what you've learned

---

**Remember**: The goal isn't to memorize everything, but to build intuition. With regular practice and review, Rust concepts that seem foreign now will become second nature. Trust the process, stay consistent, and don't skip review sessions even when you feel confident!
