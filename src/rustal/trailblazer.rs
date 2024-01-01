use crate::ast::stylitron::STYLITRON;
use crate::core::nucleus::STYLOMETRIC;
use crate::core::screen_core::SCREEN_CORE;

use std::fs::{self};

pub struct Trailblazer;

impl Trailblazer {
  pub fn new() -> Self {
    Trailblazer {}
  }

  // Format the input path to generate CSS and JS file paths.
  pub fn format_path(&self, input: &str) -> Result<(String, String), String> {
    // Remove leading "./" and "@/" from the input path.
    let path = input.replace("./", "").replace("@/", "");
    // Split the path into parts using "." as a separator and filter out empty elements.
    let mut parts: Vec<&str> = path.split(".").filter(|e| !e.is_empty()).collect();

    // Remove the last element (pop the file extension).
    parts.pop();

    // Format the path for the generated CSS and JS files in the ".galadriel/" directory.
    let formatted_path = format! {".galadriel/{}", parts.join("")};

    // Return the formatted CSS and JS file paths as a Result.
    Ok((
      format!("{}.css", formatted_path),
      format!("{}.js", formatted_path),
    ))
  }

  // Clear the contents of the STYLITRON data structure.
  pub fn clear_stylitron(&self) {
    // Obtain a lock on the STYLITRON data structure.
    let mut stylitron = STYLITRON.lock().unwrap();

    // Iterate over each entry in the STYLITRON data structure.
    for (_, data) in stylitron.iter_mut() {
      // Iterate over each entry within the nested data structure.
      for (_, value) in data.iter_mut() {
        // Clear the contents of the inner vector.
        value.clear();
      }
    }
  }

  // Clear the contents of the STYLOMETRIC data structure.
  pub fn clear_stylometric(&self) {
    // Obtain a lock on the STYLOMETRIC data structure.
    let mut stylometric = STYLOMETRIC.lock().unwrap();

    // Clear the entire STYLOMETRIC data structure.
    stylometric.clear();
  }

  // Create a file with the provided path and write the data to it.
  // Returns true if the operation is successful, false otherwise.
  fn create_file(path: &String, data: &String) -> bool {
    // Check if both path and data are not empty.
    if !path.is_empty() && !data.is_empty() {
      // Split the path into parts, removing empty elements.
      let mut parts: Vec<&str> = path.split("/").filter(|e| !e.is_empty()).collect();

      // Remove the last element (file name) to get the directory path.
      parts.pop();

      let dir_path = parts.join("/");

      // Attempt to create directories recursively. If there's an error, return false.
      if let Err(_) = fs::create_dir_all(dir_path) {
        return false;
      } else {
        // Attempt to write the data to the file. If there's an error, return false.
        if let Err(_) = fs::write(path, data) {
          return false;
        } else {
          // Return true if both directory creation and file writing are successful.
          return true;
        }
      }
    }

    // Return false if either path or data is empty.
    false
  }

  // Generate an HTML file based on the provided groups of CSS and JS paths.
  // Returns true if the operation is successful, false otherwise.
  pub fn generates_html(&self, groups: Vec<Vec<String>>) -> bool {
    // Initialize an empty string to store the HTML content.
    let mut result = String::new();

    // Iterate over each group of paths.
    for (idx, group) in groups.iter().enumerate() {
      // Iterate over each path in the group.
      for path in group.iter() {
        // Format the link tag for the CSS file.
        let script_css = format!("<link rel=\"stylesheet\" href=\"{}.css\" data-group=\"galadriel_{}\" data-precedence=\"galadriel\" />", path, idx);
        // Format the script tag for the JS file.
        let script_js = format!("<script src=\"{}.js\" data-group=\"galadriel_{}\" data-precedence=\"galadriel\"></script>", path, idx);

        // Concatenate the formatted CSS and JS tags to the result string.
        result += script_css.as_str();
        result += script_js.as_str();
      }
    }

    // Format the result as the HTML header.
    let header = format!("{}", result);

    // Create the HTML file with the generated header content.
    Self::create_file(&".galadriel/galadriel.html".to_string(), &header)
  }

  // Generate a JavaScript file based on the stylometric data.
  // Returns true if the operation is successful, false otherwise.
  pub fn generates_js(&self, path: &String) -> bool {
    // Lock the STYLOMETRIC data structure for exclusive access.
    let stylometric = STYLOMETRIC.lock().unwrap();
    // Initialize a string with the starting part of the JavaScript content.
    let mut result = "var ".to_string();

    // Iterate over each entry in the stylometric data.
    for (key, value) in stylometric.iter() {
      // Format the entry as a JavaScript variable assignment.
      let formatted = format!("{}={},", key, format!("{:?}", value));

      // Concatenate the formatted entry to the result string.
      result += formatted.as_str();
    }

    // Remove the trailing comma from the result string.
    result.remove(result.len() - 1);

    // Create the JavaScript file with the generated content.
    Self::create_file(path, &result)
  }

  // Generate a CSS file based on the stylitron data.
  // Returns true if the operation is successful, false otherwise.
  pub fn generates_css(&self, path: &String) -> bool {
    // Lock the STYLITRON data structure for exclusive access.
    let stylitron = STYLITRON.lock().unwrap();
    // Initialize variables to store the generated CSS and media queries.
    let mut result = String::new();
    let mut medias = String::new();

    // Iterate over each entry in the stylitron data.
    for (key, data) in stylitron.iter() {
      // Initialize variables to store a media query and its value.
      let mut query = String::new();
      let mut query_value = String::new();

      // Iterate over each property-value pair in the entry.
      for (property, value) in data.iter() {
        // Concatenate the formatted value (CSS) for non-media query entries.
        let formatted_value = value.join("");

        if !formatted_value.is_empty() {
          if key == "mediaQueryVariables" {
            // Concatenate the formatted value for media query entries.
            query += formatted_value.as_str();

            // If the query value is empty, get the corresponding value from SCREEN_CORE.
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

      // If a media query value is present, format the media query and concatenate it.
      if !query_value.is_empty() {
        let formatted_query = format!("@media only screen and ({}){{{}}}", query_value, query);

        medias += formatted_query.as_str();
      }
    }

    // If media queries are present, concatenate them to the result.
    if !medias.is_empty() {
      result += medias.as_str();
    }

    // Create the CSS file with the generated content.
    Self::create_file(path, &result)
  }
}
