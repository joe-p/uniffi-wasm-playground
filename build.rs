/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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

    uniffi::generate_scaffolding("src/arithmetic.udl").unwrap();
}
