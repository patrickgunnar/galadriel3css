/// Enum representing a G3CSS alias.
/// Represents an alias with a vector of strings.
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssAlias {
    Alias(Vec<String>),
}
