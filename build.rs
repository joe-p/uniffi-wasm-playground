/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::{env, path::PathBuf};

fn link_c() {
    let mut build = cc::Build::new();

    if let Some(libc) = std::env::var_os("DEP_WASM32_UNKNOWN_UNKNOWN_OPENBSD_LIBC_INCLUDE") {
        build.include(libc);
        println!("cargo::rustc-link-lib=wasm32-unknown-unknown-openbsd-libc");
    }

    build.include("falcon");

    // Get all the C files in falcon
    let falcon_files = std::fs::read_dir("falcon")
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| Some(ext == "c"))
                .unwrap_or(false)
        });
    build.files(falcon_files);
    build.compile("c_falcon");

    // We don't need to generate bindings for all the headers, so we specifify the desired headers here
    let headers = vec!["falcon/falcon.h", "falcon/deterministic.h"];

    // Additionally we don't need to use all the types and functions in these headers, so we specify the desired ones here
    let allowed_types = vec!["shake256_context"];
    let allowed_functions = vec!["falcon_det1024_keygen", "shake256_init_prng_from_seed"];

    let mut bindings_builder = bindgen::Builder::default()
        // This flag is needed when targeting wasm
        // See https://github.com/rust-lang/rust-bindgen/issues/2624#issuecomment-2518152955
        .clang_arg("-fvisibility=default")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for h in headers {
        bindings_builder = bindings_builder.header(h);
    }

    for t in allowed_types {
        bindings_builder = bindings_builder.allowlist_type(t);
    }

    for f in allowed_functions {
        bindings_builder = bindings_builder.allowlist_function(f);
    }

    let bindings = bindings_builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!(
        "cargo:warning=Bindings generated in: {}",
        out_path.join("bindings.rs").display()
    );
}

fn main() {
    link_c();
    uniffi::generate_scaffolding("src/playground.udl").unwrap();
}
