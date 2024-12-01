`cargo build`

`cargo run --bin uniffi-bindgen generate --library target/debug/libarithmetical.dylib --language python --out-dir consumers/python`

`ln -s target/debug/libarithmetical.dylib consumers/python/libarithmetical.dylib`

`python consumers/python/app.py`
