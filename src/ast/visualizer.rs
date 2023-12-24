use super::parser::Expression;
use id_tree::{InsertBehavior, Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, Visualize};

pub struct Visualizer {
    expression: Expression,
    tree: Tree<TreeNode>,
}

struct TreeNode {
    value: String,
}

impl Visualize for TreeNode {
    fn visualize(&self) -> String {
        self.value.clone()
    }
}

impl Visualizer {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            tree: TreeBuilder::new().build(),
        }
    }

    pub fn visualize(&mut self) {
        self.build_tree();

        Layouter::new(&self.tree)
            .with_file_path(std::path::Path::new("out.svg"))
            .write()
            .expect("Failed writing layout")
    }

    fn build_tree(&mut self) {
        let root = self
            .tree
            .insert(
                Node::new(TreeNode {
                    value: "Formula".to_string(),
                }),
                id_tree::InsertBehavior::AsRoot,
            )
            .unwrap();

        self.insert_expression(&self.expression.clone(), &root);
    }

    fn insert_expression(&mut self, expression: &Expression, parent: &NodeId) -> () {
        match expression {
            Expression::Variable { identifier } => {
                self.tree
                    .insert(
                        Node::new(TreeNode {
                            value: identifier.clone(),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();
            }
            Expression::Binary {
                ref left,
                ref operator,
                ref right,
            } => {
                let branch = self
                    .tree
                    .insert(
                        Node::new(TreeNode {
                            value: format!("{}", operator),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();

                self.insert_expression(left, &branch);
                self.insert_expression(right, &branch);
            }
            Expression::Unary {
                ref operator,
                ref expression,
            } => {
                let branch = self
                    .tree
                    .insert(
                        Node::new(TreeNode {
                            value: format!("{}", operator),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();
                self.insert_expression(expression, &branch);
            }
            Expression::Predicate {
                identifier,
                arguments,
            } => {
                let branch = self
                    .tree
                    .insert(
                        Node::new(TreeNode {
                            value: identifier.clone(),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();

                for argument in arguments {
                    self.tree
                        .insert(
                            Node::new(TreeNode {
                                value: match argument {
                                    Expression::Variable { identifier } => identifier.clone(),
                                    _ => String::new(),
                                },
                            }),
                            InsertBehavior::UnderNode(&branch),
                        )
                        .unwrap();
                }
            }
            Expression::Quantifier {
                operator,
                variable,
                formula,
            } => {
                let branch = self
                    .tree
                    .insert(
                        Node::new(TreeNode {
                            value: format!("{}{}", operator, variable),
                        }),
                        InsertBehavior::UnderNode(&parent),
                    )
                    .unwrap();

                self.insert_expression(formula, &branch);
            }
        }
    }
}
