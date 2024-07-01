use super::g3css_class::G3cssClass;

/// Enum representing different panoramic viewers in the G3CSS framework
/// Breakpoint - represents the media rules (e.g., Mobile, Tablet, Laptop, Desktop)
/// Children - represents the properties of elements in the G3CSS framework
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssPanoramic {
    Breakpoint(String),
    Children(Vec<G3cssClass>),
}
