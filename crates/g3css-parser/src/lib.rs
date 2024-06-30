use std::rc::Rc;

use g3css_common::{
    G3cssAlias, G3cssChildren, G3cssClass, G3cssElements, G3cssNode, G3cssPanoramic, G3cssTheme,
    G3cssVariable,
};
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
fn build_nodes_from_breakpoints(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssPanoramic>> {
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
fn build_nodes_from_panoramic_viewer(
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

/// Builds a G3CSS class node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS class node.
///
/// # Returns
/// Option containing the constructed G3CSS class node if matched, or None if the rule doesn't match.
fn build_ast_from_class(pair: pest::iterators::Pair<Rule>) -> Option<G3cssClass> {
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

/// Removes leading and trailing whitespace from the input string.
///
/// Splits the input string by whitespace and returns the first segment without leading or trailing whitespace.
/// Returns an empty string if the input is empty or consists only of whitespace.
///
/// # Arguments
///
/// - `input` - A reference to a string slice (`&str`) from which whitespace should be removed.
///
/// # Returns
///
/// A new `String` with leading and trailing whitespace removed from the input, or an empty string if no non-whitespace characters are found.
fn remove_whitespace(input: &str) -> String {
    // Split the input string by whitespace and retrieve the first segment without leading or trailing whitespace
    input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches('"')
        .to_string()
}

/// Generates a vector of strings representing an alias/variable from a Pest `Pair`.
///
/// Constructs a vector of strings based on inner pairs of the provided `Pair`.
/// Returns `Some(Vec<String>)` with the constructed alias/variable vector, or `None` if
/// parsing or construction fails.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing alias/variable components.
///
/// # Returns
///
/// An `Option<Vec<String>>` containing the constructed vector of strings
/// representing an alias/variable, or `None` if parsing fails.
fn generates_string_vec(pair: pest::iterators::Pair<Rule>) -> Option<Vec<String>> {
    // Create an empty vector to hold the alias/variable components
    let mut alias_var = vec![];

    // Iterate over each inner pair within the provided pair
    for inner_pair in pair.into_inner() {
        // Match the rule of the inner pair to determine the alias/variable component type
        match inner_pair.as_rule() {
            // If it matches Rule::leading, push the trimmed string to alias/variable vector
            Rule::leading => {
                alias_var.push(remove_whitespace(inner_pair.as_str()));
            }
            // If it matches Rule::importance, push the trimmed string to alias vector
            Rule::importance => {
                alias_var.push(remove_whitespace(inner_pair.as_str()));
            }
            // If it matches Rule::worth, push the trimmed string to variable vector
            Rule::worth => {
                alias_var.push(remove_whitespace(inner_pair.as_str()));
            }
            // Ignore other rules
            _ => (),
        }
    }

    // Return the alias vector wrapped in `Some`, indicating successful construction
    Some(alias_var)
}

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
fn build_ast_from_alias(pair: pest::iterators::Pair<Rule>) -> Option<G3cssAlias> {
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
fn build_nodes_from_aliases(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssAlias>> {
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
fn build_ast_from_variable(pair: pest::iterators::Pair<Rule>) -> Option<G3cssVariable> {
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
fn build_nodes_from_variables(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssVariable>> {
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
fn build_ast_from_theme(pair: pest::iterators::Pair<Rule>) -> Option<G3cssTheme> {
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
fn build_nodes_from_theme(pair: pest::iterators::Pair<Rule>) -> Option<Vec<G3cssTheme>> {
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
