use crate::{
    ast::parser::Expression,
    visualizer::tree::{Node, Tree},
};

pub trait Drawable {
    fn height(&self) -> usize;
    fn text_width(&self) -> usize;
}

impl Drawable for Expression {
    fn height(&self) -> usize {
        const ELT_HEIGHT: usize = 20;

        return match &self {
            Expression::Binary {
                left,
                operator: _,
                right,
            } => std::cmp::max(left.height(), right.height()),
            Expression::Predicate {
                identifier: _,
                arguments,
            } => arguments.iter().map(|a| a.height()).max().unwrap_or(0),
            Expression::Quantifier {
                operator: _,
                variable: _,
                formula,
            } => formula.height(),
            Expression::Unary {
                operator: _,
                expression,
            } => expression.height(),
            Expression::Variable { identifier: _ } => 0,
        } + ELT_HEIGHT;
    }

    fn text_width(&self) -> usize {
        const CHAR_WIDTH: usize = 8;
        const QUANTIFIER_WIDTH: usize = 10;

        return match &self {
            Expression::Binary {
                left: _,
                operator: _,
                right: _,
            } => CHAR_WIDTH,
            Expression::Predicate {
                identifier,
                arguments: _,
            } => identifier.len() * CHAR_WIDTH,
            Expression::Quantifier {
                operator: _,
                variable,
                formula: _,
            } => variable.len() * CHAR_WIDTH + QUANTIFIER_WIDTH,
            Expression::Unary {
                operator: _,
                expression: _,
            } => CHAR_WIDTH,
            Expression::Variable { identifier } => identifier.len() * CHAR_WIDTH,
        };
    }
}
