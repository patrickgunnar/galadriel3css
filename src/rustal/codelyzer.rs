use crate::rustal::blueprint::Blueprint;
use crate::rustal::file_reader::file_reader;

use nom::{
  branch::alt,
  bytes::complete::{tag, take},
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

  fn get_function(input: &str) -> ParserResult<&str> {
    alt((tag("function "), tag("function")))(input)
  }

  fn function(input: &str) -> ParserResult<&str> {
    delimited(Self::get_function, take(10usize), char('('))(input)
  }

  pub fn parser_code(&self) -> ParserResult<&str> {
    Self::function(&self.code.as_str())
  }
}
