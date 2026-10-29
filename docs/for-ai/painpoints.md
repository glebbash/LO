# LO painpoints and feature ideas

> **AI-generated.** Sections are labeled by author. Treat AI sections as feedback from using the language, not as design decisions.

## Claude Sonnet 5.5 (Copilot harness, session of 2026-10-28)

Context: porting `short-circuit.lo` and `add.lo` to the self-hosted compiler. Ordered by time lost.

1. **`get!` vs `at!` silently differ.**
    - `v.get!(i).field = x` copies and does nothing (caused the `add_local` bug).
    - `for x in v.iter!()` binds a copy, so `&x.field` dangles; `for &x in` is correct but not the default.
    - Ideas: error on assigning to a field of an rvalue copy, error on `&` of a by-value loop binding's field, or a more explicit name than `get!` (e.g. `copy!`).
2. **`&T as u32` compiles.**
    - `locals_ids.at!(i) as u32` cast the address and produced an out-of-bounds abort far from the cause.
    - Idea: require an explicit form for pointer-to-int casts, or reject this one.
3. **Aborts have no call site.**
    - `Abort code 4` plus function names only; I bisected with `stderr_writeln` prints.
    - Ideas: carry a source location in `abort`, or print the abort code name (`VEC_OUT_OF_BOUNDS`) in the shim.
4. **Verbose enum matching.** Repeated `if match Variant(x) = e { ... }` makes `codegen`/`type_code_expr` long. A `match` with arms would help.
5. **No derived equality.** `Type::equals` lists variants by hand, so a missing case silently returns `false`. Derived structural equality would remove this class of bug.
6. **Tooling.**
    - A single `compile --diff-against-rust` command would replace the manual `.wat` diff loop.
    - `inspect` could list reachable `todo!()`s per sample, to show port progress.
    - `Registry::fmt` is stubbed to `"<?>"`; porting it early makes type errors debuggable.
7. **Error-message coverage.** ~115 error sites in `typer.rs` vs ~4 error samples. A corpus of mutated samples, compared Rust vs LO, would find missing or different messages cheaply.
