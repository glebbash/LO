# LO language overview (what it is, roughly)

LO is a small, low-level, statically typed language that compiles straight to WebAssembly (WASI for I/O). If you know C, Zig or Go, most of it will feel familiar. The surface looks like Rust/TypeScript. There is no borrow checker, no GC, and almost no implicit behavior.

For the full reference see [README.md](./README.md). For real code, see `examples/test/*.lo` (small focused samples), `examples/lib/*.lo` (std) and `examples/self-hosted/*.lo` (the compiler written in LO).

## Ideology

- **As simple as practically possible.** Both the language and the compiler should stay small enough to understand fully. Every feature has to earn its place, and many decisions were weighed against "does this make the compiler bigger?" (see the goals in the [README](../README.md)).
- **WASM only.** Targeting a single, small, well-specified target is one of the biggest simplicity wins: no backends, no ABI zoo, no linker. The output is close to the source, so `wasm2wat` is the "debugger" and the provenance tool.
- **Few dependencies.** Ideally just the compiler (`lo.wasm`) and the source. The end goal is a self-hosted compiler.
- **Explicit over implicit.** Manual memory, explicit allocators, no inference for literals, no hidden control flow.
- **Practical, not minimal for its own sake.** Features that make real programs (including the compiler itself) pleasant are in, as long as they stay cheap to implement. Many were driven by writing the self-hosted compiler.
- **Document the process.** Design alternatives are kept in [exploration/](./exploration) (for example `apply-for-all.lo` and `expr-piping.lo` list options with pros and cons), and the journey is in the DevLog.

## Syntax shape

The syntax looks like Rust/TypeScript but is closer to **Jai** in how uniform it is: top-level definitions are all "name = value" shaped, and the set of top-level forms is tiny. Per the top-level AST (`src/ast.rs`, `examples/self-hosted/ast.lo`) the only top-level expressions are:

- `[inline] let NAME = value` for globals and constants
- `[export] [inline] fn name(...) { ... }` for functions (`inline` ones are macros, `name!`)
- `type Name = ...` for structs, enums and aliases
- `@intrinsic(...)` directives (`@include`, `@use_memory`, `@export_existing`, ...)

Everything else lives inside function bodies. There are no modules, namespaces, classes or impl blocks: `Type::method` and `std::mem::align_to` are just function names, and `@include` is textual.

## Features that are unusual

