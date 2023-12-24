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

type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub struct Codelyzer {
  code: String,
  pub create_styles_map: std::collections::HashMap<String, String>,
}

impl Codelyzer {
  pub fn new(path: &str) -> Self {
    let blueprint = Blueprint::new();
    let file_content = file_reader(path);

    match file_content {
      Ok(content) => Codelyzer { code: content, create_styles_map: std::collections::HashMap::new() },
      Err(_) => {
        blueprint.error("something went wrong whiling processing a file".to_string());
        blueprint.info(format!("path not processed: {}", path).to_string());

        Codelyzer {
          code: "".to_string(), create_styles_map: std::collections::HashMap::new()
        }
      }
    }
  }

  fn escape_char(input: &str) -> ParserResult<&str> {
    alt((tag("\n"), tag("\t")))(input)
  }

  fn clear_space(input: &str) -> ParserResult<&str> {
    alt((tag(" "),))(input)
  }

  fn clear_parenthesis(input: &str) -> ParserResult<&str> {
    alt((tag("("), tag(")")))(input)
  }

  fn clear_arrow(input: &str) -> ParserResult<&str> {
    alt((tag("=>"),))(input)
  }

  fn clear_curly_braces(input: &str) -> ParserResult<&str> {
    alt((tag("{"), tag("}")))(input)
  }

  fn identifier_alpha_num(input: &str) -> ParserResult<&str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
  }

  fn clear_colon(input: &str) -> ParserResult<&str> {
    alt((tag(":"),))(input)
  }

  fn clear_semicolon(input: &str) -> ParserResult<&str> {
    alt((tag(";"),))(input)
  }

  fn clear_comma(input: &str) -> ParserResult<&str> {
    alt((tag(","),))(input)
  }

  fn clear_equals(input: &str) -> ParserResult<&str> {
    alt((tag("="),))(input)
  }

  fn clear_backtick(input: &str) -> ParserResult<&str> {
    alt((tag("`"),))(input)
  }

  fn identifier_double_quotes(input: &str) -> ParserResult<&str> {
    take_until("\"")(input)
  }

  fn identifier_single_quotes(input: &str) -> ParserResult<&str> {
    alt((take_until("\'"), take_until("'")))(input)
  }

  fn identifier_backtick(input: &str) -> ParserResult<&str> {
    take_until("`")(input)
  }

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

  fn removes_comment(input: &str) -> ParserResult<&str> {
    delimited(tag("//"), take_until("\n"), tag("\n"))(input)
  }

  fn save_collected_objects(main_obj: std::collections::HashMap<&str, String>, children_obj: std::collections::HashMap<&str, String>) {
    let mut final_map: std::collections::HashMap<&str, std::collections::HashMap<&str, String>> = std::collections::HashMap::new();

    final_map.insert("main", main_obj);
    final_map.insert("targetChildren", children_obj);

    println!("{:#?}", final_map);
  }

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
        Self::removes_comment,
        Self::clear_semicolon,
        Self::clear_equals,
      )),
      alt((Self::identifier_alpha_num, Self::identifier_string)),
    ))(input)
  }

  fn parser_create_styles(input: &str) -> ParserResult<&str> {
    let mut code = input;

    let mut is_key_env = false;
    let mut is_nested_key_env = false;
    let mut processing_current_nested = false;
    let mut is_target_children_env = false;

    let mut parenthesis_count = 0;
    let mut curly_braces_count = 0;

    let mut key = "";
    let mut nested_key = "";

    let mut map: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    let mut nested_map: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    let mut children_map: std::collections::HashMap<&str, String> = std::collections::HashMap::new();

    while let Ok((rest, result)) = Self::process_create_styles(code) {
      if rest.starts_with("(") {
        parenthesis_count += 1;
      } else if rest.starts_with(")") {
        parenthesis_count -= 1;

        if parenthesis_count == 0 {
          code = rest;
          break;
        }
      }

      if result == "targetChildren" {
        is_target_children_env = true;
      } else if rest.starts_with("{") && is_target_children_env {
        curly_braces_count += 1;
      } else if rest.starts_with("}") && is_target_children_env {
        curly_braces_count -= 1;

        if curly_braces_count == 0 {
          is_target_children_env = false;
        }
      }

      if rest.starts_with(":") && !is_key_env {
        is_key_env = true;
        key = result;
      } else if rest.starts_with("{") && is_key_env {
        is_nested_key_env = true;
      } else if rest.starts_with("}") && is_key_env {
        if is_target_children_env {
          children_map.insert(key, format!("{:?}", nested_map).to_string());
        } else {
          map.insert(key, format!("{:?}", nested_map).to_string());
        }

        is_key_env = false;
        is_nested_key_env = false;
        processing_current_nested = false;
        nested_map.clear();
      } else if is_key_env && result != ":" && !result.trim().is_empty() && !is_nested_key_env {
        if is_target_children_env {
          children_map.insert(key, result.to_string());
        } else {
          map.insert(key, result.to_string());
        }

        is_key_env = false;
      } else if rest.starts_with(":") && is_key_env && !processing_current_nested {
        nested_key = result;
        processing_current_nested = true;
      } else if is_nested_key_env && result != ":" && !result.trim().is_empty() && is_key_env && processing_current_nested {
        nested_map.insert(nested_key, result.to_string());
        processing_current_nested = false;
      }

      code = rest;
    }

    Self::save_collected_objects(map, children_map);

    Ok((code, ""))
  }

  fn clear_special(input: &str) -> ParserResult<&str> {
    alt((
      Self::clear_parenthesis,
      Self::clear_curly_braces,
      Self::clear_colon,
      Self::clear_semicolon,
      Self::clear_comma,
      Self::clear_equals,
      Self::removes_comment,
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

  fn process_tokens(input: &str) -> ParserResult<&str> {
    alt((
      Self::clear_space,
      Self::escape_char,
      Self::clear_special,
      Self::identifier_alpha_num,
    ))(input)
  }

  pub fn parser_code(&mut self) -> ParserResult<&str> {
    let mut input = self.code.as_str();

    while let Ok((rest, _)) = Self::process_tokens(input) {
      if rest.starts_with("createStyles") {
        if let Ok((r, _)) = Self::parser_create_styles(rest) {
          input = r;
        } else {
          input = rest;
        }
      } else {
        input = rest;
      }
    }

    Ok(("", ""))
  }
}
