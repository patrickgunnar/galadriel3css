use crate::{types::g3css_variable::G3cssVariable, Rule};

use super::utils::generates_string_vec;

/// Builds a `G3cssVariable` AST node from a `Pair` of `Rule`.
///
/// # Arguments
///
/// - `pair` - A `Pair` of `Rule` representing a variable in the G3CSS language.
///
/// # Returns
///
/// Returns an `Option` containing a `G3cssVariable` node if the `pair` matches the `Rule::variable`,
/// or `None` if it does not match.
pub fn build_ast_from_variable(pair: pest::iterators::Pair<Rule>) -> Option<G3cssVariable> {
    match pair.as_rule() {
        // If the pair matches the `Rule::variable`, create a `G3cssVariable::Variable`
        // by generating a string vector from the pair, and wrap it in Some.
        Rule::variable => Some(G3cssVariable::Variable(generates_string_vec(pair)?)),
        // If the pair does not match the `Rule::variable`, return None.
        _ => None,
    }
}

/// Builds a vector of `G3cssVariable` nodes from a `Pair` of `Rule`.
///
/// # Arguments
///
/// - `pair` - A `Pair` of `Rule` representing variables in the G3CSS language.
///
/// # Returns
///
/// Returns an `Option` containing a vector of `G3cssVariable` nodes, or `None` if no nodes could be built.
pub fn build_nodes_from_variables(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssVariable>> {
    // Create an empty vector to store the nodes.
    let mut nodes = vec![];

    // Iterate over the inner pairs of the given pair.
    for inner_pair in pair.into_inner() {
        // Attempt to build an AST node from the inner pair.
        // If successful, push the node to the vector.
        if let Some(node) = build_ast_from_variable(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the vector of nodes wrapped in Some.
    Some(nodes)
}
