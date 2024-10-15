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
        const CHAR_WIDTH: usize = 20;

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
            } => (1 + variable.len()) * CHAR_WIDTH,
            Expression::Unary {
                operator: _,
                expression: _,
            } => CHAR_WIDTH,
            Expression::Variable { identifier } => identifier.len() * CHAR_WIDTH,
        };
    }
}

pub struct XmlRenderer {
    expression: Expression,
    content: String,
}

impl XmlRenderer {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            content: String::new(),
        }
    }

    fn render_text(&mut self, x: usize, y: usize, text: &str) {
        self.content
            .push_str(&format!("<text x=\"{}\" y=\"{}\">{}</text>", x, y, text));
    }

    fn render_line(&mut self, x_start: usize, y_start: usize, x_end: usize, y_end: usize) {
        self.content.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" />",
            x_start, y_start, x_end, y_end
        ));
    }

    pub fn render(&mut self) -> String {
        // Create binary tree
        let tree = Tree::from_expression(&self.expression);

        let mut queue: Vec<&Node> = vec![&tree.root];
        let mut height = 0;
        const PADDING: usize = 20;

        while !queue.is_empty() {
            let node = queue.remove(0);
            node.children.iter().for_each(|n| queue.push(n));

            height = height.max(node.y);

            self.render_text(node.x + node.full_width / 2, node.y + PADDING, &node.text);
            for child in &node.children {
                self.render_line(
                    node.x + node.full_width / 2 + 7,
                    node.y + PADDING + 2,
                    child.x + child.full_width / 2 + 7,
                    child.y + PADDING - 13,
                )
            }
        }

        format!(
            "<svg width=\"{}\" height=\"{}\">{}</svg>",
            tree.root.full_width,
            height + 2 * PADDING,
            self.content
        )
    }
}
