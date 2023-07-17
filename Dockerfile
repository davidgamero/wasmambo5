FROM scratch
COPY ./spin.toml ./spin.toml
RUN chown -R node ./
COPY ./target/wasm32-wasi/release/wasmambo5.wasm ./target/wasm32-wasi/release/wasmambo5.wasm
