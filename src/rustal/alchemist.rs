use crate::ast::stylitron::STYLITRON;
use crate::core::property_core::PROPERTY_CORE;
use crate::core::screen_core::SCREEN_CORE;
use crate::core::selector_core::SELECTOR_CORE;
use crate::rustal::blueprint::Blueprint;
use crate::rustal::classinator::classinator;

use linked_hash_map::LinkedHashMap;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_until, take_while1},
  sequence::delimited,
  IResult,
};
use serde_json::{from_str, Value};
use std::collections::HashMap;

// Public struct representing an Alchemist.
pub struct Alchemist {
  // A boolean flag indicating whether the configuration is modular.
  modular: bool,
}

/*
    - The Alchemist is responsible for the transformation of
    - objects into CSS styles, and also responsible to
    - append the generated CSS styles into the styles AST (STYLITRON),
    - and return an object containing the generated CSS class names.
*/
impl Alchemist {
  // Constructor method to create a new instance of Alchemist.
  // Parameters:
  // - modular: A boolean flag indicating whether the configuration is modular.
  // Returns: An instance of Alchemist.
  pub fn new(modular: bool) -> Self {
    // Create and return a new instance of Alchemist with the specified modular flag.
    Alchemist { modular }
  }

  // Function to log a warning when a handler is not processed.
  // Parameters:
  // - handler: A string representing the name of the handler.
  // - description: A string providing additional information on what the handler should include.
  fn log_handler_not_processed(handler: &str, description: &str) {
    let blueprint = Blueprint::new();

    blueprint.warn(blueprint.bold(format!("the '{}' handler was not processed", handler)));
    blueprint.warn(format!(
      "ensure the '{}' handler includes {}",
      handler, description
    ));
  }

  // Function to log an info when a handler is not processed.
  // Parameters:
  // - handler: A string representing the name of the handler.
  fn log_for_children_array_param(handler: &str) {
    Self::log_handler_not_processed(
      handler,
      "an array with HTML tag(s) as the first or second parameter in the handler",
    );

    let blueprint = Blueprint::new();

    blueprint.info(format!(
      "(example): add '{}(hover, ['div', ...], mobileScreen)' to the handler",
      if handler == "descendent" { "" } else { handler }
    ));
    blueprint.info(
      "the pseudo-selector (first parameter) and media query (third parameter) are optional"
        .to_string(),
    );
  }

  // Function to log an info when a handler is not processed.
  // Parameters:
  // - handler: A string representing the name of the handler.
  fn log_for_children_tag_param(handler: &str) {
    Self::log_handler_not_processed(handler, "an HTML tag as the first parameter in the handler");

    let blueprint = Blueprint::new();

    blueprint.info(format!(
      "(example): add '{}(div, mobileScreen)' to the handler",
      handler
    ));
    blueprint.info("the media query (second parameter) is optional".to_string());
  }

  // Function to log an info when a handler is not processed.
  // Parameters:
  // - handler: A string representing the name of the handler.
  fn log_for_children_attr_param(handler: &str) {
    Self::log_handler_not_processed(
        handler,
        "an HTML tag and an array containing the attribute(s) as the first and second parameters in the handler",
    );

    let blueprint = Blueprint::new();

    blueprint.info(format!(
      "(example): add '{}(div, ['attr', ...], mobileScreen)' to the handler",
      handler
    ));
    blueprint.info("the media query (third parameter) is optional".to_string());
  }

  // Function to extract a substring of length x from the middle of the input string.
  // Parameters:
  // - x: The desired length of the substring.
  // - input: A reference to the input string.
  // Returns: A String containing the extracted substring.
  fn get_middle_x(x: usize, input: &str) -> String {
    // Get the length of the input string
    let len = input.len();

    // Check if the length of the input string is greater than or equal to x
    if len >= x {
      // Calculate the start index of the substring
      let start = len / 2 - x / 2;
      // Calculate the end index of the substring
      let end = start + x;

      // Extract the substring from the input and convert it to a String
      input[start..end].to_string()
    } else {
      // If the length is less than x, return the entire input string
      input.to_string()
    }
  }

  // Function to generate a unique class name for styling elements dynamically.
  // Parameters:
  // - is_modular: Indicates whether the class should be modular based on the path.
  // - path: The path used to generate a modular class (if applicable).
  // - property: The CSS property associated with the class.
  // - value: The value of the CSS property.
  // - selector: Additional selector information for styling (if applicable).
  // Returns: A formatted unique class name string.
  fn generates_class_name(
    is_modular: bool,
    path: &str,
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

    format!(
      "galadriel_{}{}{}",
      Self::get_middle_x(6, &classinator(&format!("{}:{}", property, value)).as_str()),
      pseudo,
      modular
    )
  }

