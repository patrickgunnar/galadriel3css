#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod ast {
  pub mod stylitron;
}

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
  pub mod gatekeeper;
  pub mod intaker;
  pub mod pathify;
  pub mod readify;
  pub mod trailblazer;
}

use rustal::alchemist::Alchemist;
use rustal::blueprint::Blueprint;
use rustal::codelyzer::Codelyzer;
use rustal::configatron::{configatron_init, Configatron};
use rustal::gatekeeper::Gatekeeper;
use rustal::intaker::Intaker;
use rustal::pathify::pathify;
use rustal::readify::readify;
use rustal::trailblazer::Trailblazer;

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
        let trailblazer = Trailblazer::new();

        if let Ok((css_path, js_path)) = trailblazer.format_path(&path) {
          let is_css_file = trailblazer.generates_css(&css_path);

          if is_css_file {
            let is_js_file = trailblazer.generates_js(&js_path);

            if !is_js_file {
              blueprint.error("something went wrong whiling creating a JS file".to_string());
              blueprint.info(format!("path not processed: {}", path).to_string());
            }
          } else {
            blueprint.error("something went wrong whiling creating a CSS file".to_string());
            blueprint.info(format!("path not processed: {}", path).to_string());
          }

          trailblazer.clear_stylitron();
          trailblazer.clear_stylometric();
        }
      }
    }
  } else {
    blueprint.error("something went wrong whiling processing a file".to_string());
    blueprint.info(format!("path not processed: {}", path).to_string());
  }
}

#[napi]
pub fn process_gatekeeper() {
  let blueprint = Blueprint::new();
  let trailblazer = Trailblazer::new();
  let configatron = Configatron::new();
  let config = configatron.collects_from_rust(vec!["modular"]);

  let modular = match config.get("modular") {
    Some(Value::Bool(b)) => *b,
    _ => false,
  };

  if modular {
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

    let groups = gatekeeper.group_paths(&processed_paths);
    let is_html_file = trailblazer.generates_html(groups);

    if !is_html_file {
      blueprint.error("something went wrong whiling creating a HTML file".to_string());
      blueprint.info(format!("global generation not processed").to_string());
    } else {
      blueprint.log(blueprint.bold("all processes were completed!".to_string()));
    }
  } else {
    let is_css_file = trailblazer.generates_css(&".galadriel/global.css".to_string());

    if is_css_file {
      let is_js_file = trailblazer.generates_js(&".galadriel/global.js".to_string());

      if is_js_file {
        let mut groups: Vec<Vec<String>> = vec![];
        let mut group: Vec<String> = vec![];

        group.push("/.galadriel/global".to_string());
        groups.push(group);

        let is_html_file = trailblazer.generates_html(groups);

        if !is_html_file {
          blueprint.error("something went wrong whiling creating a HTML file".to_string());
          blueprint.info(format!("global generation not processed").to_string());
        }
      } else {
        blueprint.error("something went wrong whiling creating a JS file".to_string());
        blueprint.info(format!("global generation not processed").to_string());
      }
    } else {
      blueprint.error("something went wrong whiling creating a CSS file".to_string());
      blueprint.info(format!("global generation not processed").to_string());
    }

    trailblazer.clear_stylitron();
    trailblazer.clear_stylometric();
  }
}
