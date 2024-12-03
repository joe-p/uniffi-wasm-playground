/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate async_std;

use async_std::future::{pending, timeout};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::ffi::c_void;
use std::time::Duration;
use wasm_bindgen::prelude::wasm_bindgen;

mod falcon_ffi {
    // We would ideally use rust-bindgen to generate this, but it doesn't work with wasm, so we have to handwrite the bindings for now
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct shake256_context {
        pub opaque_contents: [u64; 26usize],
    }

    extern "C" {
        pub fn falcon_det1024_keygen(
            rng: *mut shake256_context,
            privkey: *mut ::std::os::raw::c_void,
            pubkey: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }

    extern "C" {
        pub fn shake256_init_prng_from_seed(
            sc: *mut shake256_context,
            seed: *const ::std::os::raw::c_void,
            seed_len: usize,
        );
    }
}

#[wasm_bindgen]
pub struct FalconKeyPair {
    #[wasm_bindgen(getter_with_clone)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter_with_clone)]
    pub private_key: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum FalconError {
    #[error("Falcon keygen failed with error code {0}")]
    FalconKeygenFailed(i32),
}

impl From<FalconError> for wasm_bindgen::JsValue {
    fn from(error: FalconError) -> Self {
        wasm_bindgen::JsValue::from_str(&error.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("Integer overflow on an operation with {a} and {b}")]
    IntegerOverflow { a: u64, b: u64 },
}

impl From<ArithmeticError> for wasm_bindgen::JsValue {
    fn from(error: ArithmeticError) -> Self {
        wasm_bindgen::JsValue::from_str(&error.to_string())
    }
}

#[wasm_bindgen]
pub fn add(a: u64, b: u64) -> Result<u64, ArithmeticError> {
    a.checked_add(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

#[wasm_bindgen]
pub fn sub(a: u64, b: u64) -> Result<u64, ArithmeticError> {
    a.checked_sub(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

#[wasm_bindgen]
pub fn div(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        panic!("Can't divide by zero");
    }
    dividend / divisor
}

#[wasm_bindgen]
pub fn equal(a: u64, b: u64) -> bool {
    a == b
}

#[wasm_bindgen]
pub async fn say_after(ms: u64, who: String) -> String {
    println!("called say_after({ms}, {who})");
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    println!("done say_after({ms}, {who})");
    format!("Hello, {who}!")
}

#[wasm_bindgen]
pub async fn http_get(url: String) -> String {
    println!("called http_get({})", &url);
    let body = surf::get(&url).recv_string().await.unwrap();

    println!("done http_get({url})");
    body
}

#[wasm_bindgen]
pub fn genkey() -> Vec<u8> {
    let mut csprng: OsRng = OsRng {};
    let signing_key = SigningKey::generate(&mut csprng);
    signing_key.to_bytes().to_vec()
}

#[wasm_bindgen]
pub fn falcon_genkey(seed: Vec<u8>) -> Result<FalconKeyPair, FalconError> {
    const PUBLIC_KEY_SIZE: usize = 1793; // FALCON_DET1024_PUBKEY_SIZE
    const PRIVATE_KEY_SIZE: usize = 2305; // FALCON_DET1024_PRIVKEY_SIZE

    let mut public_key = vec![0u8; PUBLIC_KEY_SIZE];
    let mut private_key = vec![0u8; PRIVATE_KEY_SIZE];

    let result = unsafe {
        if seed.is_empty() {
            let mut rng = std::mem::zeroed();
            falcon_ffi::shake256_init_prng_from_seed(&mut rng, std::ptr::null(), 0);
            falcon_ffi::falcon_det1024_keygen(
                &mut rng,
                private_key.as_mut_ptr() as *mut c_void,
                public_key.as_mut_ptr() as *mut c_void,
            )
        } else {
            let mut rng = std::mem::zeroed();
            falcon_ffi::shake256_init_prng_from_seed(
                &mut rng,
                seed.as_ptr() as *const c_void,
                seed.len() as usize,
            );
            falcon_ffi::falcon_det1024_keygen(
                &mut rng,
                private_key.as_mut_ptr() as *mut c_void,
                public_key.as_mut_ptr() as *mut c_void,
            )
        }
    };

    if result != 0 {
        return Err(FalconError::FalconKeygenFailed(result));
    }

    Ok(FalconKeyPair {
        public_key,
        private_key,
    })
}

#[cfg(not(target_arch = "wasm32"))]
uniffi::include_scaffolding!("arithmetic");
