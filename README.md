# Rust experiments

## Build for WASM

```
cargo build --target wasm32-unknown-unknown --release
```

## Build normally

Uncomment main function.

Comment out `crate-type = ["cdylib"]` in Cargo.toml.

```
cargo build --release
```
