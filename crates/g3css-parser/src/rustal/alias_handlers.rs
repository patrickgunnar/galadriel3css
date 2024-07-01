use crate::{types::g3css_alias::G3cssAlias, Rule};

use super::utils::generates_string_vec;

/// Builds an AST node representing a G3css alias from a Pest `Pair`.
///
/// Constructs an AST node based on the rule of the provided `Pair`.
/// Returns `Some(G3cssAlias)` with the constructed alias, or `None` if
/// the rule does not match known aliases.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing an alias rule.
///
/// # Returns
///
/// An `Option<G3cssAlias>` containing the constructed AST node representing
/// an alias, or `None` if the rule does not match known aliases.
pub fn build_ast_from_alias(pair: pest::iterators::Pair<Rule>) -> Option<G3cssAlias> {
    match pair.as_rule() {
        // Construct an alias node for generic alias using a helper function
        Rule::alias => Some(G3cssAlias::Alias(generates_string_vec(pair)?)),
        // Return None for unrecognized rules
        _ => None,
    }
}

/// Builds nodes from aliases parsed by Pest.
///
/// Parses each inner pair from the provided `Pair`, constructs an AST node
/// using `build_ast_from_alias`, and collects valid nodes into a `Vec<G3cssAlias>`.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing parsed aliases.
///
/// # Returns
///
/// An `Option<Vec<G3cssAlias>>` containing parsed AST nodes representing aliases,
/// or `None` if parsing fails.
pub fn build_nodes_from_aliases(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssAlias>> {
    // Create an empty vector to hold the parsed nodes
    let mut nodes = vec![];

    // Iterate over each inner pair within the provided pair
    for inner_pair in pair.into_inner() {
        // Attempt to build an AST node from the inner pair
        if let Some(node) = build_ast_from_alias(inner_pair) {
            // If successful, push the node onto the vector
            nodes.push(node);
        }
    }

    // Return the vector of nodes wrapped in `Some`, indicating successful parsing
    Some(nodes)
}
