# LLVM-C Bindings generation

## Extract symbols from LLVM-C headers

```sh
# clone c2ffi repo
git clone https://github.com/rpav/c2ffi.git

cd c2ffi

# add entrypoint to dockerfile
echo "ENTRYPOINT [\"/c2ffi/build/bin/c2ffi\"]" >> Docker/Test-Build-Archlinux.docker

# build and tag the image
docker build -f Docker/Test-Build-Archlinux.docker . -t c2ffi

# extract info symbols to json
docker run -v $(pwd):/data c2ffi /data/llvm-c.h > llvm-c.json
```

This will create `llvm-c.json` file containing all symbols found in LLVM-C
headers.

## Extract exposed function names

```sh
# extract exposed symbols from libLLVM.so
readelf -Ws --dyn-syms libLLVM-15git.so > symbols.txt

# find function symbols
deno run -A get-function-names.ts
```

This will create a file called `llvm-c-functions.ts` which exports array of
names of all exposed functions.

## Generate bindings

```sh
deno run -A ffigen.ts
```

Using `llvm-c-functions.ts` and `llvm-c.json` as inputs. Bindings are generated
for LLVM-C.

Generated files are:

- `enums.ts` - Enum definitions
- `functions.ts` - Function definitions
- `structs.ts` - Struct definitions including alignment info
- `types.ts` - Types definitions
