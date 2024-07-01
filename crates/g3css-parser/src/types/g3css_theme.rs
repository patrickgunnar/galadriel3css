use super::g3css_variable::G3cssVariable;

/// Enum representing a G3CSS theme.
/// Represents a theme with a vector of G3CSS variables.
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssTheme {
    Variables(Vec<G3cssVariable>),
}
