/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use rmp_serde;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::future::pending;
use std::sync::atomic::AtomicU64;
use std::time::Duration;
use tokio::time::timeout;
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
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export(async_runtime = "tokio"))]
pub async fn say_after(ms: u64, who: String) -> String {
    println!("called say_after({ms}, {who})");
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    println!("done say_after({ms}, {who})");
    format!("Hello, {who}!")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export(async_runtime = "tokio"))]
pub async fn http_get(url: String) -> String {
    println!("called http_get({})", &url);
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap();

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

// Below are examples of passing objects across the FFI boundary
// With objects, the implementation needs to be different for WASM and UniFFI
// The main difference is that UniFFI objects must be thread safe (Send + Sync)
// thus use atomic types or locks to enable safe access.
// WASM, however, not only doesn't need thread safety but it SHOULD NOT have locks
// because WASM is single threaded.

#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
pub struct FavoriteNumbers {
    pub numbers: std::sync::Mutex<Vec<u64>>,
    pub max_number: AtomicU64,
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
impl FavoriteNumbers {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            numbers: std::sync::Mutex::new(Vec::new()),
            max_number: AtomicU64::new(0),
        }
    }

    pub fn add_number(&self, number: u64) {
        self.numbers.lock().unwrap().push(number);
        self.max_number
            .fetch_max(number, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn find_min(&self) -> u64 {
        self.numbers
            .lock()
            .unwrap()
            .iter()
            .copied()
            .min()
            .unwrap_or(0)
    }

    pub fn quick_sort(&self, numbers: Option<Vec<u64>>) -> Vec<u64> {
        let numbers = numbers.unwrap_or_else(|| self.numbers.lock().unwrap().clone());
        if numbers.len() <= 1 {
            return numbers;
        }

        let pivot = numbers[numbers.len() - 1];
        let less = numbers[..numbers.len() - 1]
            .iter()
            .filter(|&&x| x <= pivot)
            .copied()
            .collect();
        let greater = numbers[..numbers.len() - 1]
            .iter()
            .filter(|&&x| x > pivot)
            .copied()
            .collect();

        let mut result = self.quick_sort(Some(less));
        result.push(pivot);
        result.extend(self.quick_sort(Some(greater)));
        result
    }
}

#[wasm_bindgen]
pub struct WasmFavoriteNumbers {
    // getter_with_clone means the JS object will have a readonly getter that returns a clone of the vec
    // rather than a reference.
    #[wasm_bindgen(getter_with_clone)]
    pub numbers: Vec<u64>,
    pub max_number: u64,
}

#[wasm_bindgen]
impl WasmFavoriteNumbers {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
            max_number: 0,
        }
    }

    #[wasm_bindgen]
    pub fn add_number(&mut self, number: u64) {
        self.numbers.push(number);
        self.max_number = self.max_number.max(number);
    }

    #[wasm_bindgen]
    pub fn find_min(&self) -> u64 {
        self.numbers.iter().min().copied().unwrap_or(0)
    }

    #[wasm_bindgen]
    pub fn quick_sort(&self, numbers: Option<Vec<u64>>) -> Vec<u64> {
        let numbers = numbers.unwrap_or_else(|| self.numbers.clone());
        if numbers.len() <= 1 {
            return numbers;
        }

        let pivot = numbers[numbers.len() - 1];
        let less = numbers[..numbers.len() - 1]
            .iter()
            .filter(|&&x| x <= pivot)
            .copied()
            .collect();
        let greater = numbers[..numbers.len() - 1]
            .iter()
            .filter(|&&x| x > pivot)
            .copied()
            .collect();

        let mut result = self.quick_sort(Some(less));
        result.push(pivot);
        result.extend(self.quick_sort(Some(greater)));
        result
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Record))]
#[derive(Tsify, Deserialize, Serialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UserRecord {
    pub id: u64,
    pub favorite_numbers: Vec<u64>,
    pub favorite_colors: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Object))]
pub struct UserObject {
    pub user_record: UserRecord,
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
impl UserObject {
    pub fn serialize(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self.user_record).unwrap()
    }

    pub fn to_record(&self) -> UserRecord {
        self.user_record.clone()
    }
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn user_object_from_record(record: UserRecord) -> UserObject {
    UserObject {
        user_record: record,
    }
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn no_op() {
    // no-op
}

#[wasm_bindgen]
pub struct WasmUserObject {
    pub id: u64,
    #[wasm_bindgen(getter_with_clone)]
    pub favorite_numbers: Vec<u64>,
    #[wasm_bindgen(getter_with_clone)]
    pub favorite_colors: Vec<String>,
}

#[wasm_bindgen]
impl WasmUserObject {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u64, favorite_numbers: Vec<u64>, favorite_colors: Vec<String>) -> Self {
        Self {
            id,
            favorite_numbers,
            favorite_colors,
        }
    }

    pub fn to_record(&self) -> UserRecord {
        UserRecord {
            id: self.id,
            favorite_numbers: self.favorite_numbers.clone(),
            favorite_colors: self.favorite_colors.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn wasm_user_object_from_record(record: UserRecord) -> WasmUserObject {
    WasmUserObject {
        id: record.id,
        favorite_numbers: record.favorite_numbers,
        favorite_colors: record.favorite_colors,
    }
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait AsyncAdder: Send + Sync {
    async fn add_async(&self, a: u64, b: u64) -> u64;
}

#[uniffi::export]
pub async fn call_async_adder(adder: &dyn AsyncAdder, a: u64, b: u64) -> u64 {
    adder.add_async(a, b).await
}
