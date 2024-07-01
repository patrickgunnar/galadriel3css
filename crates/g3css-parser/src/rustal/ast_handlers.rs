use crate::{types::g3css_node::G3cssNode, Rule};

use super::children_handler::build_node_from_children;

/// Builds a vector of G3CSS nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS nodes from.
///
/// # Returns
/// Vector of constructed G3CSS nodes.
pub fn build_ast_nodes(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssNode>> {
    // Initialize an empty vector to store the constructed nodes.
    let mut nodes = vec![];

    // Iterate over inner pairs and construct nodes based on their rules.
    for inner_pair in pair.into_inner() {
        // Attempt to build a node from the inner pair using `build_ast_from_rule`.
        if let Some(node) = build_ast_from_rule(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the vector of constructed G3CSS nodes.
    Some(nodes)
}

/// Builds a G3CSS node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS node.
///
/// # Returns
/// Option containing the constructed G3CSS node if matched, or None if the rule doesn't match.
pub fn build_ast_from_rule(pair: pest::iterators::Pair<Rule>) -> Option<G3cssNode> {
    match pair.as_rule() {
        // Iterate over inner pairs and build nodes from global rules.
        Rule::global => Some(G3cssNode::Global(build_ast_nodes(pair)?)),
        // Iterate over inner pairs and build nodes from component rules.
        Rule::component => Some(G3cssNode::Component(build_ast_nodes(pair)?)),
        // collects the value from component or global name.
        Rule::name => Some(G3cssNode::Name(pair.as_str().trim_matches('"').to_string())),
        // collects the extends value from the component.
        Rule::extends => Some(G3cssNode::Extends(
            pair.as_str().trim_matches('"').to_string(),
        )),
        Rule::component_children => {
            // Iterate over inner pairs and build nodes from children rules.
            Some(G3cssNode::Children(build_node_from_children(pair)?))
        }
        Rule::global_children => {
            // Iterate over inner pairs and build nodes from children rules.
            Some(G3cssNode::Children(build_node_from_children(pair)?))
        }
        _ => None,
    }
}
