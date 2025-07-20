use logos::Logos;

/// Token types for the TFI language lexer
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    /// Keywords
    #[token("rrr")] Const,
    #[token("pushpa")] Let,
    #[token("bahubali")] Print,
    #[token("magadheera")] If,
    #[token("karthikeya")] Else,
    #[token("pokiri")] While,
    #[token("eega")] For,
    
    /// Identifiers (variable names)
    #[regex("[a-zA-Z]+", |lex| lex.slice().to_string())] 
    Identifier(String),
    
    /// Numeric literals
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())] 
    Number(i32),
    
    /// Operators and punctuation
    #[token("=")] Assign,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(";")] Semicolon,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Multiply,
    #[token("/")] Divide,
    #[token(">")] Greater,
    #[token("<")] Less,
    #[token(">=")] GreaterEqual,
    #[token("<=")] LessEqual,
    #[token("==")] Equal,
    #[token("!=")] NotEqual,
    
    /// Whitespace (skipped)
    #[regex(r"[ \t\n\f]+", logos::skip)] 
    Whitespace,
}

impl Token {
    /// Check if the token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self, 
            Token::Const | 
            Token::Let | 
            Token::Print | 
            Token::If | 
            Token::Else | 
            Token::While | 
            Token::For
        )
    }
    
    /// Get the keyword name as a string
    pub fn keyword_name(&self) -> Option<&'static str> {
        match self {
            Token::Const => Some("rrr"),
            Token::Let => Some("pushpa"),
            Token::Print => Some("bahubali"),
            Token::If => Some("magadheera"),
            Token::Else => Some("karthikeya"),
            Token::While => Some("pokiri"),
            Token::For => Some("eega"),
            _ => None,
        }
    }
    
    /// Check if the token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self, 
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide |
            Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual |
            Token::Equal | Token::NotEqual | Token::Assign
        )
    }
    
    /// Get the operator symbol as a string
    pub fn operator_symbol(&self) -> Option<&'static str> {
        match self {
            Token::Plus => Some("+"),
            Token::Minus => Some("-"),
            Token::Multiply => Some("*"),
            Token::Divide => Some("/"),
            Token::Greater => Some(">"),
            Token::Less => Some("<"),
            Token::GreaterEqual => Some(">="),
            Token::LessEqual => Some("<="),
            Token::Equal => Some("=="),
            Token::NotEqual => Some("!="),
            Token::Assign => Some("="),
            _ => None,
        }
    }
}

/// Lexer for the TFI language
pub struct Lexer {
    tokens: Vec<Token>,
    position: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(source: &str) -> Self {
        let tokens: Vec<Token> = Token::lexer(source).filter_map(|token| token.ok()).collect();
        Self {
            tokens,
            position: 0,
        }
    }
    
    /// Get the current token
    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    /// Peek at the next token without consuming it
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }
    
    /// Advance to the next token
    pub fn advance(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        self.current()
    }
    
    /// Check if we've reached the end of tokens
    pub fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }
    
    /// Get all tokens (for debugging)
    pub fn all_tokens(&self) -> &[Token] {
        &self.tokens
    }
    
    /// Reset the lexer position
    pub fn reset(&mut self) {
        self.position = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_tokens() {
        let source = "rrr pushpa bahubali magadheera karthikeya pokiri eega";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.current(), Some(&Token::Const));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Let));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Print));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::If));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Else));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::While));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::For));
    }

    #[test]
    fn test_identifier_tokens() {
        let source = "hello world x y z";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.current(), Some(&Token::Identifier("hello".to_string())));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Identifier("world".to_string())));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Identifier("x".to_string())));
    }

    #[test]
    fn test_number_tokens() {
        let source = "42 123 0 999";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.current(), Some(&Token::Number(42)));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Number(123)));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Number(0)));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Number(999)));
    }

    #[test]
    fn test_operator_tokens() {
        let source = "= ( ) { } ; + >";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.current(), Some(&Token::Assign));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::LParen));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::RParen));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::LBrace));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::RBrace));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Semicolon));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Plus));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Greater));
    }

    #[test]
    fn test_whitespace_skipping() {
        let source = "rrr   pushpa\n\tbahubali";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.current(), Some(&Token::Const));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Let));
        lexer.advance();
        assert_eq!(lexer.current(), Some(&Token::Print));
    }

    #[test]
    fn test_token_methods() {
        assert!(Token::Const.is_keyword());
        assert!(Token::Plus.is_operator());
        assert_eq!(Token::Const.keyword_name(), Some("rrr"));
        assert_eq!(Token::Plus.operator_symbol(), Some("+"));
        assert!(!Token::Identifier("x".to_string()).is_keyword());
    }

    #[test]
    fn test_lexer_methods() {
        let source = "rrr x = 42";
        let mut lexer = Lexer::new(source);
        
        assert!(!lexer.is_eof());
        assert_eq!(lexer.peek(), Some(&Token::Identifier("x".to_string())));
        
        lexer.advance();
        lexer.advance();
        lexer.advance();
        lexer.advance();
        
        assert!(lexer.is_eof());
        
        lexer.reset();
        assert!(!lexer.is_eof());
        assert_eq!(lexer.current(), Some(&Token::Const));
    }
} 