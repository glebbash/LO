# lole-lisp

Low level LISP-like programming language using LLVM backend

## Docs

Docs are available in the
**[wiki](https://github.com/glebbash/lole-lisp/wiki)**.

## Dev setup

Before developing install latest version of [Deno](https://deno.land/).

You will also need:

- [LLVM](#LLVM-installation-(Ubuntu-only))

### Running compiler

```bash
deno run src/main.ts --allow-all --unstable \
  --src="examples/hello-world.lole" \ # source file
  --out-ir="output.ll" \              # LLVM IR output file
  --out="output" \                    # Binary output file
  -r                                  # to run instead of compiling
```

### LLVM installation (Ubuntu instructions only)

```bash
#install llvm by installation script
wget https://apt.llvm.org/llvm.sh
sudo chmod +x llvm.sh
sudo ./llvm.sh 14
```
