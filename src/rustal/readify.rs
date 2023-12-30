use std::fs::File;
use std::io::Read;

/*
  - file_reader is responsible for reading the contents of a file
*/
pub fn readify(path: &str) -> Result<String, std::io::Error> {
  // open the path
  let mut file = File::open(path)?;
  // instantiate the content storage
  let mut content = String::new();

  // reads the contents of the file
  file.read_to_string(&mut content)?;

  // return the content as a String
  Ok(content)
}
