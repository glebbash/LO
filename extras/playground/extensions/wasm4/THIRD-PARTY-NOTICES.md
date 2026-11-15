# Third-party notices

The WASM-4 runtime in `vendor/wasm4/` (`dev/` is the developer build with devtools, `slim/` is what the exported HTML contains)
comes from the `wasm4` npm package, https://github.com/aduros/wasm4, licensed under the ISC license
(`vendor/wasm4/LICENSE.txt`). The build adds it unchanged, except for one change in the developer build: it does not register its key listeners on the parent window. The exported HTML has the same structure as the one of `w4 bundle`.
