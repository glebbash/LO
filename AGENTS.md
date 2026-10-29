# Working on the LO self-hosted compiler (notes for agents)

Context: `src/*.rs` is the reference Rust compiler. `examples/self-hosted/*.lo` is the port (entry `0-cli.lo`).
The port advances one sample at a time. A sample is done when the self-hosted output is **byte-identical** to the Rust output.

## Workflow

- Full suite: `./utils.ts b test`. Run it often; it takes a while, so read only the summary: `... | grep -E "^ℹ (pass|fail)|✖"`.
- Compile with the self-hosted compiler: `./utils.ts b sh compile <file> | wasm2wat -`
- Compile with Rust for comparison: `./utils.ts b compile <file> | wasm2wat -`
- Compare the two as `.wat`. Use `wasm2wat --no-check` when the self-hosted output is invalid wasm, so you can still read it.
- Add the new sample to the "compiler matches initial" list in `utils.ts` (~line 1011).
- Self-hosted compiler errors come back as `file:line:col - message`. Compile errors in the `.lo` source of the compiler are reported the same way, via the thrown error in `utils.ts`.
- `./utils.ts b inspect <file>` gives LSP-like JSON for a file.
- Format with `./utils.ts format <file>` (prints to stdout). Run it on `.lo` files you changed; the self-hosted printer is already part of the tests.
- Language overview: `docs/language-overview.md`.

## Porting rules (the user cares about these)

1. **Port, don't invent.** Open the Rust function and translate it. Keep names, structure and order.
2. **Minimal per sample.** Port only the branches the current sample exercises. Mark the rest with `// TODO: port` and `todo!()`.
3. **Never end a function with a silent `return` instead of `todo!()`.** A quiet fallback hides bugs. One example: `get_expr_info` returning 0 made an untyped expression silently emit `i32.add`.
4. **Error messages go in `self.registry.error_arena`**: `String::from_str_in(arena, ...)` / `String::new_in(arena, cap)` and `arena.new!(Error {...})`. Do not use plain `String::from_str` for errors.
5. **Wasm instr payloads** are allocated in the module arena: `self.wasm_module.arena.new!(WasmInstrXyz { ... })`.
6. **Comments are ported verbatim, in the same places.**
    - Copy every comment from the Rust source next to the code it describes, unchanged. Do not reword, shorten or move them.
    - If a comment sits on code you did not port, keep it with a `// TODO: port` at that spot.
    - Do not write new explanatory comments yourself. If you think one is needed, list it in your end-of-session report and let the user decide.
    - `// TODO: port` markers are exempt. Keep them minimal: a bare `// TODO: port` where a block from the Rust version is missing, without describing it.
7. Don't lecture the user about tests or ethics. Be concise. Ask (`ask_user`) on real design forks.

## Writing `.lo` samples

A sample in `examples/test/` is also a showcase of the feature it covers, not just an arithmetic check. Favor realistic, concrete types (`Point2D`, `Message`) over generic ones (`Holder`, `Foo`). Comment each block with _what the feature means_, not just what number it adds. Don't repeat the same access path (local var / fn return / struct field) multiple times for one feature unless each repetition shows something genuinely different.

Define types at the top of the file, above `main`, unless a type is a one-off bound tightly to a single function (e.g. an error type used only by one parser) — keep those next to the function it belongs to.

## LO gotchas that cost real time

- **`v.get!(i)` copies; `v.at!(i)` returns a reference.**
    - Writing through a `get!` result silently does nothing. `Registry::add_local` had this bug.
    - Use `at!` when you mutate or take `&field`.
- **`for x in v.iter!()` binds a copy.** `&x.field` then dangles. Use `for &x in v.iter!()` to iterate by reference.
- **`at!` returns a pointer; `as u32` on it casts the address.** Do not use `v.at!(i)` where you want the value (e.g. `locals_ids.at!(i)` as an index). Use `get!` for primitive items.
- **`Abort code N`:** look up N in `examples/lib/std.lo` (`AbortCode`). 4 = `VEC_OUT_OF_BOUNDS`. The stack trace shows function names only, not call sites, so bisect with temporary prints (`stderr_writeln("A")`). Remove them afterwards.
- **No `as_str` on `str`.** `Registry::fmt` is still a stub that returns `"<?>"`, so printing a type gives `<?>`.
- **`Type::equals` is partial** (Never, Void, Bool, U32 only). Extend it when you compare a new type; otherwise comparisons return false silently.
- **Enums:** match with `if match Variant(x) = e { ... }`. For the negative form use `match Variant(x) = e else { ... }`.
- **Line continuation:** a newline ends an expression, so an expression (infix op, `.method()` chain, `|>`) continues on the next line only if there is a `\` before the break. This applies to any operator, not only `||`/`&&`. Inside `(...)`/call args/`{...}` literals trailing commas already continue. `\\` (double backslash) separates items on one line (`a, \\ b`, `let x = 1 \\ let y = 2`) and is only layout for the printer. See `examples/test/backslashes.lo`, `double-backslashes.lo`, `docs/exploration/backslashes.lo`.
- **Literals have no inference:** untagged int literals are always `u32`; `+1`/`-1` are `i32`; anything else needs a tag suffix (`1u64`, `5u8`). Hex literals (`0x3F`) are `u32`. Char literals (`'a'`) are `u8`. Don't cast literals, tag them.
- **String literals default to `seg(u8)`**; `@str_literal_type<T>()` (see `examples/lib/std.lo:5`, `@str_literal_type<str>()`) overrides the default to a named struct type for files that include it, purely so LSP/inspection shows that named type instead of raw `seg(u8)`. The override type only needs the same layout shape as `Seg` (`ptr` + `len`), not the same field names — `std.lo`'s `str` happens to name them `ptr`/`len` too, but that's convention, not a requirement.
- **Mismatched operand types are errors** (`u32` vs `&u32`). The compiler complaining about the compiler source is the normal way you find this.
- **Expr info:** every expression the typer visits must `store_expr_info` / `store_type`. If the typer skips a block (e.g. an `else`), codegen reads index 0 and produces plausible-looking but wrong code.
- **`Registry::get_expr_type`** returns different things by kind: `Ident` → `value_info.type_id`, `InfixOp` → `binary_op_info.result_type_id`, otherwise the raw expr_info.

## Debugging recipe

1. Self-hosted compile; if it traps, note the top frames.
2. If the output is produced but wrong: diff `.wat` of Rust vs self-hosted. The first differing instruction points at a typer-info or codegen branch.
3. Find the corresponding Rust code (`grep -n "CodeExpr::Xyz" src/typer.rs src/codegen.rs`) and compare branch by branch.
4. Run the full suite before finishing.
