use std::rc::Rc;

use g3css_common::{G3cssAlias, G3cssChildren, G3cssClass, G3cssElements, G3cssNode};
use pest::{error::Error, Parser};
use pest_derive::Parser;

mod build_ast_from_elements;
use build_ast_from_elements::build_ast_from_elements;

#[derive(Parser)]
#[grammar = "grammar/g3css.pest"]
struct G3cssParser;

/// Builds a vector of G3CSS elements nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS elements nodes from.
///
/// # Returns
/// Vector of constructed G3CSS elements nodes.
fn build_node_from_elements(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssElements>> {
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

/// Builds a G3CSS class node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS class node.
///
/// # Returns
/// Option containing the constructed G3CSS class node if matched, or None if the rule doesn't match.
fn build_ast_from_class(pair: pest::iterators::Pair<Rule>) -> Option<G3cssClass> {
    match pair.as_rule() {
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
fn build_nodes_from_class(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssClass>> {
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
fn build_nodes_from_classes(pair: pest::iterators::Pair<Rule>) -> Option<Vec<Vec<G3cssClass>>> {
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

fn remove_whitespace(input: &str) -> String {
    input.split_whitespace().next().unwrap_or("").to_string()
}

fn generates_alias_vec(pair: pest::iterators::Pair<Rule>) -> Option<Vec<String>> {
    let mut alias = vec![];

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::leading => {
                alias.push(remove_whitespace(inner_pair.as_str()));
            }
            Rule::importance => {
                alias.push(remove_whitespace(inner_pair.as_str()));
            }
            _ => (),
        }
    }

    Some(alias)
}

fn build_ast_from_alias(pair: pest::iterators::Pair<Rule>) -> Option<G3cssAlias> {
    match pair.as_rule() {
        Rule::accent_color => Some(G3cssAlias::Alias([].to_vec())),
        Rule::alias => Some(G3cssAlias::Alias(generates_alias_vec(pair)?)),
        _ => None,
    }
}

fn build_nodes_from_aliases(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssAlias>> {
    let mut nodes = vec![];

    for inner_pair in pair.into_inner() {
        // Attempt to build nodes from each `inner_pair` by calling `build_nodes_from_class`.
        // If the function returns `Some`, push the result into the `nodes` vector.
        if let Some(node) = build_ast_from_alias(inner_pair) {
            nodes.push(node);
        }
    }

    Some(nodes)
}

/// Builds a G3CSS children node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS children node.
///
/// # Returns
/// Option containing the constructed G3CSS children node if matched, or None if the rule doesn't match.
fn build_ast_from_children(pair: pest::iterators::Pair<Rule>) -> Option<G3cssChildren> {
    match pair.as_rule() {
        // Collects the value from the aliases rule.
        Rule::aliases => Some(G3cssChildren::Aliases(build_nodes_from_aliases(pair)?)),
        // Collects the value from the variables rule.
        Rule::variables => Some(G3cssChildren::Variable(
            pair.into_inner().as_str().to_string(),
        )),
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
fn build_node_from_children(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssChildren>> {
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

/// Builds a vector of G3CSS nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS nodes from.
///
/// # Returns
/// Vector of constructed G3CSS nodes.
fn build_ast_nodes(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssNode>> {
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
fn build_ast_from_rule(pair: pest::iterators::Pair<Rule>) -> Option<G3cssNode> {
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

/// Parses a source string into an abstract syntax tree (AST) of G3CSS nodes.
///
/// # Parameters
/// - `src`: Source string containing G3CSS code to parse.
///
/// # Returns
/// Result containing a vector of shared references to G3CSS nodes on success, or an Error if parsing fails.
fn parse(src: &str) -> Result<Vec<Rc<G3cssNode>>, Error<Rule>> {
    // Initialize an empty vector to store the AST nodes.
    let mut ast = vec![];

    // Attempt to parse the source string using the G3cssParser and Rule::program.
    match G3cssParser::parse(Rule::program, src) {
        Ok(pairs) => {
            // Iterate over parsed pairs and build the AST nodes based on their rules.
            for pair in pairs {
                match pair.as_rule() {
                    Rule::global => ast.push(Rc::new(build_ast_from_rule(pair).unwrap())),
                    Rule::component => ast.push(Rc::new(build_ast_from_rule(pair).unwrap())),
                    _ => {}
                }
            }
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }

    // Return the parsed AST as a Result.
    Ok(ast)
}

/// Parses a G3CSS file into an abstract syntax tree (AST) if successful.
///
/// # Parameters
/// - `file_path`: Path to the G3CSS file to parse.
pub fn g3css_parser(file_path: &str) {
    // Attempt to read the contents of the file specified by `file_path`.
    match std::fs::read_to_string(file_path) {
        Ok(raw_file) => {
            // If successfully read, `raw_file` contains the file's contents.
            // Attempt to parse the raw file into an abstract syntax tree (AST).
            if let Ok(ast) = parse(&raw_file) {
                println!("{:#?}", ast);
            }
        }
        Err(error) => {
            println!("Can't read G3CSS file!");
            println!("{}", error);
        }
    }
}
