use crate::core::property_core::PROPERTY_CORE;
use crate::core::screen_core::SCREEN_CORE;
use crate::core::selector_core::SELECTOR_CORE;
use crate::rustal::classinator::classinator;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_until, take_while1},
  sequence::delimited,
  IResult,
};
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

  fn get_middle_x(x: usize, input: &str) -> String {
    let len = input.len();

    if len >= 4 {
      let start = len / 2 - x / 2;
      let end = start + x;

      input[start..end].to_string()
    } else {
      input.to_string()
    }
  }

  fn generates_class_name(
    is_modular: bool,
    path: &str,
    is_unique: bool,
    uniqueness: &String,
    property: &String,
    value: &String,
    selector: &String,
  ) -> String {
    let modular = if is_modular {
      format!("{}", Self::get_middle_x(4, &classinator(path).as_str()))
    } else {
      "".to_string()
    };

    let pseudo = if selector.len() > 0 {
      format!("{}", Self::get_middle_x(4, &classinator(selector).as_str()))
    } else {
      "".to_string()
    };

    let unique = if is_unique {
      format!(
        "{}",
        Self::get_middle_x(4, &classinator(uniqueness).as_str())
      )
    } else {
      "".to_string()
    };

    format!(
      "galadriel_{}{}{}{}",
      Self::get_middle_x(6, &classinator(&format!("{}:{}", property, value)).as_str()),
      pseudo,
      unique,
      modular
    )
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

  // identifies the alphanumeric sequence from the string
  fn identifier_alpha_num(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
  }

  fn get_first_prop(input: &str) -> IResult<&str, &str> {
    alt((
      delimited(tag("("), Self::identifier_alpha_num, tag(",")),
      delimited(tag("("), Self::identifier_alpha_num, tag(")")),
    ))(input)
  }

  fn get_second_prop(input: &str) -> IResult<&str, &str> {
    alt((
      delimited(tag("(["), take_until("])"), tag("])")),
      delimited(tag("["), take_until("]"), tag("]")),
    ))(input)
  }

  fn get_third_prop(input: &str) -> IResult<&str, &str> {
    alt((Self::identifier_alpha_num,))(input)
  }

  fn collects_properties(controller: String) -> Result<(String, Vec<String>, String), String> {
    let mut input = controller.as_str();
    let mut first_prop = "".to_string();
    let mut second_prop = "".to_string();
    let mut third_prop = "".to_string();

    while let Ok((rest, result)) = alt((
      Self::get_first_prop,
      Self::get_second_prop,
      Self::get_third_prop,
      alt((tag("'"), tag("\""), tag(","), tag(" "), tag("("), tag(")"))),
    ))(input)
    {
      if !result.trim().is_empty() {
        if !result.contains(",")
          && !result.contains("'")
          && !result.contains("\"")
          && first_prop.is_empty()
        {
          first_prop = result.trim().to_string();
        } else if result.contains("'") || result.contains("\"") {
          second_prop = result.chars().filter(|&c| c != '"' && c != '\'').collect();
        } else if !first_prop.is_empty()
          && !result.contains(",")
          && !result.contains("'")
          && !result.contains("\"")
          && !result.contains("(")
          && !result.contains(")")
        {
          third_prop = result.trim().to_string();
        }
      }

      input = rest;
    }

    let first_prop = if first_prop == "(" {
      "".to_string()
    } else {
      first_prop
    };

    let second_prop: Vec<String> = second_prop
      .split(",")
      .map(|s| String::from(s.trim()))
      .collect();

    Ok((first_prop, second_prop, third_prop))
  }

  fn next_sibling(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn subseq_sibling(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn type_of(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn direct_children(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_starts_with(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_contains(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_ends_with(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn nth_child(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn nth_of_type(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn empty(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn checked(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn disabled(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn focus(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn active(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn not(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn visited(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn last_child(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn first_child(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn descendent(
    _is_modular: bool,
    _path: &str,
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn html_tag(
    _is_modular: bool,
    _path: &str,
    _controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn process_children_objects(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    if key.starts_with("nextSibling") {
      Self::next_sibling(is_modular, path, key.replace("nextSibling", ""), value)
    } else if key.starts_with("subseqSibling") {
      Self::subseq_sibling(is_modular, path, key.replace("subseqSibling", ""), value)
    } else if key.starts_with("typeOf") {
      Self::type_of(is_modular, path, key.replace("typeOf", ""), value)
    } else if key.starts_with("directChildren") {
      Self::direct_children(is_modular, path, key.replace("directChildren", ""), value)
    } else if key.starts_with("attrStartsWith") {
      Self::attr_starts_with(is_modular, path, key.replace("attrStartsWith", ""), value)
    } else if key.starts_with("attrContains") {
      Self::attr_contains(is_modular, path, key.replace("attrContains", ""), value)
    } else if key.starts_with("attrEndsWith") {
      Self::attr_ends_with(is_modular, path, key.replace("attrEndsWith", ""), value)
    } else if key.starts_with("nthChild") {
      Self::nth_child(is_modular, path, key.replace("nthChild", ""), value)
    } else if key.starts_with("nthOfType") {
      Self::nth_of_type(is_modular, path, key.replace("nthOfType", ""), value)
    } else if key.starts_with("empty") {
      Self::empty(is_modular, path, key.replace("empty", ""), value)
    } else if key.starts_with("checked") {
      Self::checked(is_modular, path, key.replace("checked", ""), value)
    } else if key.starts_with("disabled") {
      Self::disabled(is_modular, path, key.replace("disabled", ""), value)
    } else if key.starts_with("focus") {
      Self::focus(is_modular, path, key.replace("focus", ""), value)
    } else if key.starts_with("active") {
      Self::active(is_modular, path, key.replace("active", ""), value)
    } else if key.starts_with("not") {
      Self::not(is_modular, path, key.replace("not", ""), value)
    } else if key.starts_with("visited") {
      Self::visited(is_modular, path, key.replace("visited", ""), value)
    } else if key.starts_with("lastChild") {
      Self::last_child(is_modular, path, key.replace("lastChild", ""), value)
    } else if key.starts_with("firstChild") {
      Self::first_child(is_modular, path, key.replace("firstChild", ""), value)
    } else if key.starts_with("(") && key.ends_with(")") {
      Self::descendent(is_modular, path, key.to_string(), value)
    } else {
      Self::html_tag(is_modular, path, key.to_string(), value)
    }
  }

  fn process_nested_objects(
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String, Vec<String>), String> {
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
        selector.clone()
      } else {
        "".to_string()
      };

      let stringified_map = Self::styles_formatter(true, value);
      let json_map: Result<Vec<String>, _> = from_str(&stringified_map.as_str());

      if let Ok(styles_map) = json_map {
        return Ok((is_media_env, selector, pseudo_selector, styles_map));
      }
    }

    Ok((false, "".to_string(), "".to_string(), vec![]))
  }

  fn process_property_value(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> Result<(String, String), String> {
    let formatted_styles = Self::styles_formatter(false, &format!("{{ {:?}:{:?} }}", key, value));

    if formatted_styles.len() > 0 {
      let class_name = Self::generates_class_name(
        is_modular,
        path,
        false,
        &"".to_string(),
        key,
        value,
        &"".to_string(),
      );
      let styles = format!(".{} {}", class_name, formatted_styles);

      return Ok((styles, class_name));
    }

    Ok(("".to_string(), "".to_string()))
  }

  fn append_to_ast(_is_media: bool, _selector: &String, _key: &String, _style: String) -> bool {
    //println!("is_media: {}, selector: {}", is_media, selector);
    //println!("key: {}, style: {}\n", key, style);

    true
  }

  pub fn process_objects(
    &self,
    path: &str,
    input: HashMap<String, HashMap<String, HashMap<String, String>>>,
  ) -> HashMap<String, HashMap<String, String>> {
    let mut create_styles_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let is_modular = self.modular;

    for (identifier, val) in input.iter() {
      let mut class_name_map: HashMap<String, String> = HashMap::new();

      for (k, v) in val.iter() {
        if k == "main" {
          for (key, value) in v.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              let mut nested_class_name_map: HashMap<String, String> = HashMap::new();

              if let Ok((is_media, selector, pseudo_selector, styles_map)) =
                Self::process_nested_objects(key, &value)
              {
                for data in styles_map.iter() {
                  if data.len() > 0 && data.contains(":") {
                    let props: Vec<String> =
                      data.split(":").map(|k| k.trim().to_string()).collect();

                    let class_name = Self::generates_class_name(
                      is_modular,
                      path,
                      false,
                      &"".to_string(),
                      &props[0],
                      &props[1],
                      &selector,
                    );

                    let styles = format!(".{}{} {{ {} }}", class_name, pseudo_selector, data);
                    let inserted =
                      Self::append_to_ast(is_media, key, &props[0], styles.to_string());

                    if inserted {
                      nested_class_name_map
                        .insert(props[0].trim().to_string(), class_name.to_string());
                    }
                  }
                }

                if styles_map.len() > 0 {
                  class_name_map.insert(key.to_string(), format!("{:?}", nested_class_name_map));
                }
              }
            } else {
              if let Ok((styles, class_name)) =
                Self::process_property_value(is_modular, path, key, value)
              {
                if !styles.is_empty() {
                  let inserted = Self::append_to_ast(false, &"".to_string(), key, styles);

                  if inserted {
                    class_name_map.insert(key.to_string(), class_name);
                  }
                }
              }
            }
          }
        } else if k == "children" {
          let uniq_key = format!("targetChildren_{}", identifier);
          let uniq_value = format!("{:#?}", v);
          let uniqueness = format!("{}: {{ {}: {{ {} }} }}", identifier, uniq_key, uniq_value);

          let class_name = Self::generates_class_name(
            is_modular,
            path,
            true,
            &uniqueness,
            &uniq_key,
            &uniq_value,
            &"".to_string(),
          );

          class_name_map.insert("targetChildren".to_string(), class_name.clone());

          for (key, value) in v.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              if let Ok((_is_media, _selector, styles)) =
                Self::process_children_objects(is_modular, path, key, value)
              {
                if !styles.is_empty() {
                  /*Self::append_to_ast(
                    is_media,
                    &"".to_string(),
                    &"targetChildren".to_string(),
                    styles,
                  );*/
                }
              }
            }
          }
        }
      }

      create_styles_map.insert(identifier.to_string(), class_name_map);
    }

    create_styles_map
  }
}
