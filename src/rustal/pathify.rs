// Function to format and normalize import paths based on the provided path and import.
// Parameters:
// - path: The base path used as a reference.
// - import: The import path to be formatted.
// Returns: A formatted and normalized import path.
pub fn pathify(path: &String, import: &String) -> String {
    // Check if the import starts with "../" or "./".
  if import.starts_with("../") || import.starts_with("./") {
    // Split the provided path into parts and filter out empty entries.
    let mut parts: Vec<&str> = path.split("/").filter(|entry| !entry.is_empty()).collect();

    // If the import starts with "../", move up the directory by removing parts.
    if import.starts_with("../") {
      let back_vec: Vec<&str> = import.matches("../").collect();

      for _ in back_vec.iter() {
        parts.pop();
      }
    }

    // Remove the last part (file or directory) from the path.
    parts.pop();

    // Replace "../" or "./" in the import with an empty string.
    let formatted_import = import.replace("../", "").replace("./", "");
    // Join the parts and the formatted import to create the final path.
    let mut formatted_path = format!("{}/{}", parts.join("/"), formatted_import);

    // Remove any leading '.' from the formatted path.
    formatted_path.remove(0);

    // Return the formatted path.
    formatted_path
  } else if import.starts_with("/@") || !import.starts_with("/") {
    // If the import starts with "/@" or doesn't start with "/", add a leading '/' and replace "@/".
    format!("/{}", import.replace("@/", ""))
  } else {
    // If none of the conditions apply, return the original import path.
    import.to_string()
  }
}
