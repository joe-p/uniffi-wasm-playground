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

## What's been tested

| Feature                 | Python | Web |
| ----------------------- | ------ | --- |
| Basic FFI bindings      | ✅     | ✅  |
| Async HTTP FFI bindings | ✅     | ✅  |
| Error handling          | ✅     | ✅  |

## Building and using the python bindings

1. `cargo build`

2. `cargo run --bin uniffi-bindgen generate --library target/debug/libarithmetical.dylib --language python --out-dir consumers/python`

3. `ln -s target/debug/libarithmetical.dylib consumers/python/libarithmetical.dylib` **Note:** The extension might be `.so` or `.dll` depending on your OS.

4. `python consumers/python/app.py`

Note that `uniffi-bindgen` only needs to be ran when we make changes to the UDL file. If you are just updating the Rust code, you simply need to build and then call the python script.

## Building and using the web bindings

1. `wasm-pack build --target web -d ./consumers/web/pkg`

2. `cd consumers/web && python -m http.server`

3. Open the webpage at `http://localhost:8000`

## TODO

- [ ] Try to also integrate [wasm-pack](https://github.com/rustwasm/wasm-pack) to target web assembly that can run in the browser
- [ ] Test on Windows
- [ ] Test on Linux
