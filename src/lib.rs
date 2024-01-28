mod ast;

/* See https://stackoverflow.com/a/49020435 */
#[repr(C)]
pub struct JsInteropString {
    data: *const u8,
    len: usize,
}

#[no_mangle]
pub unsafe extern "C" fn generate_svg_from_string(s: *const JsInteropString) {
    let s = match s.as_ref() {
        Some(s) => s,
        None => return,
    };

    let data = std::slice::from_raw_parts(s.data, s.len);

    match std::str::from_utf8(data) {
        Ok(s) => predicate_visualizer::generate_svg_from_string(s),
        Err(_) => return,
    }
}

mod predicate_visualizer {
    use std::{fs, path::Path};

    use crate::ast;

    pub fn generate_svg_from_string(formula: &str) {
        let mut lexer = ast::lexer::Lexer::new(formula);
        let mut parser = ast::parser::Parser::new(lexer.tokenize());
        let expression = parser.parse().expect("Error parsing formula.");
        ast::visualizer::Visualizer::visualize(&expression);

        println!(
            "{}",
            fs::read_to_string(Path::new("./out.svg")).expect("Error building svg")
        );
    }
}
