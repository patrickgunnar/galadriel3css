use std::rc::Rc;

use g3css_common::{G3cssChildren, G3cssClass, G3cssElements, G3cssNode};
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "g3css.pest"]
struct G3cssParser;

/// Builds a G3CSS elements node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS elements node.
///
/// # Returns
/// Option containing the constructed G3CSS elements node if matched, or None if the rule doesn't match.
fn build_ast_from_elements(pair: pest::iterators::Pair<Rule>) -> Option<G3cssElements> {
    match pair.as_rule() {
        // Collects the value from the background rule.
        Rule::background => Some(G3cssElements::Background(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the display rule.
        Rule::display => Some(G3cssElements::Display(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border rule.
        Rule::border => Some(G3cssElements::Border(
            pair.as_str().trim_matches('"').to_string(),
        )),
        _ => None,
    }
}

/// Builds a vector of G3CSS elements nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS elements nodes from.
///
/// # Returns
/// Vector of constructed G3CSS elements nodes.
fn build_node_from_elements(pair: pest::iterators::Pair<Rule>) -> Vec<G3cssElements> {
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
    nodes
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
        // Iterate over inner pairs and build nodes from properties rules.
        Rule::properties => Some(G3cssClass::Properties(build_node_from_elements(pair))),
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
fn build_nodes_from_class(pair: pest::iterators::Pair<Rule>) -> Vec<G3cssClass> {
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
    nodes
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
        // Collects the value from the alias rule.
        Rule::alias => Some(G3cssChildren::Alias(pair.into_inner().as_str().to_string())),
        // Collects the value from the variables rule.
        Rule::variables => Some(G3cssChildren::Variable(
            pair.into_inner().as_str().to_string(),
        )),
        // Collects the value from the class rule.
        Rule::class => Some(G3cssChildren::Class(build_nodes_from_class(pair))),
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
fn build_node_from_children(pair: pest::iterators::Pair<Rule>) -> Vec<G3cssChildren> {
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
    nodes
}

/// Builds a vector of G3CSS nodes from inner pairs based on their rules.
///
/// # Parameters
/// - `pair`: Parsing pair containing inner pairs to build G3CSS nodes from.
///
/// # Returns
/// Vector of constructed G3CSS nodes.
fn build_ast_nodes(pair: pest::iterators::Pair<Rule>) -> Vec<G3cssNode> {
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
    nodes
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
        Rule::global => Some(G3cssNode::Global(build_ast_nodes(pair))),
        // Iterate over inner pairs and build nodes from component rules.
        Rule::component => Some(G3cssNode::Component(build_ast_nodes(pair))),
        // collects the value from component or global name.
        Rule::name => Some(G3cssNode::Name(pair.as_str().trim_matches('"').to_string())),
        // collects the extends value from the component.
        Rule::extends => Some(G3cssNode::Extends(
            pair.as_str().trim_matches('"').to_string(),
        )),
        Rule::children => {
            // Iterate over inner pairs and build nodes from children rules.
            Some(G3cssNode::Children(build_node_from_children(pair)))
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
    let pairs = G3cssParser::parse(Rule::program, src)?;

    // Iterate over parsed pairs and build the AST nodes based on their rules.
    for pair in pairs {
        match pair.as_rule() {
            Rule::global => ast.push(Rc::new(build_ast_from_rule(pair).unwrap())),
            Rule::component => ast.push(Rc::new(build_ast_from_rule(pair).unwrap())),
            _ => {}
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
