pub mod types {
    pub mod g3css_alias;
    pub mod g3css_children;
    pub mod g3css_class;
    pub mod g3css_elements;
    pub mod g3css_node;
    pub mod g3css_panoramic;
    pub mod g3css_theme;
    pub mod g3css_variable;
}

pub mod rustal {
    pub mod alias_handlers;
    pub mod ast_handlers;
    pub mod build_ast_from_elements;
    pub mod children_handler;
    pub mod class_handler;
    pub mod elements_handler;
    pub mod panoramic_handlers;
    pub mod theme_handlers;
    pub mod utils;
    pub mod variable_handler;
}

use pest::{error::Error, Parser};
use pest_derive::Parser;
use rustal::ast_handlers::build_ast_from_rule;
use std::rc::Rc;
use types::g3css_node::G3cssNode;

#[derive(Parser)]
#[grammar = "grammar/g3css.pest"]
struct G3cssParser;

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
