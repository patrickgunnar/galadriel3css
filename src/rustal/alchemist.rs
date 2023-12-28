use crate::core::property_core::PROPERTY_CORE;
use crate::core::screen_core::SCREEN_CORE;
use crate::core::selector_core::SELECTOR_CORE;
use crate::rustal::blueprint::Blueprint;
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

  fn styles_formatter(is_nested: bool, input: &String, incl_ternary: bool) -> String {
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
              if !value.contains("${") && !value.contains("}") || incl_ternary {
                map.push(format!("{}: {}", property, value));
              }
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

  fn next_sibling(controller: String, value: &String) -> Result<(bool, String, String), String> {
    if let Ok((pseudo, tags, media)) = Self::collects_properties(controller) {
      if tags.len() > 0 && !tags[0].is_empty() {
        let siblings = tags.join(" ");

        let pseudo = if let Some(v) = SELECTOR_CORE.get(&pseudo) {
          v.to_string()
        } else {
          "".to_string()
        };

        let media = if let Some(v) = SCREEN_CORE.get(&media) {
          v.to_string()
        } else {
          "".to_string()
        };

        let is_media = if !media.is_empty() { true } else { false };
        let styles = Self::styles_formatter(false, value, false);

        if !styles.is_empty() {
          let css_cls = format!("{}{} {}", siblings, pseudo, styles);

          return Ok((is_media, media, css_cls));
        }
      } else {
        let blueprint = Blueprint::new();

        blueprint.warn(blueprint.bold("the 'nextSibling' handler was not processed".to_string()));
        blueprint
          .warn("ensure the 'nextSibling' handler includes an array with HTML tag(s)".to_string());
        blueprint.info(
          "place the array property as the first or second parameter in the handler".to_string(),
        );
        blueprint.info(
          "(example): add 'nextSibling(hover, ['div', ...], mobileScreen)' to the handler"
            .to_string(),
        );
        blueprint.info(
          "the pseudo-selector (first parameter) and media query (third parameter) are optional"
            .to_string(),
        );
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  fn subseq_sibling(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn type_of(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn direct_children(
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_starts_with(
    controller: String,
    _value: &String,
  ) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_contains(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn attr_ends_with(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn nth_child(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn nth_of_type(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn empty(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn checked(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn disabled(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn focus(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn active(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn not(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn visited(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn last_child(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn first_child(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn descendent(controller: String, _value: &String) -> Result<(bool, String, String), String> {
    let _props = Self::collects_properties(controller);

    Ok((false, "".to_string(), "".to_string()))
  }

  fn html_tag(_controller: String, _value: &String) -> Result<(bool, String, String), String> {
    Ok((false, "".to_string(), "".to_string()))
  }

  fn process_children_objects(
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    if key.starts_with("nextSibling") {
      Self::next_sibling(key.replace("nextSibling", ""), value)
    } else if key.starts_with("subseqSibling") {
      Self::subseq_sibling(key.replace("subseqSibling", ""), value)
    } else if key.starts_with("typeOf") {
      Self::type_of(key.replace("typeOf", ""), value)
    } else if key.starts_with("directChildren") {
      Self::direct_children(key.replace("directChildren", ""), value)
    } else if key.starts_with("attrStartsWith") {
      Self::attr_starts_with(key.replace("attrStartsWith", ""), value)
    } else if key.starts_with("attrContains") {
      Self::attr_contains(key.replace("attrContains", ""), value)
    } else if key.starts_with("attrEndsWith") {
      Self::attr_ends_with(key.replace("attrEndsWith", ""), value)
    } else if key.starts_with("nthChild") {
      Self::nth_child(key.replace("nthChild", ""), value)
    } else if key.starts_with("nthOfType") {
      Self::nth_of_type(key.replace("nthOfType", ""), value)
    } else if key.starts_with("empty") {
      Self::empty(key.replace("empty", ""), value)
    } else if key.starts_with("checked") {
      Self::checked(key.replace("checked", ""), value)
    } else if key.starts_with("disabled") {
      Self::disabled(key.replace("disabled", ""), value)
    } else if key.starts_with("focus") {
      Self::focus(key.replace("focus", ""), value)
    } else if key.starts_with("active") {
      Self::active(key.replace("active", ""), value)
    } else if key.starts_with("not") {
      Self::not(key.replace("not", ""), value)
    } else if key.starts_with("visited") {
      Self::visited(key.replace("visited", ""), value)
    } else if key.starts_with("lastChild") {
      Self::last_child(key.replace("lastChild", ""), value)
    } else if key.starts_with("firstChild") {
      Self::first_child(key.replace("firstChild", ""), value)
    } else if key.starts_with("(") && key.ends_with(")") {
      Self::descendent(key.to_string(), value)
    } else {
      Self::html_tag(key.to_string(), value)
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
      let pseudo = if !is_media_env {
        selector.clone()
      } else {
        "".to_string()
      };

      let map = Self::styles_formatter(true, value, true);
      let json_map: Result<Vec<String>, _> = from_str(&map.as_str());

      if let Ok(styles_map) = json_map {
        return Ok((is_media_env, selector, pseudo, styles_map));
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
    let formatted_styles =
      Self::styles_formatter(false, &format!("{{ {:?}:{:?} }}", key, value), true);

    if formatted_styles.len() > 0 {
      let cls_name = Self::generates_class_name(
        is_modular,
        path,
        false,
        &"".to_string(),
        key,
        value,
        &"".to_string(),
      );

      let css_cls = format!(".{} {}", cls_name, formatted_styles);

      return Ok((css_cls, cls_name));
    }

    Ok(("".to_string(), "".to_string()))
  }

  fn process_ternary(input: &String) -> Result<(String, String), String> {
    let mut ternary = input.as_str();
    let mut is_cond = false;
    let mut first_value = String::new();
    let mut second_value = String::new();

    while let Ok((rest, result)) = alt((
      delimited(tag("\""), take_until("\""), tag("\"")),
      delimited(tag("'"), take_until("'"), tag("'")),
      Self::identifier_alpha_num,
      tag(" "),
      tag("$"),
      tag("{"),
      tag("?"),
      tag(":"),
      tag("}"),
    ))(ternary)
    {
      if result == "?" {
        is_cond = true;
      } else if is_cond {
        if result != " " && result != ":" && result != "}" {
          if first_value.is_empty() {
            first_value = result.to_string();
          } else if second_value.is_empty() {
            second_value = result.to_string();
          }
        }
      }

      ternary = rest;
    }

    Ok((first_value, second_value))
  }

  fn trigger_append_of_nested(
    is_media: bool,
    is_modular: bool,
    path: &str,
    property: &String,
    value: &String,
    selector: &String,
    pseudo: String,
    rule: &String,
    key: &String,
  ) -> Result<(bool, String), String> {
    let cls_name = Self::generates_class_name(
      is_modular,
      path,
      false,
      &"".to_string(),
      property,
      value,
      selector,
    );

    let css_cls = format!(".{}{} {{ {} }}", cls_name, pseudo, rule);
    println!("{}", css_cls);
    let is_in_ast = Self::append_to_ast(is_media, key, property, css_cls);

    Ok((is_in_ast, cls_name))
  }

  fn trigger_nested_process(
    is_modular: bool,
    is_media: bool,
    path: &str,
    key: &String,
    selector: String,
    pseudo: String,
    styles_map: Vec<String>,
  ) -> HashMap<String, String> {
    let mut intern_objects_map: HashMap<String, String> = HashMap::new();

    for value_str in styles_map.iter() {
      if value_str.len() > 0 && value_str.contains(":") {
        let mut intern_value_map: HashMap<String, String> = HashMap::new();
        let props: Vec<String> = value_str.split(":").map(|k| k.trim().to_string()).collect();

        let prop_key = &props[0];
        let prop_value = &props[1];

        if value_str.contains("${") && value_str.contains("}") {
          if let Ok((first_value, second_value)) = Self::process_ternary(value_str) {
            for ternary_v in &[first_value, second_value] {
              let rule = format!("{}: {}", prop_key, ternary_v);

              if let Ok((is_in_ast, cls_name)) = Self::trigger_append_of_nested(
                is_media,
                is_modular,
                path,
                &prop_key,
                ternary_v,
                &selector,
                pseudo.clone(),
                &rule,
                key,
              ) {
                if is_in_ast {
                  intern_value_map.insert(ternary_v.to_string(), cls_name);
                }
              }
            }
          }
        } else {
          if let Ok((is_in_ast, cls_name)) = Self::trigger_append_of_nested(
            is_media,
            is_modular,
            path,
            &prop_key,
            &prop_value,
            &selector,
            pseudo.clone(),
            value_str,
            key,
          ) {
            if is_in_ast {
              intern_value_map.insert(prop_value.to_string(), cls_name);
            }
          }
        }

        if intern_value_map.len() > 0 {
          intern_objects_map.insert(prop_key.to_string(), format!("{:?}", intern_value_map));
        }
      }
    }

    intern_objects_map
  }

  fn trigger_append_of_property_value(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> Result<(bool, String), String> {
    if let Ok((css_cls, cls_name)) = Self::process_property_value(is_modular, path, key, value) {
      if !cls_name.is_empty() {
        let is_in_ast = Self::append_to_ast(false, &"".to_string(), key, css_cls);

        return Ok((is_in_ast, cls_name));
      }
    }

    Ok((false, "".to_string()))
  }

  fn trigger_property_value_process(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> HashMap<String, String> {
    let mut intern_value_map: HashMap<String, String> = HashMap::new();

    if value.contains("${") && value.contains("}") {
      if let Ok((first_value, second_value)) = Self::process_ternary(value) {
        for ternary_v in &[first_value, second_value] {
          if let Ok((is_in_ast, cls_name)) =
            Self::trigger_append_of_property_value(is_modular, path, key, ternary_v)
          {
            if is_in_ast {
              intern_value_map.insert(ternary_v.to_string(), cls_name);
            }
          }
        }
      }
    } else {
      if let Ok((is_in_ast, cls_name)) =
        Self::trigger_append_of_property_value(is_modular, path, key, value)
      {
        if is_in_ast {
          intern_value_map.insert(value.to_string(), cls_name);
        }
      }
    }

    intern_value_map
  }

  fn trigger_children_objects_process(
    is_modular: bool,
    path: &str,
    identifier: &String,
    v: &HashMap<String, String>,
  ) -> HashMap<String, String> {
    let mut cls_map: HashMap<String, String> = HashMap::new();
    let uniq_key = format!("targetChildren_{}", identifier);
    let uniq_value = format!("{:#?}", v);
    let uniqueness = format!("{}: {{ {}: {{ {} }} }}", identifier, uniq_key, uniq_value);

    let cls_name = Self::generates_class_name(
      is_modular,
      path,
      true,
      &uniqueness,
      &uniq_key,
      &uniq_value,
      &"".to_string(),
    );

    cls_map.insert("cls".to_string(), cls_name.clone());

    for (key, value) in v.iter() {
      if value.starts_with("{") && value.ends_with("}") {
        if let Ok((is_media, selector, css_cls)) = Self::process_children_objects(key, value) {
          if !css_cls.is_empty() {
            let cls = format!(".{} {}", cls_name, css_cls);
            println!("{}", cls);

            Self::append_to_ast(is_media, &selector, &"targetChildren".to_string(), cls);
          }
        }
      }
    }

    cls_map
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
  ) -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    let is_modular = self.modular;
    let mut create_styles_map: HashMap<String, HashMap<String, HashMap<String, String>>> =
      HashMap::new();

    for (identifier, val) in input.iter() {
      let mut objects_map: HashMap<String, HashMap<String, String>> = HashMap::new();

      for (k, v) in val.iter() {
        if k == "main" {
          for (key, value) in v.iter() {
            if value.starts_with("{") && value.ends_with("}") {
              if let Ok((is_media, selector, pseudo, styles_map)) =
                Self::process_nested_objects(key, &value)
              {
                let intern_objects_map = Self::trigger_nested_process(
                  is_modular, is_media, path, key, selector, pseudo, styles_map,
                );

                if intern_objects_map.len() > 0 {
                  objects_map.insert(key.to_string(), intern_objects_map);
                }
              }
            } else {
              let intern_value_map =
                Self::trigger_property_value_process(is_modular, path, key, value);

              if intern_value_map.len() > 0 {
                objects_map.insert(key.to_string(), intern_value_map);
              }
            }
          }
        } else if k == "children" {
          let cls_map = Self::trigger_children_objects_process(is_modular, path, identifier, v);
          objects_map.insert("targetChildren".to_string(), cls_map);
        }
      }

      create_styles_map.insert(identifier.to_string(), objects_map);
    }

    create_styles_map
  }
}
