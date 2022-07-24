# LLVM-C Bindings generation

## Extract all symbols from llvm-c.h

```sh
docker run -v $(pwd):/data glebbash/deno-ffigen-c2ffi /data/input/llvm-c.h > input/llvm-c.json
```

This will generate `input/llvm-c.json` file containing all symbols found in
`input/llvm-c.h`.

## Extract exposed symbols from libLLVM.so

```sh
readelf -Ws --dyn-syms /usr/lib/llvm-14/lib/libLLVM.so > input/llvm-c_symbols.txt
```

This will generate `input/llvm-c_symbols.txt` file containing names of all
exposed symbols of `/usr/lib/llvm-14/lib/libLLVM.so`.

## Generate bindings

```sh
deno run -A https://raw.githubusercontent.com/glebbash/deno-ffigen/main/mod.ts \
  input/llvm-c.json \
  input/llvm-c_symbols.txt \
  llvm-c \
  LLVM \
  "https://github.com/llvm/llvm-project/blob/315072/llvm/include/"
```

Using `input/llvm-c.json` and `input/llvm-c_symbols.txt` as inputs. Bindings are
generated for LLVM-C.

Generated files are:

- `llvm-c/mod.ts` - Bindings entry that also dlopens the shared lib
- `llvm-c/types.ts` - Namespace with all type/enum/function definitions
- `llvm-c/symbols.ts` - Exports object with lib definition for `Deno.dlopen`
- `llvm-c/safe-ffi.ts` - Type utils for making pointer typesafe
