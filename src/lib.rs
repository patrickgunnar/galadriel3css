#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod core {
  pub mod nucleus;
  pub mod property_core;
  pub mod screen_core;
  pub mod selector_core;
}

use core::nucleus::NUCLEUS_CONFIG;

pub mod rustal {
  pub mod alchemist;
  pub mod blueprint;
  pub mod classinator;
  pub mod codelyzer;
  pub mod configatron;
  pub mod file_reader;
}

use rustal::alchemist::Alchemist;
use rustal::codelyzer::Codelyzer;
use rustal::configatron::{configatron_init, Configatron};

use serde_json::Value;

#[napi]
pub fn configatron_initializer() {
  *NUCLEUS_CONFIG.lock().unwrap() = configatron_init();
}

#[napi]
pub fn process_path(path: String) {
  let configatron = Configatron::new();
  let config = configatron.collects_from_rust(vec!["modular"]);
  let mut codelyzer = Codelyzer::new(path.as_str());

  if let Ok((_, map)) = codelyzer.parser_code() {
    let modular = match config.get("modular") {
      Some(Value::Bool(b)) => *b,
      _ => false,
    };

    let alchemist = Alchemist::new(modular);

    alchemist.process_objects(path.as_str(), map);
  }
}
