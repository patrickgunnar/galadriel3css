pub use g3css_common::*;
pub use g3css_lsp::*;
pub use g3css_observer::*;
pub use g3css_parser::*;
pub use g3css_transformer::*;

pub fn run_framework(file_path: &str) {
    g3css_parser(file_path);
}
