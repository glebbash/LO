<h1>LO Docs</h1>

> AI Use Disclosure: This doc was mostly AI generated. Its claims were verified on test programs, it was human reviewed and corrected. But it is a dense collection of facts, not a guided introduction to the language.

<br/>

If you'd just rather learn from examples:

- [test](../test/) - small test/showcase programs for all of the language features
- [lib](../lib/) - standard library
- [src](../src/) - the compiler itself

<br/>

Table of contents:

- [👀 Overview](#-overview)
- [📝 Syntax](#-syntax)
- [🧱 Types](#-types)
- [😶 Expressions](#-expressions)
- [🧩 Functions](#-functions)
- [🪄 Constants and macros](#-constants-and-macros)
- [💾 Memory](#-memory)
- [📦 Modules](#-modules)
- [🧰 Intrinsics](#-intrinsics)
- [📚 Standard library](#-standard-library)
- [🔨 Compiler usage](#-compiler-usage)
- [🧪 Compiler development](#-compiler-development)
- [🐛 Known issues](#-known-issues)

## 👀 Overview

LO is a small, statically typed, low level language that compiles to WebAssembly. A program is a set of `.lo` files compiled into a single WASM module, with no runtime and no hidden allocations.

```lo
@include("../lib/cli.lo")

fn main() {
    println("Hello World!")
}
```

The same program without the standard library shows what the language itself provides:

```lo
@use_memory(min_pages = 1, data_start = 12, exported)

fn fd_write(
    fd: u32,
    iovs: &void,
    iovs_len: u32,
    nwritten: &u32,
): u32 = @import_from("wasi_snapshot_preview1")

inline let message = "Hello World!\n"

export fn _start() {
    let nwritten = 0 as &u32

    let iovs = 4 as &seg(u8)
    *iovs = message

    let _ = fd_write(1, iovs, 1, nwritten)
}
```

Things that work differently from most languages:

- There are no semicolons. A line break ends an expression unless the line ends with `\`, see [Line breaks](#line-breaks).
- A literal's type never depends on context: `0` is a `u32` and `0u8` is a `u8`, see [Literals](#literals).
- `::` is part of a name, not a namespace operator: `std::mem::align_to` is one identifier, see [Names](#names).
- There's no shadow stack. Variables live in WASM locals and have no address, so `&` only takes addresses of memory locations, see [Memory](#-memory).
- Constants and macros are `inline` variables and functions, see [Constants and macros](#-constants-and-macros).
- `return` inside a macro returns from the function that called it, see [Macros](#macros).
- A `Result` is a tuple of a value and an error, and an error of `0` means success, see [Error handling](#error-handling).

## 📝 Syntax

### Comments

```lo
// line comments are the only kind
fn main() {} // they can follow code
```

The formatter moves comments found in unusual places, see [Comment rearrangement](#comment-rearrangement).

### Names

Identifiers use `a-z`, `A-Z`, `0-9` and `_` and can't start with a digit. A name can have several parts joined with `::`:

```lo
fn std::mem::align_to(size: u32, multiple: u32): u32 { ... }
type heap::Block = struct { ... }
inline let wasi::FD_STDOUT = 1
```

- `::` is part of the name. There are no modules or namespaces, but `::` matters for [methods](#methods) and [include aliases](#-modules).
- There can be no spaces around `::`.
- Keywords: `fn` `export` `inline` `let` `type` `struct` `enum` `if` `else` `match` `loop` `while` `for` `in` `break` `continue` `return` `defer` `do` `with` `sizeof` `as` `catch` `true` `false` `null`. `Ok` and `Err` build [results](#error-handling), and `it` is bound by [pipes](#pipes) and [do with](#do-with).

### Line breaks

There are no semicolons. An expression ends at a line break when the next line starts with a binary operator, so a `\` at the end of the line is needed to continue it:

```lo
let total = price \
    + tax \
    - discount

let escaped = String::from_str(text) \
    .replace("\n", "\\n") \
    .as_str()
```

- This applies to every binary operator, including `.`, `as`, `catch`, `?` and `|>`, and also inside parentheses.
- A line that starts with `-`, `+`, `*`, `&` or `!` without a `\` before it is a new expression with a prefix operator.
- An operator left at the end of a line also continues the expression, but the formatter joins the two lines. `\` is the way to keep the break.
- Line breaks between the items of `()`, `[]` and `{}` lists need no `\`. Trailing commas are allowed in every list and work as a [formatting hint](#formatting-hints).
- `return` takes a value only if the value starts on the same line. A bare `return` must end its line: `{ return }` is a parse error.
- Several statements can share a line: `let x = 1 let y = 2` is valid, though the formatter splits it unless you use `\\`.

### Formatting hints

Two pieces of syntax tell the [formatter](#formatting) how to lay out code:

- A trailing comma after the last item of a list puts every item on its own line. Without one, all items go on one line.
- `\\` keeps several list items or statements on one line. The compiler ignores it.

```lo
fn sum6(
    a1: u32, \\ a2: u32, \\ a3: u32,
    a4: u32, \\ a5: u32, \\ a6: u32,
): u32 {
    let x = 1 \\ let y = 2
    return a1 + a2 + a3 + a4 + a5 + a6 + x + y
}
```

Here the trailing comma after `a6: u32` makes the parameter list multiline, and `\\` groups the parameters three per line.

### Literals

| Literal   | Examples                                     | Type                                                         |
| --------- | -------------------------------------------- | ------------------------------------------------------------ |
| Integer   | `42`, `1_000_000`                            | `u32`                                                        |
| Hex       | `0x3F`, `0xFF_FF`                            | `u32`                                                        |
| Suffixed  | `255u8`, `5000000000u64`, `0xFFu16`          | the suffix type                                              |
| Character | `'a'`, `'\n'`                                | `u8`                                                         |
| Boolean   | `true`, `false`                              | `bool`                                                       |
| Null      | `null`                                       | `null`, converts to any `&?T`                                |
| String    | `"Hello\n"`                                  | `seg(u8)` or the [string literal type](#string-literal-type) |
| Array     | `[u16][1000u16, 2000u16]`                    | `seg(T)`                                                     |
| Struct    | `Point { x: 1, y: 2 }`                       | the struct                                                   |
| Result    | `Ok(value)`, `Err(error)`, `Ok:<u32, u8>(1)` | `Result(T, E)`                                               |

- A literal's type never comes from context: `let x = 0` is always a `u32`. Pick another type with a suffix (`0u8`) or a cast (`0 as u8`).
- The suffix is a type name written right after the digits, without a space.
- `+1` and `-1` are `i32`s, because unary `+` and `-` make integers signed (see [Operators](#operators)).
- `null` has a type of its own, also called `null`. It converts to any nullable pointer, and its value is the address `0`.
- Hex digits must be uppercase: `0xFF`, not `0xff`.
- There are no float, binary or octal literals.
- Escapes are `\n` `\r` `\t` `\0` `\\`, plus `\"` in strings and `\'` in characters.
- Strings can span several lines. They aren't zero-terminated, so add `\0` for hosts that expect C strings. Their length is in bytes of UTF-8.
- Characters are single bytes, so only ASCII characters fit in a character literal.
- Identical strings are stored once.
- Array items must be constants, because the array is placed in [static data](#static-data). Its `.len` is the number of items.

## 🧱 Types

### Primitive types

| Type         | Bytes | WASM value |
| ------------ | ----- | ---------- |
| `bool`       | 1     | `i32`      |
| `u8`, `i8`   | 1     | `i32`      |
| `u16`, `i16` | 2     | `i32`      |
| `u32`, `i32` | 4     | `i32`      |
| `u64`, `i64` | 8     | `i64`      |
| `void`       | 0     | none       |
| `never`      | 0     | none       |

- `void` means no value.
- `never` is the type of `return`, `break`, `continue`, `@unreachable()` and calls to functions returning `never`. It fits any type, so such expressions can stand wherever a value is expected.

### Pointers

| Type     | Meaning                                                        |
| -------- | -------------------------------------------------------------- |
| `&T`     | pointer to a `T`                                               |
| `&?T`    | nullable pointer, accepts `null`                               |
| `&[]T`   | pointer to the first of several `T`s                           |
| `&?[]T`  | nullable `&[]T`                                                |
| `&void`  | untyped pointer, any non-nullable pointer converts to it       |
| `&?void` | untyped nullable pointer, any pointer or `null` converts to it |

Pointers are 32-bit addresses into linear memory. `0` is a valid address, and also the value of `null`.

```lo
let p = 64 as &Point      // integers cast to pointers
*p = Point { x: 1, y: 2 } // write through a pointer
let x = p.x               // field access dereferences one level automatically
let y_ref = &p.y          // &u32
*y_ref += 1
```

- `&` only takes addresses of memory locations: `&p.field`, `&p.a.b`, `&(*p)`. Variables, parameters and globals have no address, because LO has no shadow stack, see [Memory](#-memory). To get a pointer to a value, copy it into memory first, for example with std's `ref!(value)`.
- `&[]T` is never dereferenced automatically and has no indexing operator. Compute addresses yourself or use std's `array_at!<T>(ptr, index)`.
- Pointer arithmetic goes through integers: `(p as u32 + 4) as &T`.
- `p == null` works for nullable pointers only.

### Structs

```lo
type Point = struct {
    x: u32,
    y: u32,
}

let p = Point { x: 1, y: 2 }
p.x = 10
```

- A struct literal lists every field, in declaration order.
- Structs are values: assigning, passing and returning copies them. In WASM a struct is a group of values, one per primitive field, so functions take and return structs without touching memory. This needs the WASM multi-value extension, see [Compiling](#compiling).
- Fields of any struct value can be read (`get_point().x`), but only fields of variables and pointers can be assigned.
- In `if`, `while` and `for` headers, struct literals need parentheses: `if (Point { x: 0, y: 0 }).x == 1 { ... }`.
- Types can be used before they are defined. A struct can contain itself through a pointer, not by value.
- A struct without fields, or with only `void` fields, has size 0. Reading and writing it compiles to nothing.

### Enums

```lo
type Color = enum {
    Red,
    Green,
    Blue,
}

type Expr = enum (&void) {
    Binary(&ExprBinary),
    Item(&ExprItem),
}

let color = Color::Green
let expr = Expr::Item(item)
```

- An enum is a `u32` tag, optionally followed by one payload slot whose type is written in parentheses: `enum (&void) { ... }`.
- Each variant's payload must be compatible with the slot type, not identical to it. That's how every variant of `Expr` above carries its own pointer type.
- Variants without a payload are values (`Color::Green`). Variants with one are built like a call with exactly one argument.
- Tags count up from `0`. Plain enums can set them explicitly (`Off = 10, On = 20`), for every variant or none.
- Plain enums, the ones without a payload slot, support `==`, `!=` and casts to and from `u32`. Enums with a payload slot are read with [pattern matching](#pattern-matching).
- `.tag` reads the tag as a `u32`.

### Segments

`seg(T)` is a segment, more commonly known as a slice: a pointer and a length. Slices usually get built-in syntax like `[]T`, but LO spells them as a named type so that [methods](#methods) can be attached to them. Segments are also the type of array literals:

```lo
let bytes = [u8][1u8, 2u8, 3u8] // seg(u8)
let count = bytes.len           // 3, a number of items
let first = *bytes.ptr          // `ptr` is a &[]u8
let head = @seg(bytes.ptr, 2)   // a seg(u8) from parts

for byte in bytes {
    // ...
}
```

### Type aliases

```lo
type Tile = bool
type Trail = Vec(Pos)
type wasi::Err = u32
```

- An alias is the same type as its target: `Tile` and `bool` are interchangeable.
- Methods are found through the target's name: `tile.flip()` calls `bool::flip`, never `Tile::flip`.
- Aliases that refer to each other in a cycle are errors.

### Type arguments

Any named type can be followed by type arguments: `Vec(u32)`, `&Vec(&Vec(Tile))`, `Box(T)`. They don't change the layout, so `Vec(u32)` is laid out like `Vec`, but the type keeps them and `itemof` reads them back. This is how std's untyped `Vec` gets typed items:

```lo
inline fn Vec::get!(&self, index: u32): itemof(typeof(self)) {
    *(self.at(index) as &itemof(typeof(self)))
}

let v = Vec::new!<u32>() // &Vec(u32)
let x = v.get!(0)        // u32
```

A `Vec(T)` converts to a plain `Vec` automatically, so regular functions can take the untagged type. The other direction needs a cast.

Type arguments don't take part in method lookup, so a method defined once as `Vec::name` works for every `Vec(T)`:

```lo
fn Vec::is_empty(&self): bool { // self: &Vec
    return self.len == 0
}

let names = Vec::new!<str>()    // &Vec(str)
let empty = names.is_empty()    // calls Vec::is_empty
```

### Type operators

| Syntax         | Meaning                                                                                  |
| -------------- | ---------------------------------------------------------------------------------------- |
| `seg(T)`       | [segment](#segments)                                                                     |
| `Result(T, E)` | [result](#error-handling)                                                                |
| `sizeof T`     | size of `T` in bytes, a `u32` expression. No parentheses: `sizeof Point`                 |
| `typeof(x)`    | type of the variable or parameter `x`                                                    |
| `itemof(T)`    | item type: `T` for `seg(T)`, `Result(T, E)` and `Name(T, ...)`, looking through pointers |
| `infer(T)`     | in [inline function](#macros) parameters, names the argument's type `T`                  |

### Layout

Layout follows C rules: primitives are aligned to their size, and a struct's size is rounded up to its largest alignment. `struct { a: u8, b: u32 }` takes 8 bytes and `struct { a: u64, b: u8 }` takes 16.

The other compound types are tuples, laid out by the same rules:

| Type           | Tuple                    |
| -------------- | ------------------------ |
| `seg(T)`       | `(ptr: &[]T, len: u32)`  |
| `Result(T, E)` | `(ok: T, err: E)`        |
| `enum (P)`     | `(tag: u32, payload: P)` |
| plain enum     | `(tag: u32)`             |
| `Name(T)`      | same as `Name`           |

## 😶 Expressions

### Variables

```lo
let count = 0     // u32
count += 1
let _ = compute() // run and discard the result
```

- `let` takes its type from the value, and there's no type annotation. Choose the type through the value: `let total = 0u64`, `let next = null as &?Node`.
- Variables and function parameters are mutable.
- `_` discards a value and can be used any number of times. It can't name a parameter.
- A name can't be defined twice in one block, but an inner block can shadow it.
- Statements must be `void`. When `compute` returns a value, `compute()` alone is an error: write `let _ = compute()`.

### Globals

```lo
let HEAP_INDEX = 0
let STATIC_DATA_SIZE = @data_size()
```

- Globals are declared at the top level. They are mutable and must start with a constant value: a literal, a struct, array or result literal, a [constant](#constants), a plain enum variant, `@data_size()`, or an `as` cast of one of these.
- Each primitive field of a global becomes its own WASM global.

### Operators

From tightest to loosest:

| Operators                                           | Meaning                              |
| --------------------------------------------------- | ------------------------------------ |
| `value catch e { ... }`                             | [catch](#error-handling)             |
| `.`                                                 | field access, method call            |
| `?`                                                 | [error propagation](#error-handling) |
| `-x` `+x` `!x` `*p` `&place`                        | prefix operators, see below          |
| `as`                                                | [cast](#casts)                       |
| `*` `/` `%`                                         | multiplication, division, remainder  |
| `+` `-`                                             | addition, subtraction                |
| `<<` `>>`                                           | shifts                               |
| `&`                                                 | bitwise and                          |
| `\|`                                                | bitwise or                           |
| `==` `!=` `<` `<=` `>` `>=`                         | comparison                           |
| `&&`                                                | logical and, short-circuiting        |
| `\|\|`                                              | logical or, short-circuiting         |
| `\|>`                                               | [pipe](#pipes)                       |
| `=` `+=` `-=` `*=` `/=` `%=` `&=` `\|=` `<<=` `>>=` | assignment                           |

| Prefix     | Meaning                                                                                  |
| ---------- | ---------------------------------------------------------------------------------------- |
| `-x`, `+x` | negate, keep. The result is the signed type of the same width: `-x` of a `u8` is an `i8` |
| `!x`       | `true` if `x` is zero. Works on `bool`, integers, pointers and plain enums               |
| `*p`       | dereference                                                                              |
| `&place`   | address of a memory location, see [Pointers](#pointers)                                  |

- Both operands of a binary operator must have the same type. There are no implicit conversions.
- Arithmetic and bitwise operators work on integers. Comparisons also work on `bool`, pointers, `null` and plain enums. Structs, segments, results and enums with a payload slot can't be compared.
- `/`, `%`, `>>` and ordering comparisons are signed for `i8` to `i64` and unsigned for everything else. Division by zero traps.
- `&` and `|` bind tighter than comparisons: `flags & MASK != 0` means `(flags & MASK) != 0`.
- There is no xor (`^`) and no bitwise not (`~`).
- Assignments are `void` statements and can't be chained.

### Casts

`value as Type` converts between:

- Two integer-like types: integers, `bool`, pointers, plain enums and `null`. Converting to a wider type extends the value, with the sign or with zeroes depending on the source type. Converting to a narrower type keeps the low bits.
- Two types with the same WASM shape: as many primitive values, with 32-bit and 64-bit values in the same positions. Examples: `str` and `seg(u8)`, `&Vec` and `&Vec(u8)`, a struct holding one pointer and a pointer.

Other casts are compile errors.

### If condition

```lo
if x < 5 {
    return 0
} else if x < 10 {
    return 1
} else {
    return 2
}
```

- The condition must be a `bool`: write `if count != 0`, not `if count`.
- Braces are required, parentheses aren't.
- `if` is a statement and doesn't produce a value.

### Loops

```lo
loop { ... }                 // until `break` or `return`
while i != 0 { ... }
for i in 0..10 { ... }       // 0, 1, ..., 9
for item in items.iter!() {  // a copy of each item of a seg(T)
    ...
}
for &item in items.iter!() { // a pointer to each item
    item.count += 1
}
```

- `for i in start..end` counts up by one until the counter equals `end`. Both bounds are evaluated once and must have the same integer type, which the counter gets. The counter is a regular variable: assigning to it changes the iteration, and a counter that skips over `end` (or a `start` above `end`) never stops.
- `for x in value` requires a `seg(T)`. `Vec` and `str` provide one with `.iter!()`.
- `break` and `continue` apply to the innermost loop. There are no labels.
- A loop never counts as a way out of a function. In a function that returns a value, a `loop` left only through `return` must be followed by `@unreachable()`.

### Defer

```lo
fn main() {
    let fd = fs::open("data.txt") catch _ {
        wasi::proc_exit(1)
    }
    defer let _ = fs::close(fd)

    // ...
}
```

`defer expr` runs `expr` when the current block ends, latest first.

- In a loop body it runs at the end of every iteration.
- `return` runs all pending defers of the function, after evaluating the returned value.
- A `defer` inside an [inline function](#macros) belongs to the caller's block.
- Any expression can be deferred. `defer if true { ... }` defers a whole block.

### Pattern matching

```lo
if match Expr::Binary(bin) = expr {
    print(bin.operator)
} else if match Expr::Item(item) = expr {
    print(item.name)
}

match Expr::Item(item) = bin.lhs else {
    return
}
println(item.name) // `item` stays in scope
```

- `if match Enum::Variant(x) = value` checks the tag and binds a copy of the payload to `x` inside the block. It chains with `else if match`, `else if` and `else`.
- `match Enum::Variant(x) = value else { ... }` binds `x` for the rest of the block. The `else` block runs on a mismatch and must not finish normally: it has to `return`, `break`, `continue`, call `@unreachable()` or call a function returning `never`.
- There is no multi-arm `match` and no exhaustiveness check.

### Error handling

`Result(T, E)` is a tuple of a value and an error. The error part doubles as the tag: `0` means success, and anything else is a failure.

```lo
type MathError = enum {
    DivisionByZero = 1, // 0 would mean success
}

fn divide(a: u32, b: u32): Result(u32, MathError) {
    if b == 0 {
        return Err(MathError::DivisionByZero)
    }
    return Ok(a / b)
}

fn run(): Result(void, MathError) {
    let half = divide(10, 2) catch err {
        return Err(err)
    }
    let bad = divide(half, 0)? // same as the `catch` above
    return Ok()
}
```

- The error type must fit in one 32-bit value: an integer up to 32 bits, a pointer or a plain enum. Errors must be non-zero, since `Err(0)` reads as success: use codes from `1`, enum tags from `1` as above, or non-null pointers.
- `Ok(x)` and `Err(e)` take their type from the function's return type. Elsewhere, write it out: `Ok:<u32, u8>(x)`. `Ok()` creates a `Result(void, E)`, and `Err(e)` fills the value with zeroes.
- `value catch e { ... }` evaluates to the value. On error it binds `e` and runs the block, which must not finish normally.
- `value?` returns the error from the current function after running pending defers. The function must return a `Result`, and the error is passed on unchanged.
- `.ok` and `.err` read the two parts without any check: `if res.err as u32 != 0 { ... }`.
- `catch` binds tighter than any operator, so `a + f() catch e { ... }` only catches `f()`.

### Pipes

> [!NOTE]
> Pipes are experimental. They're fully supported, but they aren't a focus of the language and aren't considered idiomatic yet. They exist mostly because they were small to implement.

```lo
let result = 2 \
    |> it + 2 \
    |> mul(it, it) \
    |> it.sub(2) // 14
```

`a |> b` evaluates `a` once, names it `it` and evaluates `b`. Steps can mix function calls, method calls and operators.

### Do with

```lo
do print(it) with (
    "{ \"index\": ", \\ index.to_tmp_str!(), \\ " }",
    "\n",
)
```

`do expr with (a, b, c)` repeats `expr` for every argument, with `it` bound to it. It is unrolled at compile time and covers what string formatting does elsewhere. All arguments must have the same type, and there must be at least one.

## 🧩 Functions

```lo
fn add(a: u32, b: u32): u32 {
    return a + b
}

fn log(message: str) { // no return type means `void`
    println(message)
}
```

- Functions can be used before they're defined.
- A function with a return type must not reach its end ("Control reaches end of non-void function"). Statements after a `return` get an "Unreachable expression" warning.
- Struct and segment arguments and results are passed as several WASM values.
- Every function is compiled into the output, whether it's called or not.

### Methods

A function named `Type::name` is a method of `Type`: `value.name(args)` calls `Type::name(value, args)`, where `Type` is the name of the value's type with pointers looked through.

```lo
fn Point::len_squared(self): u32 { // self: Point
    return self.x * self.x + self.y * self.y
}

fn Point::move_by(&self, dx: u32) { // self: &Point
    self.x += dx
}

fn u32::squared(self): u32 { // any type can have methods
    return self * self
}

let d = p.len_squared() // same as Point::len_squared(p)
let n = 5.squared()
ptr.move_by(1)          // ptr: &Point
Point::move_by(ptr, 1)  // the same call, by full name
```

- The first parameter receives the value, and its type can be left out: `self` gets `Type` and `&self` gets `&Type`. The name `self` isn't required, but it's the convention, and the VS Code extension highlights it.
- There's no automatic referencing or dereferencing: a `self` method needs a value and a `&self` method needs a pointer.
- Any method can also be called by its full name with the receiver passed explicitly, as in Rust and Lua. Functions that don't take the type first, like `Point::new(1, 2)`, are only called that way.

Methods of `seg(T)` and `Result(T, E)` are named `seg::name` and `Result::name`. A regular function can only accept one item type, so generic methods are written as [inline functions](#macros), which can name the receiver's type with `infer`:

```lo
fn seg::sum(self: seg(u8)): u32 { ... }                            // seg(u8) only
inline fn seg::skip!(self: infer(Self), amount: u32): Self { ... } // any seg(T)
```

### Nested functions

```lo
export fn main(): u32 {
    return factorial(5)

    fn factorial(x: u32): u32 {
        if x < 2 {
            return x
        }
        return x * factorial(x - 1)
    }
}
```

- A nested function is visible in its whole block, before and after its definition.
- Nested functions aren't closures: using the outer function's variables is an error. Globals, constants and other functions are fine.
- In WASM debug info a nested function is named `outer/inner`.

### Exports and imports

```lo
export fn add(a: u32, b: u32): u32 {
    return a + b
}

fn wasi::proc_exit(code: u32): never = @import_from("wasi_snapshot_preview1")

@export_existing(Vec::with_capacity, out = "vec_new")
```

- `export fn` exports a function under its full name, `::` parts included.
- `= @import_from("module")` declares a host function. The imported name is the last part of the LO name, so `wasi::proc_exit` imports `proc_exit`.
- Every declared import ends up in the WASM module, even when it's unused.
- `@export_existing(name)` exports an existing function, and `out = "..."` renames the export.
- `from_root` defers the export to the entry file, the one passed to the compiler: the name is looked up there instead of in the file with the directive. That's how `lib/cli.lo` exports your `main` as the WASI entry point without seeing it: `@export_existing(main, from_root, out = "_start")`.

## 🪄 Constants and macros

Constants and macros reuse the syntax of variables and functions: `inline let` declares a constant and `inline fn` declares a macro. There's no separate syntax for constants, macros or generics. The aim is the uniformity of Jai and Odin, where constants and procedures share one declaration form, with keywords that stay familiar and easy to type.

### Constants

```lo
inline let MAX_PLAYERS = 4
inline let Color::BLACK = Color { r: 0, g: 0, b: 0 }
inline let NAMES = [str]["Ada", "Grace"]
```

- A constant has no storage. Its expression is inserted wherever it's used and evaluated each time, so any expression works and `inline let X = f()` calls `f` at every use.
- Constants are declared at the top level.

### Macros

Inline functions are expanded at every call site, which makes them both macros and generics. Their name ends with `!` in definitions and calls:

```lo
inline fn double!(value: infer(T)): T {
    value + value
}

inline fn make_array!<T>(count: u32): &[]T {
    heap::alloc(count * sizeof T) as &[]T
}

let a = double!(21)           // u32
let b = double!(-3i64)        // i64
let buf = make_array!<u8>(64) // &[]u8
```

- The body is copied into every call site and type-checked there, with the actual types. An inline function that is never called is never checked.
- The last expression of the body is the result. Callers see the declared return type, which isn't checked against the body. That's how `Vec::new!<T>(): &Vec(T)` turns an untyped `&Vec` into a typed one.
- Type parameters (`<T, U>`) are passed explicitly: `make_array!<u8>(64)`. A parameter of type `infer(T)` names its argument's type `T`. In the body, `typeof` and `itemof` derive more types.
- Inline methods work like methods: `inline fn Vec::push!(&self, item: itemof(typeof(self)))` is called as `items.push!(x)`.
- The `!` is part of the name, so `Vec::at!` and `Vec::at` are different functions. std often pairs a typed macro with an untyped function this way.
- Parameters are read-only. The number of arguments and type arguments must match exactly.
- `return` inside an inline function returns from the calling function, and `defer` runs at the end of the caller's block. That's how std's `tmp_alloc!<T>()` frees its memory when the caller's block ends.
- `@inline_fn_call_loc()` returns where the inline function was called from, like `"src/main.lo:4:5"`, for tracing and assertions.

## 💾 Memory

LO works with WASM linear memory directly and never allocates memory behind your back. There's no shadow stack, the stack in linear memory where compilers for languages like C keep variables whose address is taken. Variables, parameters and globals live only in WASM locals and globals, which have no address. Anything stack-like, such as a buffer freed at the end of a block, is user code, like std's [stack allocator](#allocation).

`@use_memory` sets up the memory:

```lo
@use_memory(min_pages = 1, data_start = 12, exported)
```

| Option                | Meaning                                                          |
| --------------------- | ---------------------------------------------------------------- |
| `min_pages = N`       | initial size, in 64 KiB pages                                    |
| `data_start = N`      | address where [static data](#static-data) starts, `0` by default |
| `exported`            | exports the memory as `"memory"`                                 |
| `import_from = "env"` | imports `env.memory` instead of defining a memory                |

- A program can have one `@use_memory`. Its values must be integer literals or constants.
- If a program uses memory without `@use_memory`, it gets a private 1-page memory.
- String and array literals use memory, as do dereferences and field access or assignment through pointers. Taking an address with `&` doesn't.
- `@forbid_memory()` makes any memory use a compile error, for code that must run without one.

### Static data

String literals, array literals and `@embed_file(path)` contents are stored in data segments, one after another from `data_start`. `@data_size()` gives the address right after the last byte. It's allowed only as a global's value, and std uses it to place its stack after the static data.

### String literal type

String literals are `seg(u8)` by default. `@str_literal_type<T>()` changes their type for the whole program to any `T` made of two 32-bit values. std uses a struct with its own methods:

```lo
type str = struct {
    ptr: &[]u8,
    len: u32,
}
@str_literal_type<str>()
```

It can be set once. `@inline_fn_call_loc()` returns the same type.

## 📦 Modules

```lo
@include("../lib/cli.lo")
@include("./math.lo", alias = "math")

fn main() {
    let x = math::factorial(5)
}
```

- `@include` makes the top-level definitions of another file visible in the including file. Includes are resolved before type checking, so their position in the file doesn't matter.
- Paths starting with `.` are relative to the including file. Other paths are used as they are, relative to the compiler's working directory.
- A file is included once, however many times and through whichever relative paths it's referenced, so there's no need for include guards like in C.
- `with_extern` re-exports an include: files that include this one see its definitions too. `lib/cli.lo` uses it so that including `cli.lo` alone provides `std.lo`, `wasi.lo` and `print.lo`.
- `alias = "name"` prefixes every included definition with `name::`. Method calls don't apply the prefix, so methods of `math::Point` are called as `math::Point::method(p)`.
- Defining a name that an included file already defines is an error. There is no privacy: every top-level definition is visible to includers.

## 🧰 Intrinsics

Top-level directives:

| Directive                                                                      | Purpose                                                                       |
| ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| `@include("path", alias = "name", with_extern)`                                | includes a file, see [Modules](#-modules)                                     |
| `@use_memory(min_pages = N, data_start = N, exported, import_from = "module")` | configures memory, see [Memory](#-memory)                                     |
| `@forbid_memory()`                                                             | disallows memory use                                                          |
| `@export_existing(fn_name, out = "name", from_root)`                           | exports an existing function, see [Exports and imports](#exports-and-imports) |
| `@str_literal_type<T>()`                                                       | sets the [string literal type](#string-literal-type)                          |

Expressions:

| Intrinsic                                      | Type                                 | Purpose                                         |
| ---------------------------------------------- | ------------------------------------ | ----------------------------------------------- |
| `@unreachable()`                               | `never`                              | traps                                           |
| `@memory_size()`                               | `i32`                                | memory size in pages                            |
| `@memory_grow(pages: u32)`                     | `i32`                                | grows memory, returns the previous size or `-1` |
| `@memory_copy(dest: u32, src: u32, size: u32)` | `void`                               | copies `size` bytes                             |
| `@data_size()`                                 | `u32`                                | end of static data, only as a global's value    |
| `@embed_file("path")`                          | `seg(u8)`                            | bakes in file's contents                        |
| `@embed_files("dir_path")`                     | `seg({name: str, content: seg(u8)})` | bakes in folder's contents                      |
| `@seg(ptr: &[]T, len: u32)`                    | `seg(T)`                             | builds a segment                                |
| `@inline_fn_call_loc()`                        | string literal type                  | location of the current inline function call    |

## 📚 Standard library

The standard library lives in [lib](../lib/). WASI programs usually include just `cli.lo`.

| File            | Provides                                                                                                                      |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `cli.lo`        | `std.lo`, `wasi.lo` and `print.lo`, 40 pages of exported memory, and your `main` exported as `_start`                         |
| `std.lo`        | `str`, allocators, `Vec`, `String`, number formatting and helpers                                                             |
| `print.lo`      | `print`, `println`, `eprint`, `eprintln`, `fprint(fd, s)`                                                                     |
| `fs.lo`         | `fs::read_file!`, `fs::read_file`, `fs::open`, `fs::read_fd`, `fs::pipe`, `fs::close`, `fs::write_new_file`, `fs::create_dir` |
| `args.lo`       | `Args::load()`, `args.argc`, `args.get(i)`, `args.free()`                                                                     |
| `wasi.lo`       | WASI preview 1 imports and types                                                                                              |
| `int_parser.lo` | `parse_u32`, `parse_u32_hex`                                                                                                  |
| `str_cutter.lo` | `StrCutter`, which splits a `str` piece by piece                                                                              |
| `debug.lo`      | `debug_u32::log`, host logging provided by `utils.js`                                                                         |
| `wasm4.lo`      | [WASM-4](https://wasm4.org) bindings, see the [demos](../test/demos/wasm4/)                                                   |

`fs.lo` expects the host to preopen the working directory as file descriptor 3, which `wasmtime --dir=.` does.

### Allocation

std splits memory into static data, then a stack, then a heap:

- **Stack**: a bump allocator after the static data, 1 MiB by default. `stack::SIZE` changes that if set before the first allocation. `tmp_alloc!<T>()`, `tmp_alloc_many!<T>(count)` and `ref!(value)` allocate on it and free at the end of the caller's block.
- **Heap**: a first-fit allocator after the stack that grows memory as needed. `heap::new!(value)`, `heap::alloc!<T>()`, `heap::alloc_many!<T>(count)`, `heap::alloc(size)`, `heap::free(ptr)`.
- **Arenas**: `Arena::new(block_size)`, then `arena.new!(value)`, `arena.alloc!<T>()`, `arena.reset()`, `arena.free()`.
- **`Alloc`** stands for any of them: `Alloc::STACK`, `Alloc::HEAP` or an `&Arena`. `new!(a, value)`, `alloc(a, size)` and `free(a, ptr, size)` dispatch on it, and constructors have `_in` versions such as `Vec::with_capacity_in!<T>(a, n)`, `String::new_in(a, n)` and `n.to_str_in!(a)`.
- `abort(code)` stops the program. It writes `69420` at address `0` and the `AbortCode` at address `4`, then traps.

### Strings and collections

- **`str`**: `len`, `ptr`, `iter!()`, `char_at(i)`, `equals(s)`, `starts_with(s)`, `contains(s)`, `find(s, start)`, `slice(start, end)`, `skip_chars(n)`, `split(sep, out)`, `clone()`, `str::from_cstr(ptr)`.
- **`String`**: `String::new(capacity)`, `String::from_str(s)`, `push(c)`, `push_str(s)`, `len()`, `char_at(i)`, `as_str()`, `replace(from, to)`, `String::join(items, sep)`, `clone()`, `free()`.
- **`&Vec(T)`**: `Vec::new!<T>()`, `Vec::with_capacity!<T>(n)`, `push!(x)`, `get!(i)` (a copy), `at!(i)` (a pointer), `set!(i, x)`, `iter!()`, `push_all!(seg)`, `append!(other)`, `pop()`, `swap(i, j)`, `reverse()`, `resize!(len, x)`, `free()`, `free_deep!()`. Out-of-bounds indexes abort.
- **`seg(T)`**: `skip!(n)`, `take!(n)`. For `&[]T` there's `array_at!<T>(ptr, index)`.
- **`Result(T, E)`**: `unwrap!()`.
- **Numbers**: `n.to_tmp_str!()` for `u32` and `u64`, valid until the end of the block, `n.write_to(string)`, `u32::max!(a, b)`, `u32::MAX`.

## 🔨 Compiler usage

The compiler is a single WASM binary, `lo.wasm`. It uses [WASI](https://wasi.dev/) preview 1 and runs in any WASI runtime, for example [wasmtime](https://wasmtime.dev/):

```bash
wasmtime --dir=. lo.wasm
```

```text
Usage:
  lo compile <input.lo> // compiles source file to a WASM binary
  lo inspect <input.lo> // emits JSON inspections
  lo format  <input.lo> // prints formatted version of the source
  lo flatten <input.lo> // prints the source with all includes inlined
```

- Passing `-i` instead of a file name reads the source from stdin. It shows up as `<stdin>` in messages.
- The compiler can only read files in preopened directories, hence `--dir=.`.

> The examples below use `lo` as a shortcut. On Linux and macOS you can define it with `alias lo='wasmtime --dir=. path/to/lo.wasm'`.

### Error reporting

Errors and warnings go to stderr, one per line:

```text
ERROR: <file>:<line>:<col> - <message>
WARNING: <file>:<line>:<col> - <message>
```

The compiler keeps going after an error to report as many as it can. It exits with code 1 if there were any.

### Compiling

```bash
lo compile main.lo > main.wasm
```

Writes the WASM module to stdout. Included files are found automatically. The module has a name section with function and local names, where struct variables show up as `name#field`, so `wasm2wat` output stays readable.

The output is a WebAssembly 1.0 module plus two features standardized in WebAssembly 2.0:

- **Multi-value** is required. Structs, segments and results span several WASM values, and every `catch` and `?` uses a multi-value block.
- **Bulk memory** is used by `@memory_copy`, which std uses to grow vectors and copy strings.

### Inspecting

```bash
lo inspect main.lo
```

Prints a JSON array that describes the program, for editor integrations. The [VS Code extension](../vscode-ext/src/extension.ts) uses it for hovers, go to definition and diagnostics.

> [!NOTE]
> The format isn't stable. JSON is simply the easiest to consume for now, and a leaner format, possibly binary, is planned.

```json
[
    { "type": "file", "index": 1, "path": "main.lo" },
    { "type": "info", "hover": "let x: u32", "loc": "1/3:9-3:10" },
    {
        "type": "info",
        "link": "1/3:9-3:10",
        "hover": "let x: u32",
        "loc": "1/4:12-4:13"
    },
    {
        "type": "message",
        "content": "Unreachable expression",
        "severity": "warning",
        "loc": "1/5:5-5:14"
    },
    { "type": "end" }
]
```

- `loc` and `link` are `<file index>/<line>:<col>-<line>:<col>`, using the indexes of the `file` items. `link` points to a definition.
- Errors and warnings become `message` items instead of stderr output. The exit code is 1 if there were errors.
- The final `end` item only keeps the array valid JSON (because inspections print a comma at the end of the line).

### Formatting

```bash
lo format main.lo
```

Prints the formatted source of the specified file (does not format included files).

The style isn't configurable:

- 4-space indentation, one statement per line unless joined with `\\`.
- Lists of parameters, arguments, array items and struct literal fields are multiline when they end with a trailing comma and single-line otherwise. Struct and enum definitions are always multiline.
- `\` line breaks are kept, and continued lines are indented one level deeper.
- At most one blank line is kept between items.

#### Comment rearrangement

Comments in unusual places move to the nearest regular place. This keeps the formatter simple and comment placement consistent.

```lo
// this is a comment
fn main():
// this will be moved
u32 { // this will also be moved
    // this is fine
    return 0 // this is also fine
}
```

Is formatted as:

```lo
// this is a comment
fn main(): u32 {
    // this will be moved
    // this will also be moved
    // this is fine
    return 0 // this is also fine
}
```

### Flattening

```bash
lo flatten main.lo
```

Prints every file of the program, formatted, each under a `//// file: <path>` header and with `@include` lines commented out. Definition order doesn't matter, so the result works as one self-contained source, which is handy for sharing a reproduction.

## 🧪 Compiler development

| Path                | Contents                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------- |
| `src/`              | the compiler, written in LO, with `src/0-cli.lo` as the entry point                         |
| `lib/`              | the standard library                                                                        |
| `test/`             | test programs, with diagnostics checks in `compile-errors/` and larger programs in `demos/` |
| `initial-compiler/` | the bootstrap compiler, written in Rust                                                     |
| `lo.wasm`           | the compiler binary, built from `src/`                                                      |
| `utils.js`          | build, test and run helper for Node.js 24+ or Deno                                          |
| `vscode-ext/`       | the VS Code extension                                                                       |

### Building

The compiler builds itself: the current `lo.wasm` compiles `src/0-cli.lo` into a new `lo.wasm`.

```bash
./utils.js build      # rebuild lo.wasm
./utils.js build test # rebuild, then run the tests
```

### Bootstrap compiler

`initial-compiler/` holds the original Rust implementation. The test suite checks that both compilers produce the same binaries and diagnostics.

- Install [rustup](https://www.rust-lang.org/tools/install). The nightly toolchain and the `wasm32-unknown-unknown` target are pinned in `initial-compiler/rust-toolchain.toml`. There's also a DevContainer setup in `initial-compiler/.devcontainer`.
- Run `cd initial-compiler && ./build.sh`, which writes `initial-compiler/lo.wasm`.

### Running tests

```bash
./utils.js test            # all tests
./utils.js test --fast     # skip the comparison with the bootstrap compiler
./utils.js test --skip-fmt # skip the formatter tests
```

- Tests are defined in `utils.js`. They compile the programs in `test/`, run them, and check results, output and diagnostics.
- Formatter tests format every `.lo` file outside `docs/` and compare the result to the file. They also write the result back, so `git diff` shows what changed.

### Other commands

```bash
./utils.js compile main.lo > main.wasm # any lo.wasm command
./utils.js run main.lo -- arg1 arg2    # compile and run a program
./utils.js runWasi main.wasm arg1      # run a compiled WASI program
./utils.js debugWasi main.wasm         # serve it at http://localhost:6969 for browser DevTools
```

## 🐛 Known issues

### Bugs

- **Prefix operators take too much.** `*p + 1` parses as `*(p + 1)`, `*p as u32` as `*(p as u32)` and `!a + b` as `!(a + b)`. Wrap the operand for now: `(*p) + 1`.
- **Small integers aren't truncated.** Integers narrower than 32 bits live in 32-bit WASM values, so `250u8 + 10u8` is `260` and `300 as u8` stays `300`. Only storing to memory keeps the low bits.
- **Inline function arguments are substituted.** An argument used twice is evaluated twice, and an unused one is never evaluated. Copy arguments into variables when that matters.
- **Includes are always re-exported.** Every include behaves as if it had `with_extern`. The flag itself only matters for include cycles.
- **Payload enums ignore explicit tags.** Explicit tags are accepted on enums with a payload slot, but payload variants are built with their position as the tag, so `match` doesn't recognize them.
- **`inline let` in a function body** is accepted, but the constant is never set and reads as zero. It should be a compile error.
- **Globals can't start negative.** `let G = -1` is rejected, while `+1` works. After a rejected global value, the following top-level definitions are skipped, which leads to misleading errors about them.
- **Flattening drops include aliases**, so `lo flatten` output doesn't compile for programs that use them.
- **Some code crashes the compiler** instead of producing an error:
    - `?` in a function that doesn't return a `Result`.
    - Array items other than integer literals, optionally cast with `as`, and string literals. Negative numbers, `bool`s and constants all crash.
    - `@inline_fn_call_loc()` outside an inline function, right after the error is reported.

### Quirks

- **Nullable and array pointers convert implicitly**: `&?T` to `&T` and back, and `&[]T` to `&T`. Only `&T` to `&[]T` needs `as`.
- **Untyped parameters after the first one** also get the method's type: in `fn str::equals(self, other): bool`, `other` is a `str`.
- **`null == p` is not supported.** Only `p == null` type checks.

### Planned removals

- **`f32` and `f64`** exist as type names only to cover every WASM type. They have no support, and values of these types crash code generation.
