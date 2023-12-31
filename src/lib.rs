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
  pub mod intaker;
  pub mod pathify;
  pub mod readify;
}

use rustal::alchemist::Alchemist;
use rustal::blueprint::Blueprint;
use rustal::codelyzer::Codelyzer;
use rustal::configatron::{configatron_init, Configatron};
use rustal::gatekeeper::Gatekeeper;
use rustal::intaker::Intaker;
use rustal::pathify::pathify;
use rustal::readify::readify;

use ignore::WalkBuilder;
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

      if modular {
        // IF MODULAR, GENERATES THE CSS AND JS FILES.
        // IF NOT MODULAR, GENERATES THE CSS AND JS FILES ON THE process_gatekeeper
      }
    }
  } else {
    blueprint.error("something went wrong whiling processing a file".to_string());
    blueprint.info(format!("path not processed: {}", path).to_string());
  }
}

#[napi]
pub fn process_gatekeeper() {
  let mut processed_paths: Vec<String> = vec![];
  let mut gatekeeper = Gatekeeper::new();
  let paths: Vec<_> = WalkBuilder::new(".")
    .build()
    .filter_map(Result::ok)
    .filter(|entry| {
      entry.file_type().map_or(false, |t| t.is_file())
        && entry.path().extension().map_or(false, |ext| {
          ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx"
        })
    })
    .map(|entry| entry.into_path())
    .collect();

  for path in paths.into_iter() {
    if let Ok(code) = readify(&path.to_string_lossy()) {
      if code.contains("createStyles") {
        let path_string = path.to_string_lossy().to_string();
        let parts: Vec<&str> = path_string
          .split(".")
          .filter(|entry| !entry.is_empty())
          .collect();

        if let Some(path_str) = parts.first() {
          if !processed_paths.contains(&path_str.to_string()) {
            processed_paths.push(path_str.to_string());
          }

          let intaker = Intaker::new();
          let imports = intaker.process_code(code);

          for import in imports.iter() {
            let formatted_import = pathify(&path_string, import);

            gatekeeper.add_import(path_str, &formatted_import);
          }
        }
      }
    }
  }

  //gatekeeper.print_graph();
  let groups = gatekeeper.group_paths(&processed_paths);
  println!("{:#?}", groups);

  // GENERATES THE CSS FILE AND JS FILE CONTAINING THE CLASS NAMES ON GLOBAL CONFIGURATION IN HERE.
  // GENERATED THE INITIAL JS FILE CONTAINING THE GROUPS.
}
