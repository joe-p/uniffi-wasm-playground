/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native=falcon/");

    // Tell cargo to tell rustc to link the static library libfalcon.a
    println!("cargo:rustc-link-lib=static=falcon");

    let lib_path = PathBuf::from("falcon/libfalcon.a");
    println!("cargo:warning=Checking for library at: {:?}", lib_path);
    println!("cargo:warning=File exists: {}", lib_path.exists());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/falcon.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Print where we wrote the bindings
    println!(
        "cargo:warning=Bindings written to: {:?}",
        out_path.join("bindings.rs")
    );

    uniffi::generate_scaffolding("src/arithmetic.udl").unwrap();
}
