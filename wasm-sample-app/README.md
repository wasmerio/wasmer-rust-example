# Hello World

## Requirements
- Rust target `wasm32-unknown-unknown` - install using `rustup target add wasm32-unknown-unknown`

## Building
`cargo build --release`


## Pass String from Host to Wasm Example
To see an example of passing a string to web assembly from rust see the `examples/string.rs` file.  This can be run using `cargo run --example string`.