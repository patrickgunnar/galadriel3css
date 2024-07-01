/// Enum representing a G3CSS variable.
/// Represents a variable with a vector of strings.
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssVariable {
    Variable(Vec<String>),
}
