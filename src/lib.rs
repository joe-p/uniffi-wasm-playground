/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate async_std;

use async_std::future::{pending, timeout};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::time::Duration;
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

mod falcon_ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(not(target_arch = "wasm32"))]
uniffi::setup_scaffolding!();

// If we weren't using tsify here, we'd have to wasm_bindgen(getter_with_clone) for each field
// This would result in the returned object being a WASM pointer which could be confusing for consumers of the JS lib
// It also gives us a better type definition in the resulting .d.ts file
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Record))]
/// A deterministic Falcon-1024 key pair
pub struct FalconKeyPair {
    /// The public key
    pub public_key: Vec<u8>,
    /// The private key
    pub private_key: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Error))]
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
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Error))]
pub enum PlaygroundError {
    #[error("Integer overflow on an operation with {a} and {b}")]
    IntegerOverflow { a: u64, b: u64 },
}

impl From<PlaygroundError> for wasm_bindgen::JsValue {
    fn from(error: PlaygroundError) -> Self {
        wasm_bindgen::JsValue::from_str(&error.to_string())
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn add(a: u64, b: u64) -> Result<u64, PlaygroundError> {
    a.checked_add(b)
        .ok_or(PlaygroundError::IntegerOverflow { a, b })
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn sub(a: u64, b: u64) -> Result<u64, PlaygroundError> {
    a.checked_sub(b)
        .ok_or(PlaygroundError::IntegerOverflow { a, b })
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn div(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        panic!("Can't divide by zero");
    }
    dividend / divisor
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn equal(a: u64, b: u64) -> bool {
    a == b
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub async fn say_after(ms: u64, who: String) -> String {
    println!("called say_after({ms}, {who})");
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    println!("done say_after({ms}, {who})");
    format!("Hello, {who}!")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub async fn http_get(url: String) -> String {
    println!("called http_get({})", &url);
    let body = surf::get(&url).recv_string().await.unwrap();

    println!("done http_get({url})");
    body
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn genkey() -> Vec<u8> {
    let mut csprng: OsRng = OsRng {};
    let signing_key = SigningKey::generate(&mut csprng);
    signing_key.to_bytes().to_vec()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
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
