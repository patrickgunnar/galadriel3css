use crate::{types::g3css_theme::G3cssTheme, Rule};

use super::variable_handler::build_nodes_from_variables;

/// Builds a `G3cssTheme` AST node from a `Pair` of `Rule`.
///
/// # Arguments
///
/// - `pair` - A `Pair` of `Rule` representing a theme in the G3CSS language.
///
/// # Returns
///
/// Returns an `Option` containing a `G3cssTheme` node if the `pair` matches the `Rule::variables`,
/// or `None` if it does not match.
pub fn build_ast_from_theme(pair: pest::iterators::Pair<Rule>) -> Option<G3cssTheme> {
    match pair.as_rule() {
        // If the pair matches the `Rule::variables`, create a `G3cssTheme::Variables`
        // by building nodes from the variables pair, and wrap it in Some.
        Rule::variables => Some(G3cssTheme::Variables(build_nodes_from_variables(pair)?)),
        // If the pair does not match the `Rule::variables`, return None.
        _ => None,
    }
}

/// Builds a vector of `G3cssTheme` nodes from a `Pair` of `Rule`.
///
/// # Arguments
///
/// - `pair` - A `Pair` of `Rule` representing themes in the G3CSS language.
///
/// # Returns
///
/// Returns an `Option` containing a vector of `G3cssTheme` nodes, or `None` if no nodes could be built.
pub fn build_nodes_from_theme(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssTheme>> {
    // Create an empty vector to store the nodes.
    let mut nodes = vec![];

    // Iterate over the inner pairs of the given pair.
    for inner_pair in pair.into_inner() {
        // Attempt to build an AST node from the inner pair.
        // If successful, push the node to the vector.
        if let Some(node) = build_ast_from_theme(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the vector of nodes wrapped in Some.
    Some(nodes)
}
