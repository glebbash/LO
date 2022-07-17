# lole-lisp

Low level LISP-like programming language with LLVM and WASM(binaryen) backends

## Docs

Docs are available in the
**[wiki](https://github.com/glebbash/lole-lisp/wiki)**.

## Dev setup

Before developing install latest version of [Deno](https://deno.land/).

You will also need:

- [LLVM](#LLVM-installation-(Ubuntu-only))

### Running compiler (LLVM backend)

```bash
deno run src/main.ts --allow-all --unstable \
  --src="examples/hello-world.lole" \ # source file
  --out-ir="output.ll" \              # LLVM IR output file
  --out="output" \                    # Binary output file
  -r                                  # to run instead of compiling
```

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

### LLVM installation (Ubuntu instructions only)

```bash
#install llvm by installation script
wget https://apt.llvm.org/llvm.sh
sudo chmod +x llvm.sh
sudo ./llvm.sh 14
```
