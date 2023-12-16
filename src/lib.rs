#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod rustal {
  pub mod dummy_test;
}

use rustal::dummy_test::*;

#[napi]
pub fn dummy_sum(a: i32, b: i32) -> String {
  dummy_test(a + b)
}
