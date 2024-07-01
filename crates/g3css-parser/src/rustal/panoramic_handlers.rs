use crate::{
    types::{g3css_class::G3cssClass, g3css_panoramic::G3cssPanoramic},
    Rule,
};

use super::{class_handler::build_ast_from_class, utils::remove_whitespace};

/// Builds nodes representing breakpoint and children from a Pest `Pair`.
///
/// Constructs a vector of `G3cssPanoramic` nodes based on inner pairs of the provided `Pair`.
/// Returns `Some(Vec<G3cssPanoramic>)` with the constructed nodes, or `None` if
/// parsing or construction fails.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing the breakpoint and children.
///
/// # Returns
///
/// An `Option<Vec<G3cssPanoramic>>` containing the constructed vector of `G3cssPanoramic` nodes,
/// or `None` if parsing fails.
pub fn build_nodes_from_breakpoints(
    pair: pest::iterators::Pair<Rule>,
) -> Option<Vec<G3cssPanoramic>> {
    // Create an empty vector to hold nodes representing breakpoint and children
    let mut nodes = vec![];
    // Create an empty vector to hold children as G3cssClass
    let mut children: Vec<G3cssClass> = vec![];

    // Iterate over each inner pair within the provided pair
    for inner_pair in pair.into_inner() {
        // Match the rule of the inner pair to determine the type of node
        match inner_pair.as_rule() {
            // If the inner pair matches Rule::prime,
            // create a Breakpoint node and push it to nodes
            Rule::prime => {
                nodes.push(G3cssPanoramic::Breakpoint(remove_whitespace(
                    inner_pair.as_str(),
                )));
            }
            // For any other rule, attempt to build an AST node from the class
            _ => {
                if let Some(node) = build_ast_from_class(inner_pair) {
                    children.push(node);
                }
            }
        }
    }

    // Create a Children node containing the collected children and push it to nodes
    nodes.push(G3cssPanoramic::Children(children));
    // Return the constructed nodes wrapped in `Some`, indicating successful construction
    Some(nodes)
}

/// Builds nodes representing a panoramic viewer from a Pest `Pair`.
///
/// Constructs a vector of vectors of `G3cssPanoramic` nodes based on inner pairs of the provided `Pair`.
/// Returns `Some(Vec<Vec<G3cssPanoramic>>)` with the constructed nodes, or `None` if
/// parsing or construction fails.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing a panoramic viewer.
///
/// # Returns
///
/// An `Option<Vec<Vec<G3cssPanoramic>>>` containing the constructed vector of vectors of `G3cssPanoramic` nodes,
/// or `None` if parsing fails.
pub fn build_nodes_from_panoramic_viewer(
    pair: pest::iterators::Pair<Rule>,
) -> Option<Vec<Vec<G3cssPanoramic>>> {
    // Create an empty vector to hold nodes representing the panoramic viewer
    let mut nodes = vec![];

    // Iterate over each inner pair within the provided pair
    for inner_pair in pair.into_inner() {
        // Match the rule of the inner pair to determine the type of node
        match inner_pair.as_rule() {
            // If the inner pair matches Rule::breakpoint,
            // build nodes from breakpoints and push them to nodes
            Rule::breakpoint => {
                if let Some(node) = build_nodes_from_breakpoints(inner_pair) {
                    nodes.push(node);
                }
            }
            // Ignore other rules
            _ => (),
        }
    }

    // Return the constructed nodes wrapped in `Some`, indicating successful construction
    Some(nodes)
}
