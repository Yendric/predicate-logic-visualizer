#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ForAll,
    Exists,
    And,
    Or,
    Not,
    Implies,
    Iff,
    Identifier,
    Equals,
    NotEquals,
    ParenOpen, // ( { [ are treated as equal
    ParenClose,
    Comma,
    Colon,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenType, value: String) -> Self {
        Self { kind, value }
    }
}

pub struct Lexer {
    input: String,
    current_pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input
                .to_string()
                .replace("=>", "⇒")
                .replace("<=>", "⇔")
                .replace("~=", "≠"),
            current_pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.drop_spaces();
        let c = self.current_char();

        if let Some(c) = c {
            return match c {
                '∀' | '!' => Some(Token::new(TokenType::ForAll, self.consume())),
                '∃' | '?' => Some(Token::new(TokenType::Exists, self.consume())),
                '∧' | '&' => Some(Token::new(TokenType::And, self.consume())),
                '∨' | '|' => Some(Token::new(TokenType::Or, self.consume())),
                '¬' | '~' => Some(Token::new(TokenType::Not, self.consume())),
                '⇒' => Some(Token::new(TokenType::Implies, self.consume())),
                '⇔' => Some(Token::new(TokenType::Iff, self.consume())),
                '(' | '{' | '[' => Some(Token::new(TokenType::ParenOpen, self.consume())),
                ')' | '}' | ']' => Some(Token::new(TokenType::ParenClose, self.consume())),
                ',' => Some(Token::new(TokenType::Comma, self.consume())),
                ':' => Some(Token::new(TokenType::Colon, self.consume())),
                '=' => Some(Token::new(TokenType::Equals, self.consume())),
                '≠' => Some(Token::new(TokenType::NotEquals, self.consume())),
                c if c.is_alphanumeric() => self.read_identifier(),
                _ => panic!("Unexpected character: {}", c),
            };
        } else {
            None
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn drop_spaces(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }
            self.current_pos += 1;
        }
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.current_char() {
            if !c.is_alphanumeric() {
                break;
            }
            string.push_str(&self.consume());
        }

        Some(Token::new(TokenType::Identifier, string))
    }

    fn consume(&mut self) -> String {
        let c = self.current_char().unwrap();
        self.current_pos += 1;

        c.to_string()
    }
}
