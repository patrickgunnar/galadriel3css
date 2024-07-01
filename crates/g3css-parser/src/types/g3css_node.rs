use super::g3css_children::G3cssChildren;

/// Enum representing nodes in the G3CSS abstract syntax tree (AST)
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssNode {
    Component(Vec<G3cssNode>),
    Global(Vec<G3cssNode>),
    Name(String),
    Extends(String),
    Children(Vec<G3cssChildren>),
}
