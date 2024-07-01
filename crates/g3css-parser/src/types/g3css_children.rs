use super::{
    g3css_alias::G3cssAlias, g3css_class::G3cssClass, g3css_theme::G3cssTheme,
    g3css_variable::G3cssVariable,
};

/// Enum representing different types of children elements in the G3CSS framework
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssChildren {
    LightTheme(Vec<G3cssTheme>),
    DarkTheme(Vec<G3cssTheme>),
    Aliases(Vec<G3cssAlias>),
    Variables(Vec<G3cssVariable>),
    Class(Vec<G3cssClass>),
    Classes(Vec<Vec<G3cssClass>>),
}
