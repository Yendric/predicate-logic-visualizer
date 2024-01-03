use core::fmt;
use std::fmt::Display;

use super::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum Expression {
    Predicate {
        identifier: String,
        arguments: Vec<Expression>,
    },
    Variable {
        identifier: String,
    },
    Quantifier {
        operator: QuantifyingOperator,
        variable: String,
        formula: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum QuantifyingOperator {
    ForAll,
    Exists,
}

impl Display for QuantifyingOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            QuantifyingOperator::Exists => "∃",
            QuantifyingOperator::ForAll => "∀",
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            UnaryOperator::Not => "¬",
        };

        write!(f, "{}", string)
    }
}

/**
 * Operator precedence in predicate-logic:   Not > Equals > And > Or > Implies > Iff > ForAll, Exists
 *                                                  6      5    4       3       2               
 * Non binary operators are parsed seperately and hence don't need a number to represent their precedence.
*/
#[derive(Debug)]
pub enum BinaryOperator {
    Conjunction,
    Disjunction,
    Implication,
    Iff,
    Equals,
    NotEquals,
}

impl BinaryOperator {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Conjunction => 5,
            BinaryOperator::Disjunction => 4,
            BinaryOperator::Implication => 3,
            BinaryOperator::Iff => 2,
            BinaryOperator::Equals => 6,
            BinaryOperator::NotEquals => 6,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            BinaryOperator::Conjunction => "∧",
            BinaryOperator::Disjunction => "∨",
            BinaryOperator::Implication => "⇒",
            BinaryOperator::Iff => "⇔",
            BinaryOperator::Equals => "=",
            BinaryOperator::NotEquals => "≠",
        };

        write!(f, "{}", string)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<Expression> {
        let mut left = self.parse_unary_expression()?;

        while let Some(operator) = self.peek_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }

            self.consume(); // Consume operator

            let right = self
                .parse_binary_expression(operator_precedence)
                .expect("Expected expression after operator");

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn peek_binary_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.current()?;

        match token.kind {
            TokenType::And => Some(BinaryOperator::Conjunction),
            TokenType::Or => Some(BinaryOperator::Disjunction),
            TokenType::Implies => Some(BinaryOperator::Implication),
            TokenType::Iff => Some(BinaryOperator::Iff),
            TokenType::Equals => Some(BinaryOperator::Equals),
            TokenType::NotEquals => Some(BinaryOperator::NotEquals),
            _ => None,
        }
    }

    fn parse_unary_expression(&mut self) -> Option<Expression> {
        let token_kind = self.current()?.kind.clone();

        match token_kind {
            TokenType::Not => {
                self.consume();
                Some(Expression::Unary {
                    operator: UnaryOperator::Not,
                    expression: Box::new(self.parse_unary_expression()?),
                })
            }
            TokenType::ForAll | TokenType::Exists => {
                self.consume(); // Consume quantifier

                let variable = self
                    .consume_expect(TokenType::Identifier, "Expected variable after quantifier.")
                    .value
                    .clone(); // Consume variable

                self.consume_expect(
                    TokenType::Colon,
                    "Expected colon after variable in quantifying statement.",
                ); // Consume colon

                Some(Expression::Quantifier {
                    operator: match token_kind {
                        TokenType::ForAll => QuantifyingOperator::ForAll,
                        TokenType::Exists => QuantifyingOperator::Exists,
                        _ => unreachable!(),
                    },
                    variable,
                    formula: Box::new(
                        self.parse_expression()
                            .expect("Expected expression after quantifier."),
                    ),
                })
            }
            _ => self.parse_primary_expression(),
        }
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        match self.current()?.kind {
            TokenType::Identifier => {
                if let Some(next) = self.peek(1) {
                    if next.kind == TokenType::ParenOpen {
                        return self.parse_predicate();
                    }
                }

                Some(Expression::Variable {
                    identifier: self.consume()?.value.clone(),
                })
            }
            TokenType::ParenOpen => {
                self.consume();
                let expression = self.parse_expression()?;
                self.consume_expect(
                    TokenType::ParenClose,
                    "Expected closing parentheses after parenthesised expression.",
                );
                Some(expression)
            }
            _ => None,
        }
    }

    fn parse_predicate(&mut self) -> Option<Expression> {
        let identifier = self.consume()?.value.clone();
        self.consume_expect(
            TokenType::ParenOpen,
            "Expected open parentheses after predicate.",
        ); // Consume open parentheses

        let mut arguments = Vec::new();
        while let Some(token) = self.current() {
            if token.kind == TokenType::ParenClose {
                self.consume();
                break;
            } else if token.kind == TokenType::Comma {
                self.consume();
                continue;
            }
            arguments.push(self.parse_primary_expression()?);
        }

        Some(Expression::Predicate {
            identifier,
            arguments,
        })
    }

    fn peek(&self, offset: i32) -> Option<&Token> {
        self.tokens.get((self.current + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;

        self.peek(-1)
    }

    fn consume_expect(&mut self, kind: TokenType, message: &str) -> &Token {
        let token = self.consume().expect(message);
        assert_eq!(token.kind, kind, "{}", message);

        token
    }
}
