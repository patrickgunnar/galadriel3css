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
}

impl Codelyzer {
  pub fn new(path: &str) -> Self {
    let blueprint = Blueprint::new();
    let file_content = file_reader(path);

    match file_content {
      Ok(content) => Codelyzer { code: content },
      Err(_) => {
        blueprint.error("something went wrong whiling processing a file".to_string());
        blueprint.info(format!("path not processed: {}", path).to_string());

        Codelyzer {
          code: "".to_string(),
        }
      }
    }
  }

  fn simple_char(input: &str) -> ParserResult<&str> {
    alt((
      alt((tag("a"), tag("b"), tag("c"), tag("d"), tag("e"))),
      alt((tag("f"), tag("g"), tag("h"), tag("i"), tag("j"))),
      alt((tag("k"), tag("l"), tag("m"), tag("n"), tag("o"))),
      alt((tag("p"), tag("q"), tag("r"), tag("s"), tag("t"))),
      alt((tag("u"), tag("v"), tag("w"), tag("x"), tag("y"), tag("z"))),
      alt((tag("A"), tag("B"), tag("C"), tag("D"), tag("E"))),
      alt((tag("F"), tag("G"), tag("H"), tag("I"), tag("J"))),
      alt((tag("K"), tag("L"), tag("M"), tag("N"), tag("O"))),
      alt((tag("P"), tag("Q"), tag("R"), tag("S"), tag("T"))),
      alt((tag("U"), tag("V"), tag("W"), tag("X"), tag("Y"), tag("Z"))),
      alt((tag("0"), tag("1"), tag("2"), tag("3"), tag("4"))),
      alt((tag("5"), tag("6"), tag("7"), tag("8"), tag("9"))),
      alt((tag("á"), tag("à"), tag("â"), tag("ã"), tag("ä"))),
      alt((tag("é"), tag("è"), tag("ê"), tag("ë"), tag("í"))),
      alt((tag("ì"), tag("î"), tag("ï"), tag("ó"), tag("ò"))),
      alt((tag("ô"), tag("õ"), tag("ö"), tag("ú"), tag("ù"))),
      alt((tag("û"), tag("ü"), tag("ç"), tag("Á"), tag("À"))),
      alt((tag("Â"), tag("Ã"), tag("Ä"), tag("É"), tag("È"))),
      alt((tag("Ê"), tag("Ë"), tag("Í"), tag("Ì"), tag("Î"))),
      alt((tag("Ï"), tag("Ó"), tag("Ò"), tag("Ô"), tag("Õ"))),
      alt((tag("Ö"), tag("Ú"), tag("Ù"), tag("Û"), tag("Ü"), tag("Ç"))),
    ))(input)
  }

  fn special_char(input: &str) -> ParserResult<&str> {
    alt((
      alt((tag("!"), tag("@"), tag("#"), tag("$"), tag("%"))),
      alt((tag("¨"), tag("&"), tag("*"), tag("("), tag(")"))),
      alt((tag("_"), tag("+"), tag("-"), tag("\\"), tag("/"))),
      alt((tag("|"), tag(";"), tag(":"), tag("."), tag(">"))),
      alt((tag("<"), tag(","), tag("~"), tag("^"), tag("?"))),
      alt((tag("="), tag("'"), tag("\""), tag("¹"), tag("´"))),
      alt((tag("²"), tag("³"), tag("£"), tag("¢"), tag("¬"))),
      alt((tag("ª"), tag("º"), tag("`"), tag("["), tag("]"))),
      alt((tag("{"), tag("}"), tag("§"), tag("©"), tag("®"))),
      alt((tag("™"), tag("€"), tag("¥"), tag("∞"), tag("≠"))),
    ))(input)
  }

  fn process_tokens(input: &str) -> ParserResult<&str> {
    alt((
      Self::clear_space,
      Self::escape_char,
      Self::special_char,
      Self::simple_char,
    ))(input)
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

  fn clear_comma(input: &str) -> ParserResult<&str> {
    alt((tag(","),))(input)
  }

  /*fn clear_backtick(input: &str) -> ParserResult<&str> {
    alt((tag("`"),))(input)
  }*/

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

  fn process_create_styles(input: &str) -> ParserResult<&str> {
    alt((
      alt((
        tag("createStyles"),
        Self::clear_space,
        Self::clear_parenthesis,
        Self::clear_arrow,
        Self::escape_char,
        Self::clear_colon,
        Self::clear_comma,
        Self::clear_curly_braces,
      )),
      alt((Self::identifier_alpha_num, Self::identifier_string)),
    ))(input)
  }

  fn parser_create_styles(input: &str) -> ParserResult<&str> {
    let mut code = input;

    let mut is_key_env = false;
    let mut is_nested_key_env = false;

    let mut key = "";
    let mut nested_key = "";

    let mut map = std::collections::HashMap::new();
    let mut nested_map = std::collections::HashMap::new();

    while let Ok((rest, result)) = Self::process_create_styles(code) {
      //println!("Rest -> {:#?}", rest);
      //println!("Result -> {:#?}", result);

      if rest.starts_with(":") && !is_key_env {
        is_key_env = true;
        key = result;
      } else if rest.starts_with("{") && is_key_env {
        is_nested_key_env = true;
      } else if rest.starts_with("}") && is_key_env {
        map.insert(key, format!("{:?}", nested_map).to_string());
        is_key_env = false;
        is_nested_key_env = false;
        nested_map.clear();
      } else if is_key_env && result != ":" && result.len() > 1 && !is_nested_key_env {
        map.insert(key, result.to_string());
        is_key_env = false;
      } else if rest.starts_with(":") && is_key_env {
        nested_key = result;
      } else if is_nested_key_env && result != ":" && result.len() > 1 && is_key_env {
        nested_map.insert(nested_key, result.to_string());
      }

      if rest.starts_with("targetChildren") {
        code = "";
      } else {
        code = rest;
      }
    }

    println!("Properties -> \n{:#?}", map);

    Ok((code, ""))
  }

  pub fn parser_code(&self) -> ParserResult<&str> {
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
