mod ast;
mod visualizer;
use visualizer::drawable::XmlRenderer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_svg_from_string(formula: &str) -> String {
    let mut lexer = ast::lexer::Lexer::new(formula);
    let mut parser = ast::parser::Parser::new(lexer.tokenize());
    let expression = parser.parse().expect("Error parsing formula.");

    return XmlRenderer::new(expression).render();
}
