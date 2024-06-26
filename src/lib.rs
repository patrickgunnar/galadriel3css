pub use g3css_ast::*;
pub use g3css_lsp::*;
pub use g3css_observer::*;
pub use g3css_parser::*;
pub use g3css_transformer::*;

pub fn run_framework(file_path: &str) {
    match g3css_parser(file_path) {
        Ok(ast) => {
            g3css_ast_setter(ast);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}
