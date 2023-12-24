#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod rustal {
  pub mod blueprint;
  pub mod codelyzer;
  pub mod configatron;
  pub mod file_reader;
  pub mod nucleus;
}

use rustal::nucleus::NUCLEUS_CONFIG;
use rustal::configatron::{configatron_init, Configatron};
use rustal::codelyzer::Codelyzer;

#[napi]
pub fn configatron_initializer() {
  *NUCLEUS_CONFIG.lock().unwrap() = configatron_init();
}

#[napi]
pub fn process_path(_path: String) {
  let configatron = Configatron::new();
  let _data = configatron.collect_from_rust(vec!["global", "module", "ajx"]);
  
  let mut codelyzer = Codelyzer::new(_path.as_str());
  let _dt = codelyzer.parser_code();

  //println!("{:#?}", dt);
}
