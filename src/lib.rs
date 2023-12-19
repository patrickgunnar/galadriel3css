#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod rustal {
  pub mod blueprint;
  pub mod dummy_test;
}

use rustal::blueprint::Blueprint;

use napi::Result;

#[napi]
pub fn process_path(path: String) -> Result<()> {
  let blueprint = Blueprint::new();
  let msg = format!("starting to process current path: {}", blueprint.bold(&path)).to_string();

  print!("\n");
  blueprint.title("Galadriel3CSS Build Process");
  blueprint.log(&msg);
  blueprint.log(&msg);
  blueprint.log(&msg);

  print!("\n");
  blueprint.error(&msg);
  blueprint.error(&msg);
  blueprint.error(&msg);

  print!("\n");
  blueprint.info(&msg);
  blueprint.info(&msg);
  blueprint.info(&msg);

  print!("\n");
  blueprint.warn(&msg);
  blueprint.warn(&msg);
  blueprint.warn(&msg);

  Ok(())
}

// -------------------------------------------------------
use rustal::dummy_test::*;

#[napi]
pub fn dummy_sum(a: i32, b: i32) -> String {
  dummy_test(a + b)
}
