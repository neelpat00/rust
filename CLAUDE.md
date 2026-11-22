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
- Ch 2 Guessing game (stdin input, parse, expect): https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
- Ch 3.2 Data types (tuples, integer vs float): https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
- rand crate docs (random_bool, random_range, random; taught 2026-11-22 with a weather/die example): https://docs.rs/rand/0.10.4/rand/
- Ch 14.1 Release profiles: https://doc.rust-lang.org/book/ch14-01-release-profiles.html
- statrs StudentsT (t-distribution CDF): https://docs.rs/statrs/0.19.1/statrs/distribution/struct.StudentsT.html
- std::time::Instant (timing code): https://doc.rust-lang.org/std/time/struct.Instant.html
- Ch 10.3 Lifetimes (preview, only the `'static` part for now): https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
- Ch 4.1 Ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- Ch 4.2 References and borrowing: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- Ch 4.3 Slices: https://doc.rust-lang.org/book/ch04-03-slices.html

## Practice projects (given 2026-11-14; review the learner's code when they paste it)
1. `temp_converter`: functions, expressions, `if` as an expression. DONE 2026-11-15 (learner did F→C instead of C→F, which is fine). Extended with stdin input (`read_line(&mut)`, trim → parse into a shadowed f64, `.expect`). Predicted the `hello` → parse panic correctly. LATER UPGRADE: after the Result lesson, loop and re-ask on bad input instead of panicking.
2. `word_tools`: `&str` params, slices, borrow errors on purpose. DONE 2026-11-15 (pending: paste the clear() E0502 run). Lesson given: the `&str` param idiom (accepts &String and literals), range shorthand (`..i`, `i+1..`, `..`), returning a slice borrowed from the input (links to the 'static lesson), and edge cases (no spaces).
3. `ownership_lab`: move vs `&` vs `&mut` in function params. DONE 2026-11-15. Built one feature at a time; correctly predicted the full output (ferris!!!! trap) and when the unused_mut warning appears/disappears. Identified clone vs changing to & for the "print after shout" puzzle; was taught the option table (borrow / clone / reorder / give it back).
4. `fizzbuzz`: `if`/`else if` expression returning `String`, a `for` loop. DONE 2026-11-15: got the condition order right (n % 15 first); used `"Fizz".to_string()`; worried that returning a local String would dangle (taught: returning the owner = a move out, which is fine, unlike returning a view into a local); thought `n.to_string()` was "a reference to the function" (taught: `()` calls it and returns a new String).

5. `gamblers_ruin` (learner's idea, 2026-11-21): Monte Carlo simulation. New: `while`, `cargo add rand` (crates and dependencies → cargo & tooling), `rand::random_bool(p)` (verified to work with rand 0.10.4), `as f64` casts, `+=`. Staged: (1) play_once → bool, (2) estimate over many trials → f64, (3) stretch: compare with the formula (p=0.5: ruin = 1 - k/N; otherwise r=q/p, P(win)=(1-r^k)/(1-r^N), `.powi`). Also does the cold return-type check (bool/f64 vs String).
   - **Handoff from `/Users/neel/projects/learning` (2026-11-22; details in that CLAUDE.md):**
     - Already done in Python: loop sim, vectorised numpy sim, t-tests of sim vs theory. The maths is known (derived the biased formula and E[τ] by hand), so stage 3 is only translation; spend the effort on the Rust.
     - Port the plain one-game-at-a-time loop (no masks). It also has to report τ (bets per game), so a game yields two values: tuples are new here, teach first. Don't hand over signatures (cold return-type check).
     - Targets at start 5, goal 10, p=0.6: P(ruin)=0.116364, E[τ]=19.1818. Validate statistically (seeds won't match Python; a single rejection at α=0.05 can be chance).
     - Race at 100k games: Python loop 0.421s, numpy 0.039s. Get a bet on order and factor BEFORE timing, and don't mention build profiles first (the debug-build trap is the lesson).
     - Learner's preferences: one question per message (two at most); don't make them repeat mechanical checks for concepts already shown; they write the code.

## Layout
- `borrow_practice/`: cargo project used as a scratchpad for borrowing and slice exercises (`src/main.rs` is overwritten freely).
- New topics can get their own `cargo new <topic>_practice` project.

## Course plan
- **Branch 1, core language (current):** cargo and tooling · variables and shadowing · expressions and control flow · ownership and moves · borrowing · slices · structs and enums · match and Option · Result and `?`
- **Branch 2, later:** traits and generics · lifetimes · collections and iterators · closures · modules · concurrency

## Skill map (levels 0-5; last updated 2026-11-22)
```
Topic: Rust — Branch 1: Core language     overall: █████░░░░░ 49%
-----------------------------------------------------------------
cargo & tooling           [██████░░░░]  3/5  reads compiler help: lines and fixes the first error first (temp_converter); didn't paste final output
variables & shadowing     [████████░░]  4/5  explained mut-unused cold; knows shadowed box lives to scope end (hello 5 demo)
expressions & control     [████████░░]  4/5  wrote the if-expression unaided in temp_converter; reasoned the f64 boundaries (10.0 → nice, 25.0 → hot)
ownership & moves         [████████░░]  4/5  ownership_lab: predicted the E0382 move error, ^^^ on the later use, and the unused_mut warning, all before running
borrowing & references    [████████░░]  4/5  restored 2026-11-15: unaided push_str-while-slice-alive → rule A + fix ✓ (omitted the error code and ^^^ line). Earlier dipped to 3 over the clear()/rule B mix-up
slices & String vs &str   [██████░░░░]  3/5  LOWERED 2026-11-15: wrote `-> str` again in fizzbuzz and was unsure about view vs owner. Rule taught: all results are literals → &'static str; any result built at runtime → String. word_tools itself was solid. Re-test cold: give a function and ask which return type
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
- temp_converter (2026-11-15): wrote `-> str`. Taught: `str` is unsized and the return type needs a known size, so use `&str` (ptr+len). Then picked the compiler's `x: &f64` suggestion, which needed `*x`. Taught: deref, operators are defined only for specific type pairs (`&i32 + i32` works, `&f64 < f64` doesn't), and numbers are Copy so take them by value. Taught lifetimes / `'static` = "view valid for the whole program" (literals live in program data), compared with `const`. First guessed `String`, then `'static` with a nudge.
- Said `< 10` means `<= 9`, which is integer thinking about f64. Corrected; reasoned 77°F → 25.0 → "hot" correctly.
- Printing: `{x:.1}` format spec (like Python's `:.1f`). f64 printing drops a trailing `.0`, but the value is still f64.
- Removed `let` and thought the error was "temp moved into temp_in_c, so it's out of scope", and called the previous `let temp_in_c` shadowing. Taught: (1) `let` is required to create a binding; `x = ...` without an existing x gives E0425, a Python habit; (2) shadowing needs the SAME name re-declared; (3) f64 is Copy, and a function's return value is a new value, not the argument.
- Copy vs Move lesson (2026-11-15): first said "anything on the stack is copied". Refined to "does dropping it need to free anything?" (`i32`/`&str` → Copy; `String`/`Vec` → Move). Thought `&str` is Copy "because it lives the whole runtime"; corrected with a `&s[0..2]` example (views free nothing, and many readers are fine). Fixed calc_len with `&String`, and explained clone's cost (extra heap allocation and copy).
- Input lesson: asked what `String::new()`'s box looks like (ptr/len 0/cap 0, no heap yet) and how trim's view becomes an f64 (parse produces a NEW value, and the view is temporary). Built their own model of shadowing as "indexed boxes, newest wins, freed at }". Refined: resolved at compile time, dropped in reverse order. Said `i64` when they meant `f64`.
- RECURRING: thinks `clear()` drops/frees/ends the scope of a String, and so picks rule B. Taught: clear sets len=0 and keeps the buffer and capacity; text is still in scope; the conflict is &mut (clear) vs a live & (rule A, E0502). Compared with C's free → dangling pointer. Re-test cold with a different mutating method (e.g. push_str or truncate).
- word_tools: returned `&s` where s: &str (that's &&str, which deref coercion hides). Taught the reference-to-reference diagram; return `s`. Needed help to explain `&text` → &String → coerced to &str.
- Still often skips pasting the final output. Keep asking for it.
- Learns best from ASCII memory diagrams plus Python analogies (e.g. list object vs internal array). Gets tripped up by syntax, so decode it explicitly.

## Next step
- **NOW (learner's choice, 2026-11-22): project 5 `gamblers_ruin`**, see its handoff note. Structs & enums follows it.
  - 2026-11-22 checkpoint: the learner wrote the one-game function unaided (while, mutable copy, counter, tuple return, bool→i32 cast); it compiles and runs. Open, ONE at a time: (1) asked: at p=0.8, P(`balance += 1`) in one round of their loop (the cold direction review owed from the learning course; don't give the answer). Attempt 1 wrong, so the cold review failed; with the number line got 0.2 and named the fix (flip) themselves. Not cold, so re-test later with another threshold or direction. Test-parameter blind spot: first said 10M sims at p=0.5 would catch it (assumed a non-zero bias); after one nudge saw that buggy and fixed are identical there, so test at p≠0.5. Then said both tests catch it at 0.6/5/10 and that ruin is underestimated; declined to re-derive, so told directly: ruin is OVERestimated, and τ is blind at start = goal/2 (p↔q mirror). Validate at start 3 as well (P(ruin)=0.283880, E[τ]=20.8061). Tutor's scratch run: with only the inequality flipped, both configs pass both tests at 100k. Saved file still had `p_sim > p` then. NOW: the learner builds main. Progress: wrote `mean(&Vec<f64>)` (needed turbofish placement, did the `len() as f64` cast themselves) and a main that unpacks the tuple into two vecs with casts (unaided). Left: flip the inequality, p=0.6/start 3, print mean τ, report ruin (prob is currently the win rate), drop the per-game println, paste output. Race comes after that (bet first, per the handoff).
    - PORT DONE + VALIDATED 2026-11-22: the learner fixed all five items. Tutor ran copies of the full program: all tests pass at n=1000 (start 3) and n=100k (starts 3 and 5), largest |t|=1.90. Asked for the race bet (order + factor at 100k, start 5); nothing timed yet. BET LOCKED: "a few ms" (read as under 10 ms, so Rust first, ≥4x numpy, ≥30x the loop). `Instant` lesson given; the learner does the Rust timing. Python loop re-timed by the tutor as a function: 0.311s without the unused path, 0.338s with (median of 7; use 0.311 for the race). numpy 0.039s is from the notes, not re-timed. RACE ROUND 1 (learner's run, dev build, 100k, start 3): 283.33ms, so the bet was lost by ~30x and they found it unexpected. Build-profile lesson given after that (dev vs release, overflow checks off in release). RACE RESULT (learner's release run, one paste, 100k, start 5): **20.90ms** (bet 100ms): ~15x the Python loop (0.311s), ~1.9x numpy (0.039s from notes), ~12x its own dev build after adjusting for start. Ruin 0.11798 (t=1.60), τ 19.16576 (t≈-0.36), both pass. 10M case: got 160MB ✓ and, UNPROMPTED, proposed pre-allocating to avoid reallocations (real transfer from the heap diagrams). Doubling count: first said 10M/4 (thought it adds 4), then log2(10M) after a nudge (dropped the starting 4; given log2(10M/4) → 22, and the overshoot to 2^24 slots each, ~268MB). with_capacity bet was 50%; learner measured 845.31 vs 847.82ms.
      - Tutor's benchmarks (scratch copies, 11 runs each, 2026-11-22): **no speed saving** (830.3 vs 830.4ms at 10M; paired median 0.01%). Peak resident memory 170 vs 162MB, so the tutor's "268MB / saves 108MB" was wrong in practice (reserved address space; untouched pages aren't backed).
      - **RACE RESULT CORRECTED (supersedes 20.90ms):** short runs from an idle machine read ~17-21ms (warm-up; cause likely CPU ramp, not proven). Back to back, release is **8.35ms at 100k** (~83ns/game, linear to 10M): ~37x the Python loop, ~4.7x numpy (39ms from notes, not re-timed warm), ~30x dev (248ms warm). So the learner's first "few ms" bet was RIGHT for a release build.
      - Running totals DONE 2026-11-22, written by the learner unaided: two f64 accumulators (exact for integers up to 2^53, so no overflow issue). Tutor's 6 release runs of a copy at 10M: ~790ms (~5% faster), peak 1.2MB (was 170MB); ruin and τ match theory.
      - Self t-test: answered "sum of τ²" ✓ straight away. Asked for a p-value like `stats.t.cdf`: given statrs 0.19.1 (`--no-default-features --features std`; `use statrs::distribution::{ContinuousCDF, StudentsT}`; tutor checked it against scipy). First sight of "a trait must be in scope to use its methods" (Branch 2 preview only). First version written 2026-11-22 (theory fns, p-value fn and `&str` verdicts all unaided and right; the `&str` choice for all-literal results is one unaided data point towards slices 4, still needs the cold return-type check).
      - Self-test REVIEW: both denominators are wrong, take them one at a time, don't give the answers. Evidence from the tutor's copies: 2000 clean runs → ruin rejected 8, τ 0 (expect ~100 each); τ theory shifted by +5 still passes. τ: ✓ the learner said SD of one game but testing the mean, then gave sd/√n (a repeat of the dropped-√n slip from the stats course, so review cold there). τ FIX DONE and verified on a copy (5000 clean runs of 10k games: 5.1% reject; theory shifted +0.5: 93% reject). Ruin denominator: answered A (one bet) ✓, then "simulated prob" ✓ (valid plug-in; the tutor's copies reject ~5% with it, as with prob_theor, the exact H0 variance). BOTH TESTS DONE + VERIFIED 2026-11-22 on copies (5000 runs of 10k games, starts 5 and 3): correct simulator rejects 5.0-5.3% for both statistics; a simulator off by 0.01 in p is rejected 100% (ruin) and 78-100% (τ). SIG_FIG → ALPHA renamed by the learner. NOW, one per message: (1) `:.4` on the verdict DONE + verified; (2) cast before squaring DONE + verified on 64k-bet games (dev build, no overflow); (3a) commented-out code deleted, builds with no warnings; (3b) labels say SE, DONE, code tidy-up finished; (4a) root .gitignore added by the learner; (4b) committed by the tutor at the learner's request on branch `claude/practice-projects` (check whether it has been merged into main and pushed; both are the learner's call); (5) cold dev-vs-release check; (6) cold return-type check, then structs & enums. Keep to one labelled question (learner got lost when several points came at once; skill updated). Told directly: `{:.4}` truncates a &str to 4 chars; `(n * n) as f64` squares in i32 before the cast; SIG_FIG means significant figures. After both are fixed, wrap up project 5: tidy the commented-out code, commit (after .gitignore), cold dev-vs-release check, and on to structs & enums (a struct for the running stats is a natural bridge). Later cold re-check: dev vs release, for cargo & tooling → 4. Root .gitignore still missing at this point. The flip is theirs to type, not yet seen. Also: they spotted the integer-mean problem unprompted (told the cast can go at the division, no need to change the return type); (2) justify the type of each returned slot, not done yet; (3) use the returned tuple in main, and no per-game println for big runs. No statistical validation reported yet.
  - Tooling correction 2026-11-22: projects created inside the repo got no .gitignore, so their target/ folders aren't ignored (the tutor wrongly said they were covered, and suggested `git add -A`). Learner was given the root .gitignore fix; confirm it's in before any commit.
  - 2026-11-22: tuple lesson given (neutral `(char, f64)` example). PENDING, cold: the learner writes the signature line for one game (ruined? + τ) and justifies each returned type. The cargo project doesn't exist yet. The "(1) play_once → bool" staging is superseded by the two-value return.
1. DONE: first_word + clear (E0502), scope drop (E0597), the v.len() hidden borrow. Shadowing taught 2026-11-14. Shadowing reached 4. Expressions vs statements lesson given 2026-11-14. temp_converter, ownership_lab and word_tools done. All 4 practice projects DONE 2026-11-15. Open with a cold return-type check (&'static str vs String) to restore slices to 4. Then lesson on structs & enums (teach first). Enums were previewed via `enum Answer { Text(String), Number(i32) }` as Rust's way to hold "one of several types"; build on that, then match/Option.
2. Borrowing reached 4. For level 5: an edge case (e.g. reborrowing, or a borrow passed into a function) plus having them explain the rule as if teaching it.
3. Spaced review later: cold "why is `let x = x + 1` allowed on an immutable x?". If they answer clearly, raise shadowing to 4.
