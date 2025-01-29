# Kerogueli

A somewhat rougelike game, written in Rust.

## Instructions to build for the web using WASM [WIP]

First of all, make sure you have the `wasm32-unknown-unknown` target installed. If you don't, you can install it by running:

```bash
rustup target add wasm32-unknown-unknown
```

Also, make sure you have `wasm-bindgen` installed. If you don't, you can install it by running:

```bash
cargo install wasm-bindgen-cli
```

1. Compile the program for WASM

```bash
cargo build --release --target wasm32-unknown-unknown
```
