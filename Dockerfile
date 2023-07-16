FROM scratch
COPY ./spin.toml ./spin.toml
COPY ./target/wasm32-wasi/release/rust_is_bad.wasm ./target/wasm32-wasi/release/rust_is_bad.wasm
