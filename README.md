# Rust Embedder App Example
Blog post

See `src/main.rs` for the example implementation.

The `wasm-sample-app` directory contains an example rust wasm app to run in the embedder app.

## Requirements
- Rust target `wasm32-unknown-unknown` - install using `rustup target add wasm32-unknown-unknown`

## Running
`cd wasm-sample-app && cargo build --release && cd ..`
`cargo run`