#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod ast {
  pub mod stylitron;
}

//use ast::stylitron::STYLITRON;

pub mod core {
  pub mod nucleus;
  pub mod property_core;
  pub mod screen_core;
  pub mod selector_core;
}

use core::nucleus::NUCLEUS_CONFIG;
//use core::nucleus::STYLOMETRIC;

pub mod rustal {
  pub mod alchemist;
  pub mod blueprint;
  pub mod classinator;
  pub mod codelyzer;
  pub mod configatron;
  pub mod gatekeeper;
  pub mod readify;
}

use rustal::alchemist::Alchemist;
use rustal::blueprint::Blueprint;
use rustal::codelyzer::Codelyzer;
use rustal::configatron::{configatron_init, Configatron};
use rustal::readify::readify;
use rustal::gatekeeper::Gatekeeper;

use serde_json::Value;

#[napi]
pub fn configatron_initializer() {
  *NUCLEUS_CONFIG.lock().unwrap() = configatron_init();
}

#[napi]
pub fn process_path(path: String) {
  let blueprint = Blueprint::new();
  let configatron = Configatron::new();
  let config = configatron.collects_from_rust(vec!["modular"]);

  if let Ok(code) = readify(&path) {
    let mut codelyzer = Codelyzer::new(code.clone());

    if let Ok((_, map)) = codelyzer.parser_code() {
      let modular = match config.get("modular") {
        Some(Value::Bool(b)) => *b,
        _ => false,
      };

      let alchemist = Alchemist::new(modular);

      alchemist.process_objects(path.as_str(), map);

      let _gatekeeper = Gatekeeper::new(modular, path, code);

      //println!("{:#?}", STYLITRON.lock().unwrap());
      //println!("{:#?}", STYLOMETRIC.lock().unwrap());
    }
  } else {
    blueprint.error("something went wrong whiling processing a file".to_string());
    blueprint.info(format!("path not processed: {}", path).to_string());
  }
}
