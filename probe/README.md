## Extract exposed function names

```sh
# extract all symbols
readelf -Ws --dyn-syms libLLVM-15git.so > symbols.txt

# find function symbols
deno run -A get-function-names.ts
```

This will create a file called llvmc-functions.txt with names of all exposed
functions.
