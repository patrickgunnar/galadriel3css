use std::rc::Rc;

use g3css_common::{G3cssChildren, G3cssNode};
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "g3css.pest"]
struct G3cssParser;

fn build_ast_from_children(pair: pest::iterators::Pair<Rule>) -> Option<G3cssChildren> {
    match pair.as_rule() {
        Rule::alias => Some(G3cssChildren::Alias(pair.into_inner().as_str().to_string())),
        Rule::variables => Some(G3cssChildren::Variable(
            pair.into_inner().as_str().to_string(),
        )),
        Rule::class => Some(G3cssChildren::Class(pair.into_inner().as_str().to_string())),
        _ => None,
    }
}

fn build_ast_from_global(pair: pest::iterators::Pair<Rule>) -> Option<G3cssNode> {
    let mut nodes = vec![];

    for inner_pair in pair.into_inner() {
        if let Some(node) = build_ast_from_rule(inner_pair) {
            nodes.push(node);
        }
    }

    Some(G3cssNode::Global(nodes))
}

fn build_ast_from_component(pair: pest::iterators::Pair<Rule>) -> Option<G3cssNode> {
    let mut nodes = vec![];

    for inner_pair in pair.into_inner() {
        if let Some(node) = build_ast_from_rule(inner_pair) {
            nodes.push(node);
        }
    }

    Some(G3cssNode::Component(nodes))
}

fn build_ast_from_rule(pair: pest::iterators::Pair<Rule>) -> Option<G3cssNode> {
    match pair.as_rule() {
        Rule::global => build_ast_from_global(pair),
        Rule::component => build_ast_from_component(pair),
        Rule::class_name => Some(G3cssNode::ClassName(
            pair.as_str().trim_matches('"').to_string(),
        )),
        Rule::extends => Some(G3cssNode::Extends(
            pair.as_str().trim_matches('"').to_string(),
        )),
        Rule::children => {
            let mut nodes = vec![];

            for inner_pair in pair.into_inner() {
                if let Some(node) = build_ast_from_children(inner_pair) {
                    nodes.push(node);
                }
            }

            Some(G3cssNode::Children(nodes))
        }
        _ => None,
    }
}

fn parse(src: &str) -> Result<Vec<Rc<G3cssNode>>, Error<Rule>> {
    let mut ast = vec![];
    let pairs = G3cssParser::parse(Rule::program, src)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::global => ast.push(Rc::new(build_ast_from_rule(pair).unwrap())),
            Rule::component => ast.push(Rc::new(build_ast_from_rule(pair).unwrap())),
            _ => {}
        }
    }

    Ok(ast)
}

pub fn g3css_parser(file_path: &str) {
    match std::fs::read_to_string(file_path) {
        Ok(raw_file) => {
            if let Ok(ast) = parse(&raw_file) {
                println!("{:?}", ast);
            }
        }
        Err(error) => {
            println!("Can't read G3CSS file!");
            println!("{}", error);
        }
    }
}
