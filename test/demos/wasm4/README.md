# WASM-4 sample games

A collection of example "games" written in LO for the [WASM-4](https://wasm4.org) fantasy console.

## Running

Run any of the samples:

```shell
wasmtime --dir=. lo.wasm compile test/demos/wasm4/src/dark-maze.lo | npx wasm4 run -n -
```

## Building cart files

To build the cart file run:

```shell
wasmtime --dir=. lo.wasm compile test/demos/wasm4/src/dark-maze.lo > cart.wasm
```

## Updating the `Dark Maze` build

The playable build linked from the main README is committed as
[extras/assets/dark-maze.html](../../../extras/assets/dark-maze.html). To rebuild it run:

```shell
wasmtime --dir=. lo.wasm compile test/demos/wasm4/src/dark-maze.lo > cart.wasm
npx wasm4 bundle cart.wasm --title "Dark Maze" --html extras/assets/dark-maze.html
rm cart.wasm
```
