use crate::{types::g3css_elements::G3cssElements, Rule};

use super::build_ast_from_elements::build_ast_from_elements;

/// Builds a vector of G3CSS elements nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS elements nodes from.
///
/// # Returns
/// Vector of constructed G3CSS elements nodes.
pub fn build_node_from_elements(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssElements>> {
    // Initialize an empty vector to store the constructed elements nodes.
    let mut nodes = vec![];

    // Iterate over inner pairs and construct elements nodes based on their rules.
    for inner_pair in pair.into_inner() {
        // Attempt to build an elements node from the inner pair using `build_ast_from_elements`.
        if let Some(node) = build_ast_from_elements(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the vector of constructed G3CSS elements nodes.
    Some(nodes)
}
