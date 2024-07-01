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

/// Parses the source string `src` and returns a Result containing either a reference-counted
/// G3cssNode or an Error.
///
/// # Arguments
///
/// - `src` - A string slice that holds the source code to be parsed.
///
/// # Returns
///
/// - `Result<Rc<G3cssNode>, Error<Rule>>` - Ok containing a reference-counted G3cssNode
///   if parsing is successful, or an Err containing an Error if parsing fails.
fn parse(src: &str) -> Result<Rc<G3cssNode>, Error<Rule>> {
    // Attempt to parse the source string using the G3cssParser and Rule::program.
    match G3cssParser::parse(Rule::program, src) {
        // If parsing is successful, process the parsed pairs.
        Ok(pairs) => {
            // Iterate over parsed pairs and build the AST nodes based on their rules.
            for pair in pairs {
                match pair.as_rule() {
                    // If the rule is a global rule, build the AST node and return it.
                    Rule::global => {
                        return Ok(Rc::new(build_ast_from_rule(pair).unwrap()));
                    }
                    // If the rule is a component rule, build the AST node and return it.
                    Rule::component => {
                        return Ok(Rc::new(build_ast_from_rule(pair).unwrap()));
                    }
                    // If the rule does not match any known rules, do nothing.
                    _ => {}
                }
            }
        }
        // If parsing fails, return the error.
        Err(error) => {
            return Err(error);
        }
    }

    // If no rules match, return a unit type wrapped in a G3cssNode.
    Ok(Rc::new(G3cssNode::Unit))
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
            match parse(&raw_file) {
                Ok(ast) => {
                    println!("{:#?}", ast);
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            }
        }
        Err(error) => {
            println!("Can't read G3CSS file!");
            println!("{}", error);
        }
    }
}
