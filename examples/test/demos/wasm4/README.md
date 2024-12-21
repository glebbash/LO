# WASM-4 sample games

A collection of example games written in LO for the [WASM-4](https://wasm4.org) fantasy console.

## Running

Run any of the samples:

```shell
wasmtime --dir=. lo.wasm examples/test/demos/wasm4/src/dark-maze.lo | npx wasm4 run -n -
```

## Building cart files

To build the cart file run:

```shell
wasmtime --dir=. lo.wasm examples/test/demos/wasm4/src/dark-maze.lo > cart.wasm
```
