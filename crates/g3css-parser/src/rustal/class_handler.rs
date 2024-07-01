use crate::{types::g3css_class::G3cssClass, Rule};

use super::{
    elements_handler::build_node_from_elements,
    panoramic_handlers::build_nodes_from_panoramic_viewer,
};

/// Builds a G3CSS class node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS class node.
///
/// # Returns
/// Option containing the constructed G3CSS class node if matched, or None if the rule doesn't match.
pub fn build_ast_from_class(pair: pest::iterators::Pair<Rule>) -> Option<G3cssClass> {
    match pair.as_rule() {
        // Iterate over inner pairs and build nodes from panoramic viewer rule.
        Rule::panoramic_viewer => Some(G3cssClass::PanoramicViewer(
            build_nodes_from_panoramic_viewer(pair)?,
        )),
        // Collects the value from the class name rule.
        Rule::class_name => Some(G3cssClass::ClassName(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the inherits rule.
        Rule::inherits => Some(G3cssClass::Inherits(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the important rule.
        Rule::important => Some(G3cssClass::Important(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Iterate over inner pairs and build nodes from properties rules.
        Rule::properties => Some(G3cssClass::Properties(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from hover rules.
        Rule::hover => Some(G3cssClass::Hover(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from active rules.
        Rule::active => Some(G3cssClass::Active(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from focus rules.
        Rule::focus => Some(G3cssClass::Focus(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from first_child rules.
        Rule::first_child => Some(G3cssClass::FirstChild(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from last_child rules.
        Rule::last_child => Some(G3cssClass::LastChild(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from first_of_type rules.
        Rule::first_of_type => Some(G3cssClass::FirstOfType(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from last_of_type rules.
        Rule::last_of_type => Some(G3cssClass::LastOfType(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from only_child rules.
        Rule::only_child => Some(G3cssClass::OnlyChild(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from only_of_type rules.
        Rule::only_of_type => Some(G3cssClass::OnlyOfType(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from target_pseudo_class rules.
        Rule::target_pseudo_class => Some(G3cssClass::TargetPseudoClass(build_node_from_elements(
            pair,
        )?)),
        // Iterate over inner pairs and build nodes from visited rules.
        Rule::visited => Some(G3cssClass::Visited(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from checked rules.
        Rule::checked => Some(G3cssClass::Checked(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from disabled rules.
        Rule::disabled => Some(G3cssClass::Disabled(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from enabled rules.
        Rule::enabled => Some(G3cssClass::Enabled(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from read_only rules.
        Rule::read_only => Some(G3cssClass::ReadOnly(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from read_write rules.
        Rule::read_write => Some(G3cssClass::ReadWrite(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from placeholder_shown rules.
        Rule::placeholder_shown => Some(G3cssClass::PlaceholderShown(build_node_from_elements(
            pair,
        )?)),
        // Iterate over inner pairs and build nodes from valid rules.
        Rule::valid => Some(G3cssClass::Valid(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from invalid rules.
        Rule::invalid => Some(G3cssClass::Invalid(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from required rules.
        Rule::required => Some(G3cssClass::Required(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from optional rules.
        Rule::optional => Some(G3cssClass::Optional(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from fullscreen rules.
        Rule::fullscreen => Some(G3cssClass::Fullscreen(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from focus_within rules.
        Rule::focus_within => Some(G3cssClass::FocusWithin(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from first_line rules.
        Rule::first_line => Some(G3cssClass::FirstLine(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from first_letter rules.
        Rule::first_letter => Some(G3cssClass::FirstLetter(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from before rules.
        Rule::before => Some(G3cssClass::Before(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from after rules.
        Rule::after => Some(G3cssClass::After(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from out_of_range rules.
        Rule::out_of_range => Some(G3cssClass::OutOfRange(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from root rules.
        Rule::root => Some(G3cssClass::Root(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from first_page rules.
        Rule::first_page => Some(G3cssClass::FirstPage(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from left_page rules.
        Rule::left_page => Some(G3cssClass::LeftPage(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from right_page rules.
        Rule::right_page => Some(G3cssClass::RightPage(build_node_from_elements(pair)?)),
        // Iterate over inner pairs and build nodes from empty rules.
        Rule::empty => Some(G3cssClass::Empty(build_node_from_elements(pair)?)),
        _ => None,
    }
}

/// Builds a vector of G3CSS class nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS class nodes from.
///
/// # Returns
/// Vector of constructed G3CSS class nodes.
pub fn build_nodes_from_class(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssClass>> {
    // Initialize an empty vector to store the constructed class nodes.
    let mut nodes = vec![];

    // Iterate over inner pairs and construct class nodes based on their rules.
    for inner_pair in pair.into_inner() {
        // Attempt to build a class node from the inner pair using `build_ast_from_class`.
        if let Some(node) = build_ast_from_class(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the vector of constructed G3CSS class nodes.
    Some(nodes)
}

/// Builds a vector of `G3cssClass` nodes from a given `Pair` of `Rule`.
///
/// # Arguments
/// - `pair` - A `pest::iterators::Pair` representing the parsed input for the classes.
///
/// # Returns
/// - `Option<Vec<Vec<G3cssClass>>>` - An optional nested vector of `G3cssClass` nodes.
///   Returns `Some` containing the nested vector if nodes are successfully built,
///   otherwise returns `None`.
pub fn build_nodes_from_classes(pair: pest::iterators::Pair<Rule>) -> Option<Vec<Vec<G3cssClass>>> {
    // Initialize an empty vector to store the nodes.
    let mut nodes = vec![];

    // Iterate through the inner pairs of the provided `pair`.
    for inner_pair in pair.into_inner() {
        // Attempt to build nodes from each `inner_pair` by calling `build_nodes_from_class`.
        // If the function returns `Some`, push the result into the `nodes` vector.
        if let Some(node) = build_nodes_from_class(inner_pair) {
            nodes.push(node);
        }
    }

    // Return the nodes wrapped in `Some`.
    Some(nodes)
}
