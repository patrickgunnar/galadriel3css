use crate::rustal::blueprint::Blueprint;
use crate::rustal::file_reader::file_reader;

use nom::{
  branch::alt,
  bytes::complete::{tag, take_until, take_while1},
  character::complete::char,
  error::VerboseError,
  sequence::delimited,
  IResult,
};
use std::collections::HashMap;

type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub struct Codelyzer {
  code: String,
}

/*
  - Codelyzer is responsible for reading the content from a passed path and
  - collects the objects from the the createStyles' callback handler
*/
impl Codelyzer {
  pub fn new(path: &str) -> Self {
    let blueprint = Blueprint::new();
    // reads the file from the path
    let file_content = file_reader(path);

    match file_content {
      // if the file is read successfully
      // set the code property values to the file's content
      Ok(content) => Codelyzer { code: content },
      // if any errors are encountered
      // print some error messages and set the code to an empty string
      Err(_) => {
        blueprint.error("something went wrong whiling processing a file".to_string());
        blueprint.info(format!("path not processed: {}", path).to_string());

        Codelyzer {
          code: "".to_string(),
        }
      }
    }
  }

  // removes the break lines or tab from the string
  fn escape_char(input: &str) -> ParserResult<&str> {
    alt((tag("\n"), tag("\t")))(input)
  }

  // removes the spaces from the string
  fn clear_space(input: &str) -> ParserResult<&str> {
    alt((tag(" "),))(input)
  }

  // removes the opening or closing parenthesis from the string
  fn clear_parenthesis(input: &str) -> ParserResult<&str> {
    alt((tag("("), tag(")")))(input)
  }

  // removes the arrow function's arrow from the string
  fn clear_arrow(input: &str) -> ParserResult<&str> {
    alt((tag("=>"),))(input)
  }

  // removes the opening or closing curly bracket from the string
  fn clear_curly_braces(input: &str) -> ParserResult<&str> {
    alt((tag("{"), tag("}")))(input)
  }

  // removes the colon from the string
  fn clear_colon(input: &str) -> ParserResult<&str> {
    alt((tag(":"),))(input)
  }

  // removes the semicolon from the string
  fn clear_semicolon(input: &str) -> ParserResult<&str> {
    alt((tag(";"),))(input)
  }

  // removes the comma from the string
  fn clear_comma(input: &str) -> ParserResult<&str> {
    alt((tag(","),))(input)
  }

  // removes the equals sign from the string
  fn clear_equals(input: &str) -> ParserResult<&str> {
    alt((tag("="),))(input)
  }

  // removes the backtick from the string
  fn clear_backtick(input: &str) -> ParserResult<&str> {
    alt((tag("`"),))(input)
  }

