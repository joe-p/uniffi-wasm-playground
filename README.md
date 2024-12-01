# UniFFI Playground

This is a repo that is being used to play around with UniFFI. It is based on the [UniFFI Arithmetic Example](https://github.com/mozilla/uniffi-rs/tree/main/examples/arithmetic). So far everything has been tested on MacOS.

For more information about UniFFI, see the [official documentation](https://mozilla.github.io/uniffi-rs/) and the [README](https://github.com/mozilla/uniffi-rs/blob/main/README.md).

**TL;DR:** Write one Rust codebase, generate bindings for the following languages:

- Python
- Ruby
- Kotlin
- Swift
- React Native\*
- Kotlin Multiplatform\*
- Dart\*
- Go\*
- C#\*

\* = Third-party bindings

[wasm-pack](https://github.com/rustwasm/wasm-pack) could also be used in conjunction with UniFFI to target JS (node and web) as well.

## Key Files

- [src/lib.rs](src/lib.rs): The Rust library that contains the core logic.
- [src/arithmetic.udl](src/arithmetic.udl): The UDL file that describes the FFI bindings API.
- [consumers/python/arithmetic.py](consumers/python/arithmetic.py): The python binding generated by `uniffi-bindgen`.
- [consumers/python/app.py](consumers/python/app.py): A python script that uses the FFI bindings to call into the Rust library.
- [consumers/web/pkg/arithmetical.js](consumers/web/pkg/arithmetical.js): The bindings for the WASM generated by `wasm-pack`.
- [consumers/web/index.html](consumer/web/index.html): A simple webpage that uses WASM compiled from the Rust library.

## What's been tested

| Feature                 | Python | Web |
| ----------------------- | ------ | --- |
| Basic FFI bindings      | ✅     | ✅  |
| Async HTTP FFI bindings | ✅     | ✅  |
| Error handling          | ✅     | ✅  |

## Building

Because of https://github.com/rust-lang/cargo/issues/1197 we need to modify the `surf` features dynamically in `Cargo.toml` depending on if we're targeting wasm or not. This means instead of using `cargo build` directly we have a Python script `build.py` that does this and then calls either `cargo build` or `wasm-pack build` respectively.

To build the Python bindings: `python build.py py`

To build the web bindings: `python build.py wasm`

## Running

To run the Python bindings: `python consumers/python/app.py`

To run the web bindings: `cd consumers/web && python -m http.server`

## TODO

- [ ] Test on Windows
- [ ] Test on Linux
- [ ] Test other languages supported by UniFFI
- [ ] Further exploration of [wasm2js](https://github.com/WebAssembly/binaryen/blob/main/src/wasm2js.h) to potentially enable native JS output (vs needing to use wasm), which could be useful for maximal browser compatibility
