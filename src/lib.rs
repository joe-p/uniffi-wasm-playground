/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate async_std;

use async_std::future::{pending, timeout};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("Integer overflow on an operation with {a} and {b}")]
    IntegerOverflow { a: u64, b: u64 },
}

fn add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

fn sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

fn div(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        panic!("Can't divide by zero");
    }
    dividend / divisor
}

fn equal(a: u64, b: u64) -> bool {
    a == b
}

pub async fn say_after(ms: u64, who: String) -> String {
    println!("called say_after({ms}, {who})");
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    println!("done say_after({ms}, {who})");
    format!("Hello, {who}!")
}

type Result<T, E = ArithmeticError> = std::result::Result<T, E>;

uniffi::include_scaffolding!("arithmetic");
