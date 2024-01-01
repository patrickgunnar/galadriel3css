use crate::ast::stylitron::STYLITRON;
use crate::core::nucleus::STYLOMETRIC;
use crate::core::screen_core::SCREEN_CORE;

use std::fs::{self};

pub struct Trailblazer;

impl Trailblazer {
  pub fn new() -> Self {
    Trailblazer {}
  }

  pub fn format_path(&self, input: &str) -> Result<(String, String), String> {
    let path = input.replace("./", "").replace("@/", "");
    let mut parts: Vec<&str> = path.split(".").filter(|e| !e.is_empty()).collect();

    parts.pop();

    let formatted_path = format! {".galadriel/{}", parts.join("")};

    Ok((
      format!("{}.css", formatted_path),
      format!("{}.js", formatted_path),
    ))
  }

  pub fn clear_stylitron(&self) {
    let mut stylitron = STYLITRON.lock().unwrap();

    for (_, data) in stylitron.iter_mut() {
      for (_, value) in data.iter_mut() {
        value.clear();
      }
    }
  }

  pub fn clear_stylometric(&self) {
    let mut stylometric = STYLOMETRIC.lock().unwrap();

    stylometric.clear();
  }

  fn create_file(path: &String, data: &String) -> bool {
    if !path.is_empty() && !data.is_empty() {
      let mut parts: Vec<&str> = path.split("/").filter(|e| !e.is_empty()).collect();

      parts.pop();

      let dir_path = parts.join("/");

      if let Err(_) = fs::create_dir_all(dir_path) {
        return false;
      } else {
        if let Err(_) = fs::write(path, data) {
          return false;
        } else {
          return true;
        }
      }
    }

    false
  }

  pub fn generates_js(&self, path: &String) -> bool {
    let stylometric = STYLOMETRIC.lock().unwrap();
    let mut result = "var ".to_string();

    for (key, value) in stylometric.iter() {
      let formatted = format!("{}={},", key, format!("{:?}", value));

      result += formatted.as_str();
    }

    result.remove(result.len() - 1);

    Self::create_file(path, &result)
  }

  pub fn generates_css(&self, path: &String) -> bool {
    let stylitron = STYLITRON.lock().unwrap();
    let mut result = String::new();
    let mut medias = String::new();

    for (key, data) in stylitron.iter() {
      let mut query = String::new();
      let mut query_value = String::new();

      for (property, value) in data.iter() {
        let formatted_value = value.join("");

        if !formatted_value.is_empty() {
          if key == "mediaQueryVariables" {
            query += formatted_value.as_str();

            if query_value.is_empty() {
              query_value = if let Some(v) = SCREEN_CORE.get(property) {
                v.to_string()
              } else {
                "".to_string()
              }
            }
          } else {
            result += formatted_value.as_str();
          }
        }
      }

      if !query_value.is_empty() {
        let formatted_query = format!("@media only screen and ({}){{{}}}", query_value, query);

        medias += formatted_query.as_str();
      }
    }

    if !medias.is_empty() {
      result += medias.as_str();
    }

    Self::create_file(path, &result)
  }
}
