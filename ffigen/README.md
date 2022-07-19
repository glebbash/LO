# LLVM-C Bindings generation

## Extract all symbols from llvm-c.h

```sh
# clone c2ffi repo
git clone https://github.com/rpav/c2ffi.git

cd c2ffi

# add entrypoint to dockerfile
echo "ENTRYPOINT [\"/c2ffi/build/bin/c2ffi\"]" >> Docker/Test-Build-Archlinux.docker

# build and tag the image
docker build -f Docker/Test-Build-Archlinux.docker . -t c2ffi

# extract info symbols to json
docker run -v $(pwd):/data c2ffi /data/input/llvm-c.h > input/llvm-c.json
```

This will generate `input/llvm-c.json` file containing all symbols found in
`input/llvm-c.h`.

## Extract exposed symbols from libLLVM.so

```sh
# extract exposed symbols from libLLVM.so
readelf -Ws --dyn-syms input/libLLVM-15git.so > input/llvm-c-exposed.txt
```

This will generate `input/llvm-c-exposed.txt` file containing names of all
exposed symbols of `input/libLLVM-15git.so`.

## Generate bindings

```sh
deno run -A ffigen/mod.ts \
  input/llvm-c.json \
  input/llvm-c-exposed.txt \
  llvm-c \
  LLVM \
  "https://github.com/llvm/llvm-project/blob/315072/llvm/include/"
```

Using `input/llvm-c.json` and `input/llvm-c-exposed.txt` as inputs. Bindings are
generated for LLVM-C.

Generated files are:

- `llvm-c/mod.ts` - Bindings entry that also dlopens the shared lib
- `llvm-c/types.ts` - Namespace with all type/enum/function definitions
- `llvm-c/symbols.ts` - Exports object with lib definition for `Deno.dlopen`
- `llvm-c/safe-ffi.ts` - Type utils for making pointer typesafe
