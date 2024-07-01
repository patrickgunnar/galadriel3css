use crate::{types::g3css_children::G3cssChildren, Rule};

use super::{
    alias_handlers::build_nodes_from_aliases,
    class_handler::{build_nodes_from_class, build_nodes_from_classes},
    theme_handlers::build_nodes_from_theme,
    variable_handler::build_nodes_from_variables,
};

/// Builds a G3CSS children node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS children node.
///
/// # Returns
/// Option containing the constructed G3CSS children node if matched, or None if the rule doesn't match.
pub fn build_ast_from_children(pair: pest::iterators::Pair<Rule>) -> Option<G3cssChildren> {
    match pair.as_rule() {
        // Collects the value from the aliases rule.
        Rule::aliases => Some(G3cssChildren::Aliases(build_nodes_from_aliases(pair)?)),
        // Collects the value from the variables rule.
        Rule::variables => Some(G3cssChildren::Variables(build_nodes_from_variables(pair)?)),
        // Collects the value from the light theme rule.
        Rule::light_theme => Some(G3cssChildren::LightTheme(build_nodes_from_theme(pair)?)),
        // Collects the value from the dark theme rule.
        Rule::dark_theme => Some(G3cssChildren::DarkTheme(build_nodes_from_theme(pair)?)),
        // Collects the value from the class rule.
        Rule::class => Some(G3cssChildren::Class(build_nodes_from_class(pair)?)),
        // Collects the value from the classes rule.
        Rule::classes => Some(G3cssChildren::Classes(build_nodes_from_classes(pair)?)),
        _ => None,
    }
}

/// Builds a vector of G3CSS children nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS children nodes from.
///
/// # Returns
/// Vector of constructed G3CSS children nodes.
pub fn build_node_from_children(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssChildren>> {
    // Initialize an empty vector to store the constructed children nodes.
    let mut nodes = vec![];

    // Iterate over inner pairs and construct children nodes based on their rules.
    for inner_pair in pair.into_inner() {
        // Attempt to build a children node from the inner pair using `build_ast_from_children`.
        if let Some(node) = build_ast_from_children(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the vector of constructed G3CSS children nodes.
    Some(nodes)
}
