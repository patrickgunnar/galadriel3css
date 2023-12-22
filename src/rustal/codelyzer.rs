use crate::rustal::blueprint::Blueprint;
use crate::rustal::file_reader::file_reader;

use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

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

  fn escape_space(input: &str) -> ParserResult<&str> {
    alt((tag(" "),))(input)
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
    alt((Self::escape_space, Self::special_char, Self::simple_char))(input)
  }

  pub fn parser_code(&self) -> ParserResult<&str> {
    let mut input = self
      .code
      .replace("\n", "")
      .replace("\t", "")
      .replace("\r", "")
      .replace("\x0C", "")
      .replace("\x08", "");

    while let Ok((rest, _)) = Self::process_tokens(input.as_str()) {
      if rest.starts_with("createStyles") {
        println!("{:#?}", rest);
      } else {
        input = rest.to_string();
      }
    }

    Ok(("", ""))
  }
}
