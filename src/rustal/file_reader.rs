use std::fs::File;
use std::io::Read;

pub fn file_reader(path: &str) -> Result<String, std::io::Error> {
  let mut file = File::open(path)?;
  let mut content = String::new();

  file.read_to_string(&mut content)?;

  Ok(content)
}