  // identifies the alphanumeric sequence from the string
  fn identifier_alpha_num(input: &str) -> ParserResult<&str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
  }

  // identifies the double quotes from the string
  fn identifier_double_quotes(input: &str) -> ParserResult<&str> {
    take_until("\"")(input)
  }

  // identifies the single quote from the string
  fn identifier_single_quotes(input: &str) -> ParserResult<&str> {
    alt((take_until("\'"), take_until("'")))(input)
  }

  // identifies the backticks from the string
  fn identifier_backtick(input: &str) -> ParserResult<&str> {
    take_until("`")(input)
  }

  // identifies a string from the string
  // "GET THE CONTENT FROM HERE",
  // 'GET THE CONTENT FROM HERE' or
  // `GET THE CONTENT FROM HERE`
  fn identifier_string(input: &str) -> ParserResult<&str> {
    alt((
      delimited(char('"'), Self::identifier_double_quotes, char('"')),
      delimited(tag("\""), Self::identifier_double_quotes, tag("\"")),
      delimited(char('\''), Self::identifier_single_quotes, char('\'')),
      delimited(tag("'"), Self::identifier_single_quotes, tag("'")),
      delimited(char('`'), Self::identifier_backtick, char('`')),
      delimited(tag("`"), Self::identifier_backtick, tag("`")),
    ))(input)
  }

  // identifies the comment lines from the string
  fn identifier_comment(input: &str) -> ParserResult<&str> {
    delimited(tag("//"), take_until("\n"), tag("\n"))(input)
  }

  // process the string to remove non-objects content from
  // the createStyles' callback handler and collects the
  // key/value pairs inside from it
  fn process_create_styles(input: &str) -> ParserResult<&str> {
    alt((
      alt((
        tag("createStyles"),
        tag("targetChildren"),
        tag("function"),
        Self::clear_space,
        Self::clear_parenthesis,
        Self::clear_arrow,
        Self::escape_char,
        Self::clear_colon,
        Self::clear_comma,
        Self::clear_curly_braces,
        Self::identifier_comment,
        Self::clear_semicolon,
        Self::clear_equals,
      )),
      alt((Self::identifier_alpha_num, Self::identifier_string)),
    ))(input)
  }

  // the createStyles parser
  // it must collects the objects, nested objects and
  // targetChildren objects from createStyles' callback handler
  // than it must return a map with the objects and nested objects
  // and another map with the targetChildren objects
  fn parser_create_styles(
    input: &str,
  ) -> IResult<&str, (HashMap<String, String>, HashMap<String, String>)> {
    // the code to be processed
    let mut code = input;

    // conditional variable
    // conditional for the keys environment
    // conditional for the nested keys environment
    // conditional for the current object process
    // conditional for the target children environment
    let mut is_key_env = false;
    let mut is_nested_key_env = false;
    let mut processing_current_nested = false;
    let mut is_target_children_env = false;

    // state variables
    // holds the amount of parenthesis processed
    // holds the amount of braces processed
    let mut parenthesis_count = 0;
    let mut curly_braces_count = 0;

    // temporary  state variables
    // holds the current key value
    // holds the current nested key value
    let mut key = "";
    let mut nested_key = "";

    // maps to store the collected maps
    // stores the objects and nested objects as key:"{VALUES}"
    // temporary store the nested objects or nested objects
    // from target children to be later saved in map or children map
    // stores the target children objects
    let mut map: HashMap<String, String> = HashMap::new();
    let mut nested_map: HashMap<String, String> = HashMap::new();
    let mut children_map: HashMap<String, String> = HashMap::new();

    // loops until the end of the current createStyles statement
    // REST ---> the remaining of the current processing loop
    // RESULT ---> the collected part of the string
    while let Ok((rest, result)) = Self::process_create_styles(code) {
      // checks if the rest of the string starts with an opening parenthesis
      if rest.starts_with("(") {
        // sums 1 to the parenthesis control
        parenthesis_count += 1;
        // checks if the rest of the string starts with a closing parenthesis
      } else if rest.starts_with(")") {
        // subtracts 1 from the parenthesis control
        parenthesis_count -= 1;

        // if the parenthesis control reaches to 0
        // sets the rest of the string to the code variable
        // than breaks the loop and returns the rest of the string,
        // the map containing the objects and nested objects, and
        // the children map containing nested objects
        // from the targetChildren handler
        if parenthesis_count == 0 {
          code = rest;
          break;
        }
      }

      // if the current result returns the target children flag
      if result == "targetChildren" {
        // sets the target children environment to true
        is_target_children_env = true;
        // if the rest of the string starts with an opening braces
      } else if rest.starts_with("{") && is_target_children_env {
        // sums one to the braces control
        curly_braces_count += 1;
        // if the rest of the string starts with a closing braces
      } else if rest.starts_with("}") && is_target_children_env {
        // subtracts one from the braces control
        curly_braces_count -= 1;

        // if the braces control reaches 0
        // sets the target children environment control to false
        if curly_braces_count == 0 {
          is_target_children_env = false;
        }
      }

      // if the trimmed rest of the string starts with a colon
      // and the key environment is false
      if rest.trim().starts_with(":") && !is_key_env {
        // sets the key environment control to true
        is_key_env = true;
        // sets the key's value to the result's value
        key = result;
        // if the rest of the string starts with an opening bracket
        // and the key environment is true
      } else if rest.starts_with("{") && is_key_env {
        // set the nested environment control to true
        is_nested_key_env = true;
        // if the rest of the string starts with a closing bracket
        // and the key environment is true
      } else if rest.starts_with("}") && is_key_env {
        // if the trimmed nested key is not empty and
        // the result is not equals to colon and not equals to a comma
        // and the trimmed result is not empty and the nested environment control is true
        if !nested_key.trim().is_empty()
          && result != ":"
          && result != ","
          && !result.trim().is_empty()
          && is_nested_key_env
        {
          // insert the nested key and result (key:value pairs) into the nested map
          nested_map.insert(nested_key.to_string(), result.to_string());
          // if the trimmed key is not empty and
          // the result is not equals to colon and not equals to a comma
          // and the trimmed result is not empty and the nested environment control is false
        } else if !key.trim().is_empty()
          && result != ":"
          && result != ","
          && !result.trim().is_empty()
          && !is_nested_key_env
        {
          // insert the key and result (key:value pairs) into the map
          map.insert(key.to_string(), result.to_string());
        }

        // if the nested environment and target children environment are true
        if is_nested_key_env && is_target_children_env {
          // insert the key and the stringified nested map (key:value pairs) into the children's map
          children_map.insert(key.to_string(), format!("{:?}", nested_map).to_string());
          // if the nested environment only is true
        } else if is_nested_key_env {
          // insert the key and the stringified nested map (key:value pairs) into the map
          map.insert(key.to_string(), format!("{:?}", nested_map).to_string());
        }

        // resets the controls variable
        // resets the storage variables
        // clears the nested map
        is_key_env = false;
        is_nested_key_env = false;
        processing_current_nested = false;
        nested_key = "";
        key = "";
        nested_map.clear();
        // if the key environment is true and the result is not a colon
        // and the result is not comma and the trimmed key is not empty
        // and the trimmed result is not empty and the nested environment is false
      } else if is_key_env
        && result != ":"
        && result != ","
        && !key.trim().is_empty()
        && !result.trim().is_empty()
        && !is_nested_key_env
      {
        // if the target children environment is true
        if is_target_children_env {
          // if the result does not contain the function keyword
          // and the key does not contain the targetChildren flag
          if !result.contains("function") && !key.contains("targetChildren") {
            // insert the key and the result (key:value pairs) into the children's map
            children_map.insert(key.to_string(), result.to_string());
          }
        } else {
          // if the target children environment is false
          // insert the key and result (key:value pairs) into the map
          map.insert(key.to_string(), result.to_string());
        }

        // resets the key environment and storage
        is_key_env = false;
        key = "";
        // if the trimmed rest of the string starts with a colon
        // and the key environment is true and the nested environment is true
        // and the processing current nested environment is false
      } else if rest.trim().starts_with(":")
        && is_key_env
        && is_nested_key_env
        && !processing_current_nested
      {
        // sets the nested key storage to the current result's value
        nested_key = result;
        // sets the processing current nested environment to true
        processing_current_nested = true;
        // nested environment is true and the result is not a colon
        // and the result is not a comma and the trimmed nested key is not empty
        // and the trimmed result is not empty and the key environment is true
        // and the processing current nested environment is true
      } else if is_nested_key_env
        && result != ":"
        && result != ","
        && !nested_key.trim().is_empty()
        && !result.trim().is_empty()
        && is_key_env
        && processing_current_nested
      {
        // insert the nested key and result (key:value pairs) into the nested map
        nested_map.insert(nested_key.to_string(), result.to_string());
        // resets the processing current nested environment
        processing_current_nested = false;
        // resets the nested key
        nested_key = "";
      }

      // is the rest of the string starts with a comma
      if rest.starts_with(",") {
        // if the key environment is true
        // and the nested environment is false
        if is_key_env && !is_nested_key_env {
          // resets the key and the key environment
          key = "";
          is_key_env = false;
        } else {
          // if the processing environment is true (nested environment)
          // resets the nested key
          // and the processing current nested environment
          nested_key = "";
          processing_current_nested = false;
        }
      }

      // sets the code to the current rest of the string
      code = rest;
    }

    // returns the remaining code
    // the map containing the objects and nested objects
    // the children's map containing the nested objects
    Ok((code, (map, children_map)))
  }

  // removes special characters from the string
  fn clear_special(input: &str) -> ParserResult<&str> {
    alt((
      Self::clear_parenthesis,
      Self::clear_curly_braces,
      Self::clear_colon,
      Self::clear_semicolon,
      Self::clear_comma,
      Self::clear_equals,
      Self::identifier_comment,
      Self::clear_backtick,
      alt((tag("!"), tag("@"), tag("#"), tag("$"), tag("%"))),
      alt((tag("²"), tag("³"), tag("£"), tag("¢"), tag("¬"))),
      alt((tag("ª"), tag("º"), tag("."), tag("["), tag("]"))),
      alt((tag("_"), tag("+"), tag("-"), tag("\\"), tag("/"))),
      alt((tag("™"), tag("€"), tag("¥"), tag("∞"), tag("≠"))),
      alt((tag("<"), tag("~"), tag("^"), tag("?"), tag("®"))),
      alt((tag("'"), tag("\""), tag("¹"), tag("´"), tag("©"))),
      alt((tag("¨"), tag("&"), tag("*"), tag("§"), tag(">"), tag("|"))),
    ))(input)
  }

  // process the content from the script string
  fn process_tokens(input: &str) -> ParserResult<&str> {
    alt((
      Self::clear_space,
      Self::escape_char,
      Self::clear_special,
      Self::identifier_alpha_num,
    ))(input)
  }

  // parses the code from a string
  pub fn parser_code(
    &mut self,
  ) -> IResult<&str, HashMap<String, HashMap<String, HashMap<String, String>>>> {
    // transform the code String into a reference of string
    let mut input = self.code.as_str();
    // the map containing the collected objects
    // from the scope of the createStyles' callback handler
    let mut create_styles_map: HashMap<String, HashMap<String, HashMap<String, String>>> =
      HashMap::new();

    // loops to process the entire script string
    // until all the createStyles statements have been processed
    while let Ok((rest, _)) = Self::process_tokens(input) {
      // if the rest of the string starts with the createStyles flag
      if rest.starts_with("createStyles") {
        // parsers the createStyles handler and
        // receives the rest of the processed string and the objects' map
        // and the targetChildren's objects
        if let Ok((r, (main, children))) = Self::parser_create_styles(rest) {
          // re-declare the main map into a mutable one
          let mut main: HashMap<String, String> = main;
          // instantiate the map to holds the received objects' map and targetChildren's objects
          let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
          // collects the identifier from main map
          let identifier = match main.remove("identifier") {
            Some(id) => id,
            None => {
              let blueprint = Blueprint::new();

              blueprint.warn(blueprint.bold("the 'createStyles' function was not processed".to_string()));
              blueprint.warn("missing the 'identifier' property in the callback of 'createStyles'".to_string());
              blueprint.info("add the 'identifier' property to the return object of the callback in 'createStyles'".to_string());

              "".to_string()
            },
          };

          // if there is no identifier as property of the callback's return object
          if !identifier.is_empty() {
            // inserts the objects' map as the value of the "main" key in map
            map.insert("main".to_string(), main);
            // inserts the targetChildren objects' map as the value of the "children" key in map
            map.insert("children".to_string(), children);
            // inserts the current map into the create styles map
            create_styles_map.insert(identifier.to_string(), map);
          }

          // sets the input with the  remaining of the processed string
          input = r;
        } else {
          // if any error happens
          // sets the input with the remaining string
          input = rest;
        }
      } else {
        // if it does not starts with the createStyles flag
        // sets the input with the remaining string
        input = rest;
      }
    }

    // returns an empty string and the create styles map
    // containing all the collected objects from all the
    // create styles handlers that may exists in the
    // current script
    Ok(("", create_styles_map))
  }
}
