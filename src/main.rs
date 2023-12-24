use std::io;

mod ast;

fn main() {
    println!("Please enter a formula: ");

    let mut formula = String::new();
    io::stdin()
        .read_line(&mut formula)
        .expect("Error reading input.");

    let mut lexer = ast::lexer::Lexer::new(&formula);
    let mut parser = ast::parser::Parser::new(lexer.tokenize());
    let expression = parser.parse().expect("Error parsing formula.");
    let mut visualizer = ast::visualizer::Visualizer::new(expression);
    visualizer.visualize();
}
