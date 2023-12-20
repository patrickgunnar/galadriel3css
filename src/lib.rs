#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod rustal {
  pub mod blueprint;
  pub mod configatron;
  pub mod dummy_test;
  pub mod file_reader;
  pub mod nucleus;
}

use napi::Result;
use rustal::configatron::{Configatron, configatron_init};
use rustal::nucleus::NUCLEUS_CONFIG;

#[napi]
pub fn configatron_initializer() {
  *NUCLEUS_CONFIG.lock().unwrap() = configatron_init();
}

#[napi]
pub fn process_path(_path: String) -> Result<()> {
  let configatron = Configatron::new();
  let data = configatron.collect_from_rust(vec!["global", "module", "ajx"]);

  println!("{:?}", data);

  Ok(())
}

// -------------------------------------------------------
use rustal::dummy_test::*;

#[napi]
pub fn dummy_sum(a: i32, b: i32) -> String {
  dummy_test(a + b)
}