  // Function to format styles based on a JSON-like input.
  // Parameters:
  // - is_nested: Indicates whether the styles should be nested or not.
  // - input: JSON-like input string containing style information.
  // - incl_ternary: Indicates whether to include ternary expressions in the output.
  // Returns: A formatted string representing the styles.
  fn styles_formatter(is_nested: bool, input: &String, incl_ternary: bool) -> String {
    // Create a vector to store formatted style entries
    let mut map: Vec<String> = Vec::new();
    // Parse the input string into a JSON-like structure
    let content: Result<serde_json::Value, _> = from_str(&input);

    // Handle the parsed content
    match content {
      Ok(objects) => {
        // Check if the parsed content is an object
        if let Some(obj) = objects.as_object() {
          // Iterate through key-value pairs in the object
          for (k, v) in obj.iter() {
            // Get the corresponding property from the PROPERTY_CORE map
            let property = if let Some(v) = PROPERTY_CORE.get(k) {
              v.to_string()
            } else {
              "".to_string()
            };

            // Extract the value based on the JSON value type
            let value = match v {
              Value::String(s) => s,
              _ => "",
            };

            // Check if both property and value are non-empty
            if !property.is_empty() && !value.is_empty() {
              // Check if the value contains variables or should include ternary expressions
              if !value.contains("${") && !value.contains("}") || incl_ternary {
                // Add the formatted style entry to the vector
                map.push(format!("{}: {}", property, value));
              }
            }
          }
        }
      }
      Err(_) => {}
    }

    // Format the final styles string based on conditions
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

  // collects the first parameter from the targetChildren handler
  fn get_first_prop(input: &str) -> IResult<&str, &str> {
    alt((
      delimited(tag("("), Self::identifier_alpha_num, tag(",")),
      delimited(tag("("), Self::identifier_alpha_num, tag(")")),
    ))(input)
  }

  // collects the second parameter from the targetChildren handler
  fn get_second_prop(input: &str) -> IResult<&str, &str> {
    alt((
      delimited(tag("(["), take_until("])"), tag("])")),
      delimited(tag("["), take_until("]"), tag("]")),
    ))(input)
  }

  // collects the third parameter from the targetChildren handler
  fn get_third_prop(input: &str) -> IResult<&str, &str> {
    alt((Self::identifier_alpha_num,))(input)
  }

  // Function to collect properties from a controller string.
  // Parameters:
  // - controller: The input controller string.
  // Returns: Result containing a tuple with three components: first property, vector of second properties, and third property.
  //          The Result is Ok if the operation is successful, otherwise an Err with an error message.
  fn collects_properties(controller: String) -> Result<(String, Vec<String>, String), String> {
    // Convert the controller string to a string slice for parsing
    let mut input = controller.as_str();
    // Initialize variables to store the extracted properties
    let mut first_prop = "".to_string();
    let mut second_prop = "".to_string();
    let mut third_prop = "".to_string();

    // Parse the input using a parser combinators approach
    // Handle common separators and symbols in the input
    while let Ok((rest, result)) = alt((
      Self::get_first_prop,
      Self::get_second_prop,
      Self::get_third_prop,
      alt((tag("'"), tag("\""), tag(","), tag(" "), tag("("), tag(")"))),
    ))(input)
    {
      // Check if the result is not empty after trimming
      if !result.trim().is_empty() {
        // Check conditions to assign results to specific properties
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

      // Update the input for the next iteration
      input = rest;
    }

    // Handle special case where first_prop is "("
    let first_prop = if first_prop == "(" {
      "".to_string()
    } else {
      first_prop
    };

    // Split the second_prop into a vector of strings using ","
    let second_prop: Vec<String> = second_prop
      .split(",")
      .map(|s| String::from(s.trim()))
      .collect();

    // Return the result as a tuple in a Result
    Ok((first_prop, second_prop, third_prop))
  }

  // Function to process children handler for an array of tags.
  // Parameters:
  // - sep: Separator used to join tags into a string.
  // - tags: Vector of tags representing HTML elements.
  // - pseudo: Pseudo-selector associated with the handler.
  // - media: Media query associated with the handler.
  // - input: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn process_children_handler_for_array(
    sep: &str,
    tags: Vec<String>,
    pseudo: &String,
    media: &String,
    input: &String,
  ) -> Result<(bool, String, String), String> {
    // Join tags into a string using the provided separator
    let siblings = tags.join(sep);

    // Get the pseudo-selector from the SELECTOR_CORE map
    let pseudo_sel = if let Some(v) = SELECTOR_CORE.get(pseudo) {
      v.to_string()
    } else {
      "".to_string()
    };

    // Check if the media query is present in the SCREEN_CORE map
    let media = if let Some(_) = SCREEN_CORE.get(media) {
      media.to_string()
    } else {
      "".to_string()
    };

    // Determine if media query is present
    let is_media = !media.is_empty();
    // Call the styles_formatter function to process the CSS-like input string
    let styles = Self::styles_formatter(false, input, false);

    // Check if styles are not empty and construct the CSS class string
    if !styles.is_empty() {
      let css_cls = format!("{}{} {}", siblings, pseudo_sel, styles);

      // Return the result as a tuple in a Result
      return Ok((is_media, media, css_cls));
    }

    // Return a default result if styles are empty
    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to process children handler for a specific HTML tag.
  // Parameters:
  // - tag: HTML tag associated with the handler.
  // - pseudo: Pseudo-selector associated with the handler.
  // - media: Media query associated with the handler.
  // - input: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn process_children_handler_for_tag(
    tag: &String,
    pseudo: &str,
    media: &String,
    input: &String,
  ) -> Result<(bool, String, String), String> {
    // Check if the media query is present in the SCREEN_CORE map
    let media = if let Some(v) = SCREEN_CORE.get(media) {
      v.to_string()
    } else {
      "".to_string()
    };

    // Determine if media query is present
    let is_media = !media.is_empty();
    // Call the styles_formatter function to process the CSS-like input string
    let styles = Self::styles_formatter(false, input, false);

    // Check if styles are not empty and construct the CSS class string
    if !styles.is_empty() {
      let css_cls = format!("{}:{} {}", tag, pseudo, styles);

      // Return the result as a tuple in a Result
      return Ok((is_media, media, css_cls));
    }

    // Return a default result if styles are empty
    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to process children handler for an attribute of a specific HTML tag.
  // Parameters:
  // - sep: Separator used to join parts of the attribute selector.
  // - tag: HTML tag associated with the handler.
  // - typo: Attribute type or identifier.
  // - attr: Attribute value.
  // - media: Media query associated with the handler.
  // - input: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn process_children_handler_for_attr(
    sep: &str,
    tag: &String,
    typo: &str,
    attr: &String,
    media: &String,
    input: &String,
  ) -> Result<(bool, String, String), String> {
    // Check if the media query is present in the SCREEN_CORE map
    let media = if let Some(v) = SCREEN_CORE.get(media) {
      v.to_string()
    } else {
      "".to_string()
    };

    // Determine if media query is present
    let is_media = !media.is_empty();
    // Call the styles_formatter function to process the CSS-like input string
    let styles = Self::styles_formatter(false, input, false);

    // Check if styles are not empty and construct the CSS class string
    if !styles.is_empty() {
      let css_cls = format!(
        "{}[{}{}{}] {}",
        tag,
        typo,
        sep,
        format!("'{}'", attr),
        styles
      );

      // Return the result as a tuple in a Result
      return Ok((is_media, media, css_cls));
    }

    // Return a default result if styles are empty
    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to process children handler for a functional pseudo-class of a specific HTML tag.
  // Parameters:
  // - tag: HTML tag associated with the handler.
  // - pseudo: Functional pseudo-class associated with the handler.
  // - attr: Attribute value associated with the pseudo-class.
  // - media: Media query associated with the handler.
  // - input: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn process_children_handler_for_func(
    tag: &String,
    pseudo: &str,
    attr: &String,
    media: &String,
    input: &String,
  ) -> Result<(bool, String, String), String> {
    // Check if the media query is present in the SCREEN_CORE map
    let media = if let Some(v) = SCREEN_CORE.get(media) {
      v.to_string()
    } else {
      "".to_string()
    };

    // Determine if media query is present
    let is_media = !media.is_empty();
    // Call the styles_formatter function to process the CSS-like input string
    let styles = Self::styles_formatter(false, input, false);

    // Check if styles are not empty and construct the CSS class string
    if !styles.is_empty() {
      let css_cls = format!("{}:{}({}) {}", tag, pseudo, attr, styles);

      // Return the result as a tuple in a Result
      return Ok((is_media, media, css_cls));
    }

    // Return a default result if styles are empty
    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the next sibling of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn next_sibling(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((pseudo, tags, media)) = Self::collects_properties(controller) {
      // Check if there are tags and the first tag is not empty
      if tags.len() > 0 && !tags[0].is_empty() {
        // Process children handler for an array of tags with a next sibling selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_array(" + ", tags, &pseudo, &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_array_param("nextSibling");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the subsequent sibling of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn subseq_sibling(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((pseudo, tags, media)) = Self::collects_properties(controller) {
      // Check if there are tags and the first tag is not empty
      if tags.len() > 0 && !tags[0].is_empty() {
        // Process children handler for an array of tags with a subsequent sibling selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_array(" ~ ", tags, &pseudo, &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_array_param("subseqSibling");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the type of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn type_of(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and if the attribute at 0 position is present
      if !tag.is_empty() && !attr[0].is_empty() {
        // Process children handler for an attribute with a type selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_attr("=", &tag, "type", &attr[0], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("typeOf");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the direct children of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn direct_children(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((pseudo, tags, media)) = Self::collects_properties(controller) {
      // Check if there are tags and the first tag is not empty
      if tags.len() > 0 && !tags[0].is_empty() {
        // Process children handler for an array of tags with a direct children selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_array(" > ", tags, &pseudo, &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_array_param("directChildren");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the attribute starts with of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn attr_starts_with(
    controller: String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and the attribute length is greater than one
      // if the attribute at position 0 is not empty and if the attribute at position 1 is not empty
      if !tag.is_empty() && attr.len() > 1 && !attr[0].is_empty() && !attr[1].is_empty() {
        // Process children handler for an attribute with an attribute starts with selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_attr("^=", &tag, &attr[0], &attr[1], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("attrStartsWith");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the attribute contains of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn attr_contains(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and the attribute length is greater than one
      // if the attribute at position 0 is not empty and if the attribute at position 1 is not empty
      if !tag.is_empty() && attr.len() > 1 && !attr[0].is_empty() && !attr[1].is_empty() {
        // Process children handler for an attribute with an attribute contains selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_attr("*=", &tag, &attr[0], &attr[1], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("attrContains");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the attribute ends with of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn attr_ends_with(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and the attribute length is greater than one
      // if the attribute at position 0 is not empty and if the attribute at position 1 is not empty
      if !tag.is_empty() && attr.len() > 1 && !attr[0].is_empty() && !attr[1].is_empty() {
        // Process children handler for an attribute with an attribute ends with selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_attr("$=", &tag, &attr[0], &attr[1], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("attrEndsWith");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the nth-child of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn nth_child(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and the attribute at position 0 is not empty
      if !tag.is_empty() && !attr[0].is_empty() {
        // Process children handler for a function with a nth-child selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_func(&tag, "nth-child", &attr[0], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("nthChild");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the nth-of-type of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn nth_of_type(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and the attribute at position 0 is not empty
      if !tag.is_empty() && !attr[0].is_empty() {
        // Process children handler for a function with a nth-of-type selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_func(&tag, "nth-of-type", &attr[0], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("nthOfType");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the empty of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn empty(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with an empty selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "empty", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("empty");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the checked of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn checked(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with a checked selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "checked", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("checked");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the disabled of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn disabled(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with a disabled selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "disabled", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("disabled");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the focus of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn focus(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with a focus selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "focus", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("focus");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the active of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn active(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with an active selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "active", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("active");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the not of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn not(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, attr, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty and the attribute at position 0 is not empty
      if !tag.is_empty() && !attr[0].is_empty() {
        // Process children handler for a tag with a not selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_func(&tag, "not", &attr[0], &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_attr_param("not");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the visited of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn visited(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with a visited selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "visited", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("visited");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the last-child of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn last_child(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with a last-child selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "last-child", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("last-child");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the first-child of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn first_child(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if the tag is not empty
      if !tag.is_empty() {
        // Process children handler for a tag with a first-child selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_tag(&tag, "first-child", &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_tag_param("first-child");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for the descendent of an HTML element.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn descendent(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((pseudo, tags, media)) = Self::collects_properties(controller) {
      // Check if there are tags and the first tag is not empty
      if tags.len() > 0 && !tags[0].is_empty() {
        // Process children handler for an array of tags with a descendent selector
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_array(" ", tags, &pseudo, &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a message for an empty or invalid tag array
        Self::log_for_children_array_param("descendent");
      }
    }

    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to handle styling for a specific HTML tag and its children.
  // Parameters:
  // - controller: Controller string containing information about the styling.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn html_tag(controller: String, value: &String) -> Result<(bool, String, String), String> {
    // Attempt to collect properties from the controller string
    if let Ok((tag, _, media)) = Self::collects_properties(controller) {
      // Check if a valid HTML tag is present
      if !tag.is_empty() {
        // Create a vector with the extracted tag
        let mut tags = vec![];

        // stores only one tag in the vector
        tags.push(tag);

        // Process children handler for an array of tags
        if let Ok((is_media, media, css_cls)) =
          Self::process_children_handler_for_array("", tags, &"".to_string(), &media, value)
        {
          return Ok((is_media, media, css_cls));
        }
      } else {
        // Log a warning if the HTML tag is missing in the 'targetChildren' handler
        let blueprint = Blueprint::new();

        blueprint
          .warn(blueprint.bold("the 'targetChildren' handler is missing an HTML tag".to_string()));
        blueprint.info(
          "ensure to include the necessary HTML tag to process styles for the children".to_string(),
        );
      }
    }

    // Return a default result if there are no valid properties
    Ok((false, "".to_string(), "".to_string()))
  }

  // Function to process different types of children handlers based on the provided key.
  // Parameters:
  // - key: String indicating the type of children handler.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with three components: a boolean indicating if media is present,
  //          the media query string, and the generated CSS class string. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn process_children_objects(
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String), String> {
    // Check the key to determine the type of children handler and call the corresponding function
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

  // Function to process nested objects based on the provided key and value.
  // Parameters:
  // - key: String indicating the selector or media query.
  // - value: CSS-like input string containing style information.
  // Returns: Result containing a tuple with four components: a boolean indicating if it's a media environment,
  //          the selector string, pseudo-selector string, and a vector of styles. The Result is Ok if the operation is successful,
  //          otherwise an Err with an error message.
  fn process_nested_objects(
    key: &String,
    value: &String,
  ) -> Result<(bool, String, String, Vec<String>), String> {
    // Initialize variables
    let mut is_media_env = false;
    // Check if the key is in the SELECTOR_CORE map
    let selector = if let Some(v) = SELECTOR_CORE.get(key) {
      v.to_string()
    } else {
      // Check if the key is in the SCREEN_CORE map (indicating a media environment)
      if let Some(v) = SCREEN_CORE.get(key) {
        is_media_env = true;
        v.to_string()
      } else {
        "".to_string()
      }
    };

    // Check if the selector is not empty
    if !selector.is_empty() {
      // Initialize pseudo-selector based on the media environment
      let pseudo = if !is_media_env {
        selector.clone()
      } else {
        "".to_string()
      };

      // Process styles formatter and parse the JSON map
      let map = Self::styles_formatter(true, value, true);
      let json_map: Result<Vec<String>, _> = from_str(&map.as_str());

      // Check if the JSON map is successfully parsed
      if let Ok(styles_map) = json_map {
        return Ok((is_media_env, selector, pseudo, styles_map));
      }
    }

    // Return a default result if there are no valid styles
    Ok((false, "".to_string(), "".to_string(), vec![]))
  }

  // Function to process a property-value pair and generate CSS classes.
  // Parameters:
  // - is_modular: Boolean indicating if the styling is modular.
  // - path: String representing the path for modular styling.
  // - key: String representing the property key.
  // - value: String representing the property value.
  // Returns: Result containing a tuple with two components: the generated CSS class string and the class name.
  //          The Result is Ok if the operation is successful, otherwise an Err with an error message.
  fn process_property_value(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> Result<(String, String), String> {
    // Format the styles using the styles_formatter function
    let formatted_styles =
      Self::styles_formatter(false, &format!("{{ {:?}:{:?} }}", key, value), true);

      // Check if the formatted styles are not empty
    if formatted_styles.len() > 0 {
      // Generate a class name using generates_class_name function
      let cls_name = Self::generates_class_name(
        is_modular,
        path,
        key,
        value,
        &"".to_string(),
      );

      // Construct the CSS class string
      let css_cls = format!(".{} {}", cls_name, formatted_styles);

      // Return the result as a tuple in a Result
      return Ok((css_cls, cls_name));
    }

    // Return a default result if the formatted styles are empty
    Ok(("".to_string(), "".to_string()))
  }

  // Function to process a ternary operation in a CSS-like input string.
  // Parameters:
  // - input: CSS-like input string containing the ternary operation.
  // Returns: Result containing a tuple with two components: the first and second values of the ternary operation.
  //          The Result is Ok if the operation is successful, otherwise an Err with an error message.
  fn process_ternary(input: &String) -> Result<(String, String), String> {
    // Initialize variables
    let mut ternary = input.as_str();
    let mut is_cond = false;
    let mut first_value = String::new();
    let mut second_value = String::new();

    // Loop through the input string and process elements
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
      // Check if the result is the conditional "?" symbol
      if result == "?" {
        is_cond = true;
      } else if is_cond {
        // If in the conditional part, process values
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

    // Return the result as a tuple in a Result
    Ok((first_value, second_value))
  }

  // Function to trigger the append operation of nested styles.
  // Parameters:
  // - is_media: Boolean indicating if it's a media environment.
  // - is_modular: Boolean indicating if the styling is modular.
  // - path: String representing the path for modular styling.
  // - property: String representing the property key.
  // - value: String representing the property value.
  // - selector: String representing the selector.
  // - pseudo: String representing the pseudo-selector.
  // - rule: String representing the rule to be appended.
  // - key: String representing the key associated with the style.
  // Returns: Result containing a tuple with two components: a boolean indicating if the append operation is successful and
  //          the generated CSS class name. The Result is Ok if the operation is successful, otherwise an Err with an error message.
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
    // Generate a class name using generates_class_name function
    let cls_name = Self::generates_class_name(
      is_modular,
      path,
      property,
      value,
      selector,
    );

    // Construct the CSS class string
    let css_cls = format!(".{}{} {{ {} }}", cls_name, pseudo, rule);
    // Append the style rule to the Abstract Syntax Tree (AST)
    let is_in_ast = Self::append_to_ast(is_media, key, property, css_cls);

    // Return the result as a tuple in a Result
    Ok((is_in_ast, cls_name))
  }

  // Function to trigger the nested processing of styles.
  // Parameters:
  // - is_modular: Boolean indicating if the styling is modular.
  // - is_media: Boolean indicating if it's a media environment.
  // - path: String representing the path for modular styling.
  // - key: String representing the key associated with the style.
  // - selector: String representing the selector.
  // - pseudo: String representing the pseudo-selector.
  // - styles_map: Vector of strings representing styles.
  // Returns: HashMap containing the processed nested styles.
  fn trigger_nested_process(
    is_modular: bool,
    is_media: bool,
    path: &str,
    key: &String,
    selector: String,
    pseudo: String,
    styles_map: Vec<String>,
  ) -> HashMap<String, String> {
    // Initialize a HashMap to store the processed nested styles
    let mut intern_objects_map: HashMap<String, String> = HashMap::new();

    // Iterate over the styles_map
    for value_str in styles_map.iter() {
      // Check if the value string is not empty and contains a colon
      if value_str.len() > 0 && value_str.contains(":") {
        // Initialize a HashMap to store individual property-value pairs
        let mut intern_value_map: HashMap<String, String> = HashMap::new();
        // Split the value string into property and value
        let props: Vec<String> = value_str.split(":").map(|k| k.trim().to_string()).collect();

        let prop_key = &props[0];
        let prop_value = &props[1];

        // Check if the value string contains a ternary operation
        if value_str.contains("${") && value_str.contains("}") {
          // Process the ternary values
          if let Ok((first_value, second_value)) = Self::process_ternary(value_str) {
            // Iterate over the ternary values
            for ternary_v in &[first_value, second_value] {
              // Construct the rule for the property-value pair
              let rule = format!("{}: {}", prop_key, ternary_v);

              // Trigger the append of nested styles
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
                // If the append is successful, insert into the intern_value_map
                if is_in_ast {
                  intern_value_map.insert(ternary_v.to_string(), cls_name);
                }
              }
            }
          }
        } else {
          // If there's no ternary operation, trigger the append for the property-value pair
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
            // If the append is successful, insert into the intern_value_map
            if is_in_ast {
              intern_value_map.insert(prop_value.to_string(), cls_name);
            }
          }
        }

        // If there are values in intern_value_map, insert into intern_objects_map
        if intern_value_map.len() > 0 {
          intern_objects_map.insert(prop_key.to_string(), format!("{:?}", intern_value_map));
        }
      }
    }

    intern_objects_map
  }

  // Function to trigger the append operation for a property-value pair.
  // Parameters:
  // - is_modular: Boolean indicating if the styling is modular.
  // - path: String representing the path for modular styling.
  // - key: String representing the property key.
  // - value: String representing the property value.
  // Returns: Result containing a tuple with two components: a boolean indicating if the append operation is successful and
  //          the generated CSS class name. The Result is Ok if the operation is successful, otherwise an Err with an error message.
  fn trigger_append_of_property_value(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> Result<(bool, String), String> {
    // Process the property-value pair to generate CSS classes
    if let Ok((css_cls, cls_name)) = Self::process_property_value(is_modular, path, key, value) {
      // Check if the generated class name is not empty
      if !cls_name.is_empty() {
        // Append the style rule to the Abstract Syntax Tree (AST)
        let is_in_ast = Self::append_to_ast(false, &"".to_string(), key, css_cls);

        // Return the result as a tuple in a Result
        return Ok((is_in_ast, cls_name));
      }
    }

    // Return a default result if the processing fails
    Ok((false, "".to_string()))
  }

  // Function to trigger the processing of a property value.
  // Parameters:
  // - is_modular: Boolean indicating if the styling is modular.
  // - path: String representing the path for modular styling.
  // - key: String representing the property key.
  // - value: String representing the property value.
  // Returns: HashMap containing the processed property values and associated CSS class names.
  fn trigger_property_value_process(
    is_modular: bool,
    path: &str,
    key: &String,
    value: &String,
  ) -> HashMap<String, String> {
    // Initialize a HashMap to store the processed property values and associated CSS class names
    let mut intern_value_map: HashMap<String, String> = HashMap::new();

    // Check if the value contains a ternary operation
    if value.contains("${") && value.contains("}") {
      // Process the ternary values
      if let Ok((first_value, second_value)) = Self::process_ternary(value) {
        // Iterate over the ternary values
        for ternary_v in &[first_value, second_value] {
          // Trigger the append operation for the property value
          if let Ok((is_in_ast, cls_name)) =
            Self::trigger_append_of_property_value(is_modular, path, key, ternary_v)
          {
            // If the append is successful, insert into the intern_value_map
            if is_in_ast {
              intern_value_map.insert(ternary_v.to_string(), cls_name);
            }
          }
        }
      }
    } else {
      // If there's no ternary operation, trigger the append for the property value
      if let Ok((is_in_ast, cls_name)) =
        Self::trigger_append_of_property_value(is_modular, path, key, value)
      {
        // If the append is successful, insert into the intern_value_map
        if is_in_ast {
          intern_value_map.insert(value.to_string(), cls_name);
        }
      }
    }

    intern_value_map
  }

  // Function to trigger the processing of children objects.
  // Parameters:
  // - is_modular: Boolean indicating if the styling is modular.
  // - path: String representing the path for modular styling.
  // - identifier: String representing an identifier for the children objects.
  // - v: HashMap containing children objects and their associated CSS class names.
  // Returns: HashMap containing the processed CSS class map for the children objects.
  fn trigger_children_objects_process(
    is_modular: bool,
    path: &str,
    identifier: &String,
    v: &HashMap<String, String>,
  ) -> HashMap<String, String> {
    // Initialize a HashMap to store the processed CSS class map for the children objects
    let mut cls_map: HashMap<String, String> = HashMap::new();
    // Generate a unique CSS class name for the children objects
    let cls_name = format!(
      "children_{}{}",
      identifier,
      if is_modular {
        Self::get_middle_x(4, &classinator(path))
      } else {
        "".to_string()
      }
    );

    // Insert the CSS class name into the cls_map
    cls_map.insert("cls".to_string(), cls_name.clone());

    // Iterate over the children objects (key-value pairs) in the HashMap
    for (key, value) in v.iter() {
      // Check if the value is wrapped in curly braces indicating a block of styles
      if value.starts_with("{") && value.ends_with("}") {
        // Process the children objects and obtain media, selector, and CSS class
        if let Ok((is_media, selector, css_cls)) = Self::process_children_objects(key, value) {
          // Check if the generated CSS class is not empty
          if !css_cls.is_empty() {
            // Construct the complete CSS rule with the generated CSS class
            let cls = format!(".{} {}", cls_name, css_cls);

            // Append the style rule to the Abstract Syntax Tree (AST)
            Self::append_to_ast(is_media, &selector, &"targetChildren".to_string(), cls);
          }
        }
      }
    }

    // Return the processed CSS class map for the children objects
    cls_map
  }

  // Function to append a style rule to the Abstract Syntax Tree (AST).
  // Parameters:
  // - is_media: Boolean indicating if the style rule is within a media query.
  // - selector: String representing the selector for the style rule.
  // - attr: String representing an attribute or target for the style rule.
  // - css_cls: String representing the CSS class associated with the style rule.
  // Returns: Boolean indicating if the append operation is successful.
  fn append_to_ast(is_media: bool, selector: &String, attr: &String, css_cls: String) -> bool {
    // Lock the STYLITRON mutex for thread-safe access
    let mut stylitron = STYLITRON.lock().unwrap();
    // Flag to track if the property is not found in the AST
    let mut is_no_prop = false;

    // Iterate over the AST nodes
    for (key, node) in stylitron.iter_mut() {
      // Iterate over the properties within each node
      for (property, storage) in node.into_iter() {
        // Check if the style rule is within a media query and the selector is not empty
        if is_media && !selector.is_empty() {
          if key == "mediaQueryVariables" {
            // Check if the selector matches the property
            if selector == property {
              // If the CSS class is not already present, append it and return true
              if !storage.contains(&css_cls) {
                storage.push(css_cls.clone());

                return true;
              }

              // Set the flag if the property is found but the CSS class is already present
              is_no_prop = true;
            }
          }
          // Check if the style rule is not within a media query, the selector is not empty, and the attribute is not "targetChildren"
        } else if !is_media && !selector.is_empty() && attr != "targetChildren" {
          if key == "pseudoSelectors" {
            // Check if the selector matches the property
            if selector == property {
              // If the CSS class is not already present, append it and return true
              if !storage.contains(&css_cls) {
                storage.push(css_cls.clone());

                return true;
              }

              // Set the flag if the property is found but the CSS class is already present
              is_no_prop = true;
            }
          }
          // Check if the attribute is "targetChildren"
        } else if attr == "targetChildren" {
          if key == "targetChildren" {
            // Check if the property is "children"
            if property == "children" {
              // If the CSS class is not already present, append it and return true
              if !storage.contains(&css_cls) {
                storage.push(css_cls.clone());

                return true;
              }

              // Set the flag if the property is found but the CSS class is already present
              is_no_prop = true;
            }
          }
          // Check if the style rule is not within a media query and the selector is empty
        } else if !is_media && selector.is_empty() {
          // Check if the property matches the attribute
          if property == attr {
            // If the CSS class is not already present, append it and return true
            if !storage.contains(&css_cls) {
              storage.push(css_cls.clone());

              return true;
            }

            // Set the flag if the property is found but the CSS class is already present
            is_no_prop = true;
          }
        }
      }
    }

    // Check if the style rule is not within a media query, the selector is empty, and the attribute is not found
    if !is_media && !is_no_prop {
      // Insert the attribute and CSS class into "otherProperties" if not present
      stylitron
        .entry("otherProperties".to_string())
        .or_insert_with(|| LinkedHashMap::new())
        .entry(attr.to_string())
        .or_insert_with(|| Vec::new())
        .push(css_cls.clone());

      return true;
    }

    // Return false if the append operation is not successful
    false
  }

  // Public method to process a nested structure of style objects.
  // Parameters:
  // - self: A reference to the current instance of the struct.
  // - path: A string representing the path.
  // - input: A HashMap representing the input style objects.
  // Returns: A HashMap representing the processed style objects - (class names object).
  pub fn process_objects(
    &self,
    path: &str,
    input: HashMap<String, HashMap<String, HashMap<String, String>>>,
  ) -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    // Extract whether the styling is modular from the struct
    let is_modular = self.modular;
    // Initialize an empty HashMap to store the processed style objects - (class names object).
    let mut create_styles_map: HashMap<String, HashMap<String, HashMap<String, String>>> =
      HashMap::new();

    // Iterate over the input style objects
    for (identifier, val) in input.iter() {
      // Initialize an empty HashMap to store the processed style properties for each identifier - (class names of the styles).
      let mut objects_map: HashMap<String, HashMap<String, String>> = HashMap::new();

      // Iterate over the properties of each identifier
      for (k, v) in val.iter() {
        // Check if the property is "main"
        if k == "main" {
          // Iterate over the key-value pairs in the "main" property
          for (key, value) in v.iter() {
            // Check if the value is a nested object
            if value.starts_with("{") && value.ends_with("}") {
              // Process the nested object and obtain the styles map
              if let Ok((is_media, selector, pseudo, styles_map)) =
                Self::process_nested_objects(key, &value)
              {
                // Trigger the processing of nested styles and obtain the internal objects map
                let intern_objects_map = Self::trigger_nested_process(
                  is_modular, is_media, path, key, selector, pseudo, styles_map,
                );

                // If there are processed styles, insert them into the objects_map
                if intern_objects_map.len() > 0 {
                  objects_map.insert(key.to_string(), intern_objects_map);
                }
              }
            } else {
              // Process the property value and obtain the internal value map
              let intern_value_map =
                Self::trigger_property_value_process(is_modular, path, key, value);

              // If there are processed values, insert them into the objects_map
              if intern_value_map.len() > 0 {
                objects_map.insert(key.to_string(), intern_value_map);
              }
            }
          }
          // Check if the property is "children"
        } else if k == "children" {
          // Trigger the processing of children objects and obtain the class map
          let cls_map = Self::trigger_children_objects_process(is_modular, path, identifier, v);
          // Insert the class map into the objects_map with the key "targetChildren"
          objects_map.insert("targetChildren".to_string(), cls_map);
        }
      }

      // Insert the processed objects_map into the create_styles_map with the identifier as the key
      create_styles_map.insert(identifier.to_string(), objects_map);
    }

    // Return the final processed style objects - (class names object).
    create_styles_map
  }
}
