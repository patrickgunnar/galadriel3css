use nom::{
  branch::alt,
  bytes::complete::{tag, take_until, take_while1},
  error::VerboseError,
  sequence::delimited,
  IResult,
};

type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub struct Intaker;

/*
    - Intaker is responsible for collecting the import's path from a JS, JSX, TS or TSX file
*/
impl Intaker {
  pub fn new() -> Self {
    Intaker {}
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

  // identifies the comment lines from the string
  fn identifier_comment(input: &str) -> ParserResult<&str> {
    delimited(tag("//"), take_until("\n"), tag("\n"))(input)
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
      Self::clear_space,
      Self::escape_char,
      Self::clear_arrow,
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

  // identifies the alphanumeric sequence from the string
  fn identifier_alpha_num(input: &str) -> ParserResult<&str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
  }

  // identifies a string from the string
  fn identifier_string(input: &str) -> ParserResult<&str> {
    alt((
      delimited(tag("\""), take_until("\""), tag("\"")),
      delimited(tag("'"), take_until("'"), tag("'")),
    ))(input)
  }

  // current string processor collection
  fn process_collections(input: &str) -> ParserResult<&str> {
    alt((
      Self::identifier_string,
      tag("require"),
      tag("import"),
      tag("from"),
      Self::identifier_alpha_num,
      Self::clear_special,
    ))(input)
  }

  // process the current code to collects the import from it
  // and return an array containing the import's path
  pub fn process_code(&self, code: String) -> Vec<String> {
    // the code to be processed
    let mut input = code.as_str();
    // control variable to check if weather is inside a import or not
    let mut is_import = false;
    // the array containing the paths to be returned
    let mut imports: Vec<String> = vec![];

    // loops until exists content inside the string
    while let Ok((rest, result)) = Self::process_collections(input) {
      // if the returned result is import statement
      if result == "require" || result == "from" || result == "import" {
        // set the import environment to true
        is_import = true;
        // if the import environment is true
      } else if is_import {
        // checks if the result can be an import
        if result.starts_with("./")
          || result.starts_with("@/")
          || result.starts_with("../")
          || result.starts_with("/")
          || result.contains("/")
        {
          // push the path into the array
          imports.push(result.to_string());
          // reset the control variable
          is_import = false;
        }
      }

      // the input receives the rest of the string
      input = rest;
    }

    // return the array containing the collected paths
    imports
  }
}
