use crate::{
    ast::parser::Expression,
    visualizer::tree::{Node, Tree},
};

pub struct SvgRenderer {
    expression: Expression,
    content: String,
}

impl SvgRenderer {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            content: String::new(),
        }
    }

    fn render_text(&mut self, x: usize, y: usize, text: &str) {
        self.content.push_str(&format!(
            "<text font-family=\"monospace\" x=\"{}\" y=\"{}\">{}</text>",
            x, y, text
        ));
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

            // box: [node.x, node.x+node.full_width]
            // text width: node.width
            self.render_text(
                node.x + node.full_width / 2 - node.width / 2,
                node.y + PADDING,
                &node.text,
            );

            // render box size for debug purposes:
            // self.render_line(
            //     node.x + node.full_width / 2,
            //     node.y,
            //     node.x + node.full_width / 2,
            //     node.y + PADDING,
            // );
            // self.render_line(node.x, node.y, node.x + node.full_width, node.y);

            for child in &node.children {
                self.render_line(
                    node.x + node.full_width / 2,
                    node.y + PADDING + 2,
                    child.x + child.full_width / 2,
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
