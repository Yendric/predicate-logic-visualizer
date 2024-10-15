use crate::ast::parser::Expression;

use super::drawable::Drawable;

pub struct Node {
    pub text: String,
    pub children: Vec<Node>,
    pub full_width: usize,
    pub y: usize,
    pub x: usize,
}

pub struct Tree {
    pub root: Node,
}

impl Tree {
    pub fn from_expression(expression: &Expression) -> Self {
        return Self {
            root: Node::from_expression(expression, 0, 0),
        };
    }
}

impl Node {
    fn from_expression(expression: &Expression, start_y: usize, start_x: usize) -> Self {
        let mut children = vec![];
        let text;
        let mut full_width = expression.text_width();
        const X_SEPERATION: usize = 20;
        const Y_SEPERATION: usize = 40;

        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                text = format!("{}", operator);
                let node_1 = Node::from_expression(left, start_y + Y_SEPERATION, start_x);
                let node_2 = Node::from_expression(
                    right,
                    start_y + Y_SEPERATION,
                    start_x + node_1.full_width + X_SEPERATION,
                );
                full_width = full_width.max(node_1.full_width + node_2.full_width + X_SEPERATION);
                children.push(node_1);
                children.push(node_2);
            }
            Expression::Predicate {
                identifier,
                arguments,
            } => {
                text = identifier.clone();
                let mut width_sum = 0;
                for arg in arguments {
                    let node =
                        Node::from_expression(arg, start_y + Y_SEPERATION, start_x + width_sum);
                    width_sum += node.full_width + X_SEPERATION;
                    children.push(node);
                }
                width_sum = width_sum.saturating_sub(X_SEPERATION); // ensure we don't go negative when arguments empty
                full_width = full_width.max(width_sum);
            }
            Expression::Quantifier {
                operator,
                variable,
                formula,
            } => {
                text = format!("{}{}", operator, variable);
                let node = Node::from_expression(formula, start_y + Y_SEPERATION, start_x);
                full_width = full_width.max(node.full_width);
                children.push(node);
            }
            Expression::Unary {
                operator,
                expression,
            } => {
                text = format!("{}", operator);
                let node = Node::from_expression(expression, start_y + Y_SEPERATION, start_x);
                full_width = full_width.max(node.full_width);
                children.push(node);
            }
            Expression::Variable { identifier } => {
                text = identifier.clone();
            }
        }

        Self {
            text,
            children,
            y: start_y,
            x: start_x,
            full_width,
        }
    }
}
