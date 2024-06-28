#[derive(PartialEq, Debug, Clone)]
pub enum G3cssChildren {
    Alias(String),
    Variable(String),
    Class(String),
    Classes(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum G3cssNode {
    Component(Vec<G3cssNode>),
    Global(Vec<G3cssNode>),
    ClassName(String),
    Extends(String),
    Children(Vec<G3cssChildren>),
}
