use crate::core::property_core::PROPERTY_CORE;
use crate::core::screen_core::SCREEN_CORE;
use crate::core::selector_core::SELECTOR_CORE;
use serde_json::{from_str, Value};
use std::collections::HashMap;

pub struct Alchemist {
  modular: bool,
}

/*
    - Alchemist is responsible for the transformation of
    - objects into CSS styles, and also responsible for
    - add the generated CSS styles into the styles AST
*/
impl Alchemist {
  pub fn new(modular: bool) -> Self {
    Alchemist { modular }
  }

  fn generates_class_name(
    _is_modular: bool,
    _is_unique: bool,
    _property: &String,
    _value: String,
    _selector: &String,
  ) -> String {
    "".to_string()
  }

  fn styles_formatter(is_nested: bool, input: &String) -> String {
    let mut map: Vec<String> = Vec::new();
    let content: Result<serde_json::Value, _> = from_str(&input);

    match content {
      Ok(objects) => {
        if let Some(obj) = objects.as_object() {
          for (k, v) in obj.iter() {
            let property = if let Some(v) = PROPERTY_CORE.get(k) {
              v.to_string()
            } else {
              "".to_string()
            };

            let value = match v {
              Value::String(s) => s,
              _ => "",
            };

            if !property.is_empty() && !value.is_empty() {
              map.push(format!("{}: {}", property, value));
            }
          }
        }
      }
      Err(_) => {}
    }

    if map.len() > 0 && !is_nested {
      format!("{{ {} }}", map.join("; "))
    } else if is_nested {
      format!("{:?}", map)
    } else {
      "".to_string()
    }
  }

  fn next_sibling(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn subseq_sibling(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn type_of(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn direct_children(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_starts_with(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_contains(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_ends_with(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn nth_child(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn nth_of_type(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn empty(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn checked(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn disabled(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn focus(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn active(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn not(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn visited(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn last_child(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn first_child(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn descendent(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn html_tag(
    _is_modular: bool,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn process_children_objects(
    is_modular: bool,
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    if key.starts_with("nextSibling") {
      Self::next_sibling(is_modular, key.replace("nextSibling", ""), value)
    } else if key.starts_with("subseqSibling") {
      Self::subseq_sibling(is_modular, key.replace("subseqSibling", ""), value)
    } else if key.starts_with("typeOf") {
      Self::type_of(is_modular, key.replace("typeOf", ""), value)
    } else if key.starts_with("directChildren") {
      Self::direct_children(is_modular, key.replace("directChildren", ""), value)
    } else if key.starts_with("attrStartsWith") {
      Self::attr_starts_with(is_modular, key.replace("attrStartsWith", ""), value)
    } else if key.starts_with("attrContains") {
      Self::attr_contains(is_modular, key.replace("attrContains", ""), value)
    } else if key.starts_with("attrEndsWith") {
      Self::attr_ends_with(is_modular, key.replace("attrEndsWith", ""), value)
    } else if key.starts_with("nthChild") {
      Self::nth_child(is_modular, key.replace("nthChild", ""), value)
    } else if key.starts_with("nthOfType") {
      Self::nth_of_type(is_modular, key.replace("nthOfType", ""), value)
    } else if key.starts_with("empty") {
      Self::empty(is_modular, key.replace("empty", ""), value)
    } else if key.starts_with("checked") {
      Self::checked(is_modular, key.replace("checked", ""), value)
    } else if key.starts_with("disabled") {
      Self::disabled(is_modular, key.replace("disabled", ""), value)
    } else if key.starts_with("focus") {
      Self::focus(is_modular, key.replace("focus", ""), value)
    } else if key.starts_with("active") {
      Self::active(is_modular, key.replace("active", ""), value)
    } else if key.starts_with("not") {
      Self::not(is_modular, key.replace("not", ""), value)
    } else if key.starts_with("visited") {
      Self::visited(is_modular, key.replace("visited", ""), value)
    } else if key.starts_with("lastChild") {
      Self::last_child(is_modular, key.replace("lastChild", ""), value)
    } else if key.starts_with("firstChild") {
      Self::first_child(is_modular, key.replace("firstChild", ""), value)
    } else if key.starts_with("(") && key.ends_with(")") {
      Self::descendent(is_modular, key.to_string(), value)
    } else {
      Self::html_tag(is_modular, key.to_string(), value)
    }
  }

  fn process_nested_objects(
    _is_modular: bool,
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    let mut is_media_env = false;
    let selector = if let Some(v) = SELECTOR_CORE.get(key) {
      v.to_string()
    } else {
      if let Some(v) = SCREEN_CORE.get(key) {
        is_media_env = true;
        v.to_string()
      } else {
        "".to_string()
      }
    };

    if !selector.is_empty() {
      let pseudo_selector = if !is_media_env {
        selector
      } else {
        "".to_string()
      };

      let stringified_map = Self::styles_formatter(true, value);
      let styles_map: Result<Vec<String>, _> = from_str(&stringified_map.as_str());

      if let Ok(map) = styles_map {
        let mut stringified_styles: HashMap<String, String> = HashMap::new();

        for style in map.iter() {
          if style.len() > 0 && style.contains(":") {
            let styles = format!(
              ".{}{} {{ {} }}",
              "galadriel_4414455", pseudo_selector, style
            );

            stringified_styles.insert("galadriel_4414455".to_string(), styles);
          }
        }

        return Ok((
          is_media_env,
          format!("{:?}", stringified_styles),
          "".to_string(),
        ));
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  fn process_property_value(
    _is_modular: bool,
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    let formatted_styles = Self::styles_formatter(false, &format!("{{ {:?}:{:?} }}", key, value));

    if formatted_styles.len() > 0 {
      let styles = format!(".{} {}", "galadriel_4414455", formatted_styles);

      return Ok((false, styles, "".to_string()));
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  fn processing_objects_by_type(
    is_modular: bool,
    is_nested: bool,
    is_children: bool,
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    if is_nested {
      Self::process_nested_objects(is_modular, key, &value)
    } else if is_children {
      Self::process_children_objects(is_modular, key, value)
    } else {
      Self::process_property_value(is_modular, key, value)
    }
  }

  pub fn process_objects(&self, input: HashMap<String, HashMap<String, HashMap<String, String>>>) {
    let is_modular = self.modular;

    for (_, val) in input.iter() {
      for (k, v) in val.iter() {
        if k == "main" {
          for (key, value) in v.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              if let Ok((is_media, styles, _class_name)) =
                Self::processing_objects_by_type(is_modular, true, false, key, value)
              {
                if !styles.is_empty() {
                  let styles_map: Result<HashMap<String, String>, _> = from_str(&styles.as_str());

                  if let Ok(map) = styles_map {
                    for _style in map.iter() {
                      

                      if is_media {
                        // adds into the media node
                        //println!("{:#?} --> {:#?}", key, style);
                      } else {
                        // adds into the property node
                        //println!("{:#?}", style);
                      }
                    }
                  }
                }
              }
            } else {
              if let Ok((_, styles, _class_name)) =
                Self::processing_objects_by_type(is_modular, false, false, key, value)
              {
                if !styles.is_empty() {
                  // adds into the property node
                  //println!("{:#?}", styles);
                }
              }
            }
          }
        } else if k == "children" {
          for (key, value) in v.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              if let Ok((is_media, styles, _class_name)) =
                Self::processing_objects_by_type(is_modular, false, true, key, value)
              {
                //println!("{:#?}", styles);

                if !styles.is_empty() {
                  if is_media {
                    // adds into the media node
                  } else {
                    // adds into the property node
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
