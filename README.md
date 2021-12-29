# lole-lisp
Low level LISP-like programming language using LLVM backend

## Setup

Before developing, you need to set up the correct version of NodeJS, which you can find in `.nvmrc`.

You can achieve this via [nvm](#install-nvm) by running:

```bash
nvm install
```

Setup dependencies with

```bash
npm install
```

You will also need:
 - [LLVM](#LLVM-installation-(Ubuntu-only))
 - CMake 3.17+

## Running compiler

```bash
ts-node src/main.ts \
  --src="examples/hello-world.lole" \ # source file
  --out-ir="output.ll" \              # LLVM IR output file
  --out="output" \                    # Binary output file
  -r                                  # to interpret instead of compiling
```

### LLVM installation (Ubuntu only)

```bash
#install llvm by installation script
wget https://apt.llvm.org/llvm.sh
sudo chmod +x llvm.sh
sudo ./llvm.sh 13

# install cmake by apt-get
sudo apt-get install cmake
```

Note: CMake 3.17+ is required. You can follow [this](https://askubuntu.com/questions/355565/how-do-i-install-the-latest-version-of-cmake-from-the-command-line) to update.

Note installation may fail with
```
Target "llvm-bindings" links to target "ZLIB::ZLIB" but the target was not found.
```
To fix this you need to install zlib:

```bash
sudo apt-get install libz-dev
```

### Install nvm

- on [Linux/Mac](https://github.com/nvm-sh/nvm#install--update-script)
- on [Windows](https://github.com/coreybutler/nvm-windows)
