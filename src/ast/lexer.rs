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
            input: input.to_string(),
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
            if c.is_alphabetic() {
                return self.read_identifier();
            }

            let token_type;

            match c {
                '∀' => token_type = TokenType::ForAll,
                '∃' => token_type = TokenType::Exists,
                '∧' => token_type = TokenType::And,
                '∨' => token_type = TokenType::Or,
                '¬' => token_type = TokenType::Not,
                '⇒' => token_type = TokenType::Implies,
                '⇔' => token_type = TokenType::Iff,
                '(' | '{' | '[' => token_type = TokenType::ParenOpen,
                ')' | '}' | ']' => token_type = TokenType::ParenClose,
                ',' => token_type = TokenType::Comma,
                ':' => token_type = TokenType::Colon,
                '=' => token_type = TokenType::Equals,
                '≠' => token_type = TokenType::NotEquals,
                _ => panic!("Unexpected character: {}", c),
            }

            Some(Token::new(token_type, self.consume()))
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
            if !c.is_alphabetic() {
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
