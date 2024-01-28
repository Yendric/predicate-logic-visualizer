use super::parser::Expression;
use id_tree::{InsertBehavior, Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, Visualize};

pub struct Visualizer {}

struct TreeNode {
    value: String,
}

impl Visualize for TreeNode {
    fn visualize(&self) -> String {
        self.value.clone()
    }
}

impl Visualizer {
    pub fn visualize(expression: &Expression) {
        Layouter::new(&Visualizer::build_tree(expression))
            .with_file_path(std::path::Path::new("out.svg"))
            .write()
            .expect("Failed writing layout")
    }

    fn build_tree(expression: &Expression) -> Tree<TreeNode> {
        let mut tree = TreeBuilder::new().build();
        let root = tree
            .insert(
                Node::new(TreeNode {
                    value: "Formula".to_string(),
                }),
                id_tree::InsertBehavior::AsRoot,
            )
            .unwrap();

        Visualizer::insert_expression(&mut tree, expression, &root);

        tree
    }

    fn insert_expression(
        tree: &mut Tree<TreeNode>,
        expression: &Expression,
        parent: &NodeId,
    ) -> () {
        match expression {
            Expression::Variable { identifier } => {
                tree.insert(
                    Node::new(TreeNode {
                        value: identifier.clone(),
                    }),
                    InsertBehavior::UnderNode(&parent),
                )
                .unwrap();
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let branch = tree
                    .insert(
                        Node::new(TreeNode {
                            value: format!("{}", operator),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();

                Visualizer::insert_expression(tree, left, &branch);
                Visualizer::insert_expression(tree, right, &branch);
            }
            Expression::Unary {
                ref operator,
                ref expression,
            } => {
                let branch = tree
                    .insert(
                        Node::new(TreeNode {
                            value: format!("{}", operator),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();
                Visualizer::insert_expression(tree, expression, &branch);
            }
            Expression::Predicate {
                identifier,
                arguments,
            } => {
                let branch = tree
                    .insert(
                        Node::new(TreeNode {
                            value: identifier.clone(),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();

                for argument in arguments {
                    Visualizer::insert_expression(tree, argument, &branch);
                }
            }
            Expression::Quantifier {
                operator,
                variable,
                formula,
            } => {
                let branch = tree
                    .insert(
                        Node::new(TreeNode {
                            value: format!("{}{}", operator, variable),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();

                Visualizer::insert_expression(tree, formula, &branch);
            }
        }
    }
}