- **`catch` is an expression**, not a statement. `let x = fallible() catch err { ... }` binds the ok value, and the `catch` block must diverge (`return`, `break`, `@unreachable()`...). `?` propagates the error. `Result(T, E)` is stored as two values with the error value `0` meaning "no error".
- **`match` is a pattern _test_.** `if match Enum::Variant(x) = e { ... } else if match ...`, and `match Enum::Variant(x) = e else { ... }` to bind-or-diverge. (Exhaustive matches are on the roadmap.)
- **Line continuation like bash.** A newline ends an expression; a trailing `\` continues it. This is what lets the language be semicolon-free without being whitespace-sensitive:
    ```lo
    return 2 \
        |> it + 2 \
        |> mul(it, it)
    ```
- **Double backslash `\\` packs several items onto one line** (call arguments, params, even statements): `a, \\ b, \\ c` or `let x = 1 \\ let y = 2`. It carries no meaning for the compiler, but the formatter preserves it, so you control how a long list is laid out instead of having it exploded one-per-line or joined into one line.
- **Expression piping** `x |> f(it)`, where `it` is the piped value. Methods and pipes can be mixed.
- **`do expr with (a, b, c)`** is a compile-time unrolled "for each argument" (`it` is the item), used to print or write many pieces without string formatting.
- **Macros look like ordinary functions** (`inline fn name!`), are type-checked, and use `infer(T)`, `typeof`, `itemof` and `<T>` type params instead of a separate generics system.
- **Enums are tagged unions with an explicit payload type**: `enum (u32) {...}`, `enum (&void) {...}`; the parenthesized type is the shared payload representation, and variants carry values that fit it.
- **Everything is inspectable.** The compiler has an `inspect` mode that emits LSP-like info (types, definitions, errors) as JSON; the VS Code extension is built on it.
- **Definition order doesn't matter** (automatic forward declarations).

## Where it's going

Finish the self-hosted port, then exhaustive matches, macro validation, a compiler-managed stack, a better error ABI (`<tag><ok><err>`), and size/debug tooling.

## Closest comparisons

- **C with modern syntax and wasm as the target.** Manual memory, raw pointers, structs by value, `sizeof`. Closest in spirit.
- **Zig** for: allocators passed explicitly (`Alloc`, `Arena`, `heap`), `comptime`-like inline functions, `defer`, no hidden allocations.
- **Rust** for: syntax (`fn`, `let`, `enum`, `match`, `Result`), methods via `fn Type::method(&self)`. There is no ownership or lifetimes.
- **Go** for: simplicity goals, `defer`, `for` loops, no inheritance.
- **WAT/AssemblyScript** for: the output is predictable wasm, close to the source (temporaries and all). Debug names are emitted.

## Feature set

**Types:** `u8 i8 u16 i16 u32 i32 u64 i64 f32 f64 bool void never null`, pointers `&T`, nullable `&?T`, arrays/slices `&[]T`, `seg(T)` (pointer + length), `str`, structs, enums, `Result(Ok, Err)`, aliases `type X = ...`.

**Literals (no inference):** untagged int = `u32`, `+1`/`-1` = `i32`, otherwise tag with a suffix (`1u64`). Hex is `u32`. Chars are `u8`. Strings are `str`.

**Functions:**

- `fn name(args): ret { ... }`, `export fn` for wasm exports, `fn x() = @import_from("module")` for imports.
- Methods are just functions named `Type::name`; `self` / `&self` first param. Namespacing like `std::mem::align_to` is only part of the name.
- Order of definitions doesn't matter (auto forward declarations); only cyclic type aliases are errors.

**Macro-like `inline fn name!`:**

- Inlined at the call site. Used for generic-ish things (`Vec::push!`, `heap::new!`, `Box::new!`).
- `infer(T)` captures a param's type, `itemof(typeof(x))` derives types from it, `<T>` explicit type params.
- Comparable to Zig `inline fn` / `comptime` or C macros that are type-checked.
- `@inline_fn_call_loc()` gives the caller location.

**Control flow:** `if` / `else if` / `else`, `while`, `loop`, `for i in 0..n`, `for x in xs`, `for &x in xs` (by reference), `break`, `continue`, `return`. Short-circuit `&&`/`||`. Compound assignment (`+=`, `*=`, ...).

**Pattern-ish matching:** enums with payloads, `if match Enum::Variant(x) = e { ... } else if match ...`, and `match Enum::Variant(x) = e else { ... }` for the "must be this variant" form. Not a full `match` with arms.

**Errors:** `Result(T, E)` as a two-value return (error value 0 means no error). `catch err { ... }` and the `?` propagation operator.

**Other expressions:**

- `defer` (runs at scope exit; in loops after each iteration).
- `do expr with (a, b, c)` repeats `expr` with `it` bound to each item.
- `x |> f(it)` expression piping.
- `as` casts, `sizeof T`, `*p` deref, `&x` ref.
- Struct literals, array literals `[u32][1, 2, 3]`.

**Top-level & globals:**

- `let X = 0` for globals, `inline let` for constants.
- `@include("path")` / `@use_memory(min_pages = 1)` / `@export_existing(...)` / `@str_literal_type<str>()` compiler directives.
- `@use_memory(...)` declares the module's memory; `@forbid_memory()` turns memory use into a compile error (see `compile-errors/forbid-memory.lo`).

**Syntax quirks:**

- No semicolons. A newline ends an expression. To continue an expression on the next line, put `\` at the end of the line (any operator, `.call()` chains, `|>`).
- `\\` separates several items on one line (list/param/statement layout, kept by the formatter).
- Trailing commas allowed.

**Tooling:**

- `lo.wasm` is the compiler: `compile`, `inspect` (LSP-like JSON), `format`, plus an experimental interpreter.
- A VS Code extension provides highlighting and diagnostics.
- The Rust compiler (`src/`) is the bootstrap; `examples/self-hosted/` is the port in progress (lexer, parser, printer, AST done; typer and codegen partially).

## What it does _not_ have

No classes or inheritance, no traits/interfaces, no closures, no garbage collector, no operator overloading, no exceptions, no exhaustive `match` with arms, no type inference for literals, and no borrow checking. Generics exist only through `inline fn` / `infer(T)` and `Vec(T)`-style parameterized types.
