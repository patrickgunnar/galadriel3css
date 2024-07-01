use std::rc::Rc;

use g3css_parser::types::g3css_node::G3cssNode;

pub fn g3css_ast_setter(ast: Rc<G3cssNode>) {
    println!("{:#?}", ast);
}
