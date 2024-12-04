/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::{env, path::PathBuf};

fn main() {
    let mut build = cc::Build::new();

    if let Some(libc) = std::env::var_os("DEP_WASM32_UNKNOWN_UNKNOWN_OPENBSD_LIBC_INCLUDE") {
        build.include(libc);
        println!("cargo::rustc-link-lib=wasm32-unknown-unknown-openbsd-libc");
    }

    build.include("falcon");
    build.file("falcon/shake.c");
    build.file("falcon/common.c");
    build.file("falcon/codec.c");
    build.file("falcon/keygen.c");
    build.file("falcon/deterministic.c");
    build.file("falcon/fpr.c");
    build.file("falcon/sign.c");
    build.file("falcon/fft.c");
    build.file("falcon/falcon.c");
    build.file("falcon/vrfy.c");
    build.file("falcon/rng.c");
    build.compile("c_falcon");

    let bindings = bindgen::Builder::default()
        // The headers we want to generate bindings for
        .header("falcon/falcon.h")
        .header("falcon/deterministic.h")
        // See https://github.com/rust-lang/rust-bindgen/issues/2624#issuecomment-2518152955
        .clang_arg("-fvisibility=default")
        // To avoid generating bindings for unused types/functions, we'll use allow lists
        .allowlist_type("shake256_context")
        .allowlist_function("falcon_det1024_keygen")
        .allowlist_function("shake256_init_prng_from_seed")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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

    uniffi::generate_scaffolding("src/arithmetic.udl").unwrap();
}
