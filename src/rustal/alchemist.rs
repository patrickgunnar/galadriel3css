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

  fn next_sibling(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn subseq_sibling(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn type_of(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn direct_children(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn attr_starts_with(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn attr_contains(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn attr_ends_with(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn nth_child(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn nth_of_type(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn empty(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn checked(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn disabled(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn focus(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn active(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn not(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn visited(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn last_child(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn first_child(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn descendent(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn html_tag(
    _is_modular: bool,
    _property: &String,
    _value: String,
    _controller: String,
  ) -> Result<(bool, String), String> {
    Ok((false, "".to_string()))
  }

  fn generates_children_style(
    is_modular: bool,
    property: &String,
    value: String,
    controller: &str,
  ) -> Result<(bool, String), String> {
    if controller.starts_with("nextSibling") {
      Self::next_sibling(
        is_modular,
        property,
        value,
        controller.replace("nextSibling", ""),
      )
    } else if controller.starts_with("subseqSibling") {
      Self::subseq_sibling(
        is_modular,
        property,
        value,
        controller.replace("subseqSibling", ""),
      )
    } else if controller.starts_with("typeOf") {
      Self::type_of(
        is_modular,
        property,
        value,
        controller.replace("typeOf", ""),
      )
    } else if controller.starts_with("directChildren") {
      Self::direct_children(
        is_modular,
        property,
        value,
        controller.replace("directChildren", ""),
      )
    } else if controller.starts_with("attrStartsWith") {
      Self::attr_starts_with(
        is_modular,
        property,
        value,
        controller.replace("attrStartsWith", ""),
      )
    } else if controller.starts_with("attrContains") {
      Self::attr_contains(
        is_modular,
        property,
        value,
        controller.replace("attrContains", ""),
      )
    } else if controller.starts_with("attrEndsWith") {
      Self::attr_ends_with(
        is_modular,
        property,
        value,
        controller.replace("attrEndsWith", ""),
      )
    } else if controller.starts_with("nthChild") {
      Self::nth_child(
        is_modular,
        property,
        value,
        controller.replace("nthChild", ""),
      )
    } else if controller.starts_with("nthOfType") {
      Self::nth_of_type(
        is_modular,
        property,
        value,
        controller.replace("nthOfType", ""),
      )
    } else if controller.starts_with("empty") {
      Self::empty(is_modular, property, value, controller.replace("empty", ""))
    } else if controller.starts_with("checked") {
      Self::checked(
        is_modular,
        property,
        value,
        controller.replace("checked", ""),
      )
    } else if controller.starts_with("disabled") {
      Self::disabled(
        is_modular,
        property,
        value,
        controller.replace("disabled", ""),
      )
    } else if controller.starts_with("focus") {
      Self::focus(is_modular, property, value, controller.replace("focus", ""))
    } else if controller.starts_with("active") {
      Self::active(
        is_modular,
        property,
        value,
        controller.replace("active", ""),
      )
    } else if controller.starts_with("not") {
      Self::not(is_modular, property, value, controller.replace("not", ""))
    } else if controller.starts_with("visited") {
      Self::visited(
        is_modular,
        property,
        value,
        controller.replace("visited", ""),
      )
    } else if controller.starts_with("lastChild") {
      Self::last_child(
        is_modular,
        property,
        value,
        controller.replace("lastChild", ""),
      )
    } else if controller.starts_with("firstChild") {
      Self::first_child(
        is_modular,
        property,
        value,
        controller.replace("firstChild", ""),
      )
    } else if controller.starts_with("(") && controller.ends_with(")") {
      Self::descendent(is_modular, property, value, controller.to_string())
    } else {
      Self::html_tag(is_modular, property, value, controller.to_string())
    }
  }

  fn generates_nested_style(
    _is_modular: bool,
    property: &String,
    value: String,
    selector: &String,
  ) -> Result<(bool, String), String> {
    let mut is_media_env = false;
    let selector_value = if let Some(v) = SELECTOR_CORE.get(selector) {
      v.to_string()
    } else {
      if let Some(v) = SCREEN_CORE.get(selector) {
        is_media_env = true;
        v.to_string()
      } else {
        "".to_string()
      }
    };

    let property_value = if let Some(v) = PROPERTY_CORE.get(property) {
      v.to_string()
    } else {
      "".to_string()
    };

    if !selector_value.is_empty() && !property_value.is_empty() && !value.trim().is_empty() {
      let styles = format!(
        ".{}{} {{ {}: {} }}",
        "galadriel_4414455",
        if !is_media_env {
          selector_value
        } else {
          "".to_string()
        },
        property_value,
        value
      );

      return Ok((is_media_env, styles));
    }

    Ok((false, "".to_string()))
  }

  fn generates_property_style(
    _is_modular: bool,
    property: &String,
    value: String,
  ) -> Result<(bool, String), String> {
    let property_value = if let Some(v) = PROPERTY_CORE.get(property) {
      v.to_string()
    } else {
      "".to_string()
    };

    if !property_value.is_empty() && !value.trim().is_empty() {
      let styles = format!(
        ".{} {{ {}: {} }}",
        "galadriel_4414455", property_value, value
      );

      return Ok((false, styles));
    }

    Ok((false, "".to_string()))
  }

  fn processing_styles(
    is_modular: bool,
    is_nested: bool,
    is_children: bool,
    property: &String,
    value: String,
    selector: &String,
  ) -> Result<(bool, String), String> {
    if is_nested {
      Self::generates_nested_style(is_modular, property, value, selector)
    } else if is_children {
      Self::generates_children_style(is_modular, property, value, selector.trim())
    } else {
      Self::generates_property_style(is_modular, property, value)
    }
  }

  pub fn process_objects(&self, input: HashMap<String, HashMap<String, HashMap<String, String>>>) {
    let is_modular = self.modular;

    for (_, v) in input.iter() {
      for (k, val) in v.iter() {
        if k == "main" {
          for (key, value) in val.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              let nested_content: Result<serde_json::Value, _> = from_str(&value);

              match nested_content {
                Ok(objects) => {
                  if let Some(obj) = objects.as_object() {
                    for (nested_key, nested_v) in obj.iter() {
                      let nested_value = match nested_v {
                        Value::String(s) => s,
                        _ => "",
                      };

                      if let Ok((is_media, styles)) = Self::processing_styles(
                        is_modular,
                        true,
                        false,
                        nested_key,
                        nested_value.to_string(),
                        key,
                      ) {
                        if !styles.is_empty() {
                          //println!("{:#?}", styles);

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
                Err(_) => {}
              }
            } else {
              if let Ok((_, styles)) = Self::processing_styles(
                is_modular,
                false,
                false,
                key,
                value.to_string(),
                &"".to_string(),
              ) {
                if !styles.is_empty() {
                  // adds into the property node
                  println!("{:#?}", styles);
                }
              }
            }
          }
        } else if k == "children" {
          for (key, value) in val.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              let nested_content: Result<serde_json::Value, _> = from_str(&value);

              match nested_content {
                Ok(objects) => {
                  if let Some(obj) = objects.as_object() {
                    for (nested_key, nested_v) in obj.iter() {
                      let nested_value = match nested_v {
                        Value::String(s) => s,
                        _ => "",
                      };

                      if let Ok((is_media, styles)) = Self::processing_styles(
                        is_modular,
                        false,
                        true,
                        nested_key,
                        nested_value.to_string(),
                        key,
                      ) {
                        //println!("is_media ---> {:#?}", is_media);
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
                Err(_) => {}
              }
            }
          }
        }
      }
    }
  }
}
