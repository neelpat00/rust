# Rust learning workspace

This folder is for learning Rust through Socratic tutoring. It is not a product codebase.

## How to resume
- Run `/personal-tutor` (the learner's customised tutor skill in `~/.claude/skills/personal-tutor/`) and say "continue the Rust course". The original `/socratic-tutor` is kept but no longer used.
- **Teach first, then question.** For each NEW concept, give a short lesson (what it is, a Python comparison, an ASCII diagram where it helps) and a Rust Book reading link, THEN ask questions. The learner asked for this on 2026-11-14 because questions on untaught concepts felt like guessing. Use Socratic questions to check and deepen understanding, not to discover brand-new material.
- Keep to Socratic mode: ask one question at a time, don't give answers until 2-3 failed attempts, and level up strictly. When a code check is possible, have the learner run it with `cargo run` and paste the output.
- The learner knows **Python** and is new to Rust syntax. Use Python comparisons where they help, and keep the syntax burden low.
- After each session, **update the skill map and "Next step" below**.

## Reading (The Rust Book, free online)
- Ch 3.1 Variables, mutability, shadowing: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
- Ch 3.3 Functions (statements vs expressions): https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- Ch 3.5 Control flow (`if` as an expression): https://doc.rust-lang.org/book/ch03-05-control-flow.html
- Ch 4.1 Ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- Ch 4.2 References and borrowing: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- Ch 4.3 Slices: https://doc.rust-lang.org/book/ch04-03-slices.html

## Practice projects (given 2026-11-14; review the learner's code when they paste it)
1. `temp_converter`: functions, expressions, `if` as an expression
2. `word_tools`: `&str` params, slices, borrow errors on purpose
3. `ownership_lab`: move vs `&` vs `&mut` in function params
4. `fizzbuzz`: `if`/`else if` expression returning `String`, a `for` loop

## Layout
- `borrow_practice/`: cargo project used as a scratchpad for borrowing and slice exercises (`src/main.rs` is overwritten freely).
- New topics can get their own `cargo new <topic>_practice` project.

## Course plan
- **Branch 1, core language (current):** cargo and tooling · variables and shadowing · expressions and control flow · ownership and moves · borrowing · slices · structs and enums · match and Option · Result and `?`
- **Branch 2, later:** traits and generics · lifetimes · collections and iterators · closures · modules · concurrency

## Skill map (levels 0-5; last updated 2026-11-11)
```
Topic: Rust — Branch 1: Core language     overall: ████░░░░░░ 31%
-----------------------------------------------------------------
cargo & tooling           [████░░░░░░]  2/5  used cargo new/run, read warnings (missed predicting unused_mut warning)
variables & shadowing     [████████░░]  4/5  explained mut-unused cold; knows shadowed box lives to scope end (hello 5 demo)
expressions & control     [██████░░░░]  3/5  taught 2026-11-14; add_one `;` → () ✓, if-branch type mismatch ✓; needed a nudge to link it to the compile-time typed box
ownership & moves         [██████░░░░]  3/5  said "moves it out of scope" (imprecise: s1 is invalidated, not out of scope)
borrowing & references    [████████░░]  4/5  applied liveness cold (r dead after push_str → println s ok)
slices & String vs &str   [██████░░░░]  3/5  knows String=owner/&str=view, :: vs ., let mut vs &mut; rule A (E0502) + rule B (E0597 outlive owner) ✓
structs & enums           [░░░░░░░░░░]  0/5
pattern matching/Option   [░░░░░░░░░░]  0/5
error handling (Result)   [░░░░░░░░░░]  0/5
-----------------------------------------------------------------
```

## Misconceptions seen
- Explains borrow errors through memory (reallocation, dangling pointers) instead of the borrow rule. Steer them to "which references are alive on this line?"
- Thought spare `Vec` capacity could make a borrow conflict compile (the compiler never looks at runtime values).
- Thought two `&mut` lines always fail. Corrected: the rule only applies to references that are *alive at the same time* (non-lexical lifetimes).
- Called `push` a macro (macros end in `!`).
- Mixed up `let mut r = &s` with mutable access. Now knows that `mut` after `let` is about the variable and `mut` after `&` is about the borrow.
- Thought `&mut s` points at the heap and that "the compiler fixes references" after reallocation. Shown a stack/heap diagram: `r` points at `s`, and `push_str` updates `s.ptr` at runtime. `&v[0]` points INTO the heap, which is why it would dangle.
- Kept predicting the error's `^^^` lands on the later `println!`. Taught: it lands on the line that CREATES the borrow it can't allow (method calls hide borrows via &self/&mut self). Got it right on a retest (v.len()).
- Says "out of scope" when they mean "borrow no longer alive" (scope = until `}`; alive = until last use; dropped = owner leaves scope).
- Thought `let mut` sets the variable's type or allows any new value. Taught: the type is fixed for life, and `mut` allows a new value of the SAME type (typed-box picture).
- Tends to run the code before predicting warnings. Ask for the prediction first.
- Thought shadowing frees the old value. Taught: it's hidden, not dropped; drop happens at scope end.
- Python refcount model: "it stays alive because a reference still points at it". Taught: references never keep owners alive; scope decides, and the borrow checker only verifies.
- Thought an inner `{ }` block gets its own stack. Taught: one stack frame per function call; blocks only limit name visibility and when drops happen.
- Said "doesn't return anything" for a function ending in `;`. Taught: it returns `()` (unit, like None).
- Thought Rust if-branches must match because "Python if is a statement". Corrected: Python's ternary is an expression too; the real reason is that Rust fixes each variable's type at compile time.
- Learns best from ASCII memory diagrams plus Python analogies (e.g. list object vs internal array). Gets tripped up by syntax, so decode it explicitly.

## Next step
1. DONE: first_word + clear (E0502), scope drop (E0597), the v.len() hidden borrow. Shadowing taught 2026-11-14. Shadowing reached 4. Expressions vs statements lesson given 2026-11-14. Expressions done (3/5). NEXT: lesson on structs & enums (teach first). Enums were previewed via `enum Answer { Text(String), Number(i32) }` as Rust's way to hold "one of several types"; build on that, then match/Option.
2. Borrowing reached 4. For level 5: an edge case (e.g. reborrowing, or a borrow passed into a function) plus having them explain the rule as if teaching it.
3. Spaced review later: cold "why is `let x = x + 1` allowed on an immutable x?". If they answer clearly, raise shadowing to 4.
