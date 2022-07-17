# lole-lisp

Low level LISP-like programming language compiled to WASM

## Docs

Docs are available in the
**[wiki](https://github.com/glebbash/lole-lisp/wiki)**.

## Dev setup

Before developing install latest version of [Deno](https://deno.land/).

### Running compiler (WASM backend)

```bash
deno run src/main-wasm.ts -A \
  --src="examples/hello-world.lole" \ # source file
  --out="output.wasm" \               # WASM output file
  --wat="output.wat" \                # WAT output file
  -r                                  # to run instead of compiling
```

Or with a shortcut:

```bash
./exec.sh examples/hello-world.lole
```

### Running tests

```bash
deno task test
```
