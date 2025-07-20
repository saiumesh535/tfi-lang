# Lexer (Tokenization) - TFI Language

## What is a Lexer?

A lexer (or tokenizer) is the first component of a compiler that breaks down source code into individual tokens. It recognizes keywords, identifiers, numbers, operators, and other language elements. Think of it as the "word processor" of the compiler - it reads the raw text and identifies meaningful units.

## How it Works in TFI

The TFI lexer uses the `logos` crate to efficiently tokenize source code. It scans the input character by character and groups them into meaningful tokens based on predefined patterns.

## Token Types

### Keywords (Telugu Movie Names)

| TFI Keyword | Token Type | JavaScript Equivalent |
|-------------|------------|----------------------|
| `rrr` | `Token::Const` | `const` |
| `pushpa` | `Token::Let` | `let` |
| `bahubali` | `Token::Print` | `console.log` |
| `magadheera` | `Token::If` | `if` |
| `karthikeya` | `Token::Else` | `else` |
| `pokiri` | `Token::While` | `while` |
| `eega` | `Token::For` | `for` |

### Other Tokens

| Token Type | Pattern | Examples |
|------------|---------|----------|
| `Identifier` | `[a-zA-Z]+` | `x`, `y`, `counter`, `myVariable` |
| `Number` | `[0-9]+` | `42`, `100`, `0`, `999` |
| `String` | `"..."` | `"Hello"`, `"World"`, `"42"` |
| `Operator` | Various | `+`, `-`, `*`, `/`, `>`, `<`, `>=`, `<=`, `==`, `!=` |
| `Punctuation` | Various | `(`, `)`, `{`, `}`, `;`, `=`, `,` |

## Examples

### Example 1: Simple Variable Declaration

**TFI Source:**
```tfi
rrr x = 42;
```

**Token Stream:**
```
Token::Const
Token::Identifier("x")
Token::Assign
Token::Number(42)
Token::Semicolon
```

**Visual Representation:**
```
[rrr] [x] [=] [42] [;]
```

### Example 2: Print Statement

**TFI Source:**
```tfi
bahubali("Hello, world!");
```

**Token Stream:**
```
Token::Print
Token::LParen
Token::String("Hello, world!")
Token::RParen
Token::Semicolon
```

**Visual Representation:**
```
[bahubali] [(] ["Hello, world!"] [)] [;]
```

### Example 3: Binary Expression

**TFI Source:**
```tfi
rrr result = x + y;
```

**Token Stream:**
```
Token::Const
Token::Identifier("result")
Token::Assign
Token::Identifier("x")
Token::Plus
Token::Identifier("y")
Token::Semicolon
```

**Visual Representation:**
```
[rrr] [result] [=] [x] [+] [y] [;]
```

### Example 4: If Statement

**TFI Source:**
```tfi
magadheera(x > 5) {
    bahubali("x is greater than 5");
}
```

**Token Stream:**
```
Token::If
Token::LParen
Token::Identifier("x")
Token::Greater
Token::Number(5)
Token::RParen
Token::LBrace
Token::Print
Token::LParen
Token::String("x is greater than 5")
Token::RParen
Token::Semicolon
Token::RBrace
```

**Visual Representation:**
```
[magadheera] [(] [x] [>] [5] [)] [{]
    [bahubali] [(] ["x is greater than 5"] [)] [;]
[}]
```

### Example 5: While Loop

**TFI Source:**
```tfi
pokiri(i < 10) {
    bahubali(i);
    pushpa i = i + 1;
}
```

**Token Stream:**
```
Token::While
Token::LParen
Token::Identifier("i")
Token::Less
Token::Number(10)
Token::RParen
Token::LBrace
Token::Print
Token::LParen
Token::Identifier("i")
Token::RParen
Token::Semicolon
Token::Let
Token::Identifier("i")
Token::Assign
Token::Identifier("i")
Token::Plus
Token::Number(1)
Token::Semicolon
Token::RBrace
```

**Visual Representation:**
```
[pokiri] [(] [i] [<] [10] [)] [{]
    [bahubali] [(] [i] [)] [;]
    [pushpa] [i] [=] [i] [+] [1] [;]
[}]
```

### Example 6: For Loop

**TFI Source:**
```tfi
eega(rrr i = 0; i < 5; i + 1) {
    bahubali(i);
}
```

**Token Stream:**
```
Token::For
Token::LParen
Token::Const
Token::Identifier("i")
Token::Assign
Token::Number(0)
Token::Semicolon
Token::Identifier("i")
Token::Less
Token::Number(5)
Token::Semicolon
Token::Identifier("i")
Token::Plus
Token::Number(1)
Token::RParen
Token::LBrace
Token::Print
Token::LParen
Token::Identifier("i")
Token::RParen
Token::Semicolon
Token::RBrace
```

**Visual Representation:**
```
[eega] [(] [rrr] [i] [=] [0] [;] [i] [<] [5] [;] [i] [+] [1] [)] [{]
    [bahubali] [(] [i] [)] [;]
[}]
```

## Lexer Implementation

### Token Definition

```rust
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
```

### Lexer Structure

```rust
pub struct Lexer {
    tokens: Vec<Token>,
    position: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(source: &str) -> Self {
        let tokens: Vec<Token> = Token::lexer(source)
            .filter_map(|token| token.ok())
            .collect();
        Self { tokens, position: 0 }
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
}
```

## Key Features

### 1. Whitespace Skipping
The lexer automatically ignores spaces, tabs, and newlines:
```tfi
rrr   x   =   42   ;
```
Becomes:
```
Token::Const, Token::Identifier("x"), Token::Assign, Token::Number(42), Token::Semicolon
```

### 2. Error Recovery
The lexer continues processing even if some tokens are invalid:
```tfi
rrr x = 42;
invalid@token;
bahubali("Hello");
```
The lexer will skip the invalid token and continue with valid ones.

### 3. Efficient Tokenization
Using the `logos` crate provides:
- Fast tokenization
- Memory efficient
- Easy to extend with new tokens

### 4. Extensible Design
Adding new keywords is simple:
```rust
#[token("newkeyword")] NewKeyword,
```

## Common Patterns

### Pattern 1: Keyword Detection
```rust
impl Token {
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
}
```

### Pattern 2: Operator Detection
```rust
impl Token {
    pub fn is_operator(&self) -> bool {
        matches!(self, 
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide |
            Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual |
            Token::Equal | Token::NotEqual | Token::Assign
        )
    }
}
```

### Pattern 3: Token Navigation
```rust
let mut lexer = Lexer::new("rrr x = 42;");
assert_eq!(lexer.current(), Some(&Token::Const));
lexer.advance();
assert_eq!(lexer.current(), Some(&Token::Identifier("x".to_string())));
```

## Testing the Lexer

The lexer includes comprehensive tests:

```rust
#[test]
fn test_keyword_tokens() {
    let source = "rrr pushpa bahubali magadheera karthikeya pokiri eega";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.current(), Some(&Token::Const));
    lexer.advance();
    assert_eq!(lexer.current(), Some(&Token::Let));
    // ... continue for all keywords
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
```

## Error Handling

### Invalid Tokens
When the lexer encounters invalid tokens, it can:
1. Skip them and continue
2. Report them as errors
3. Try to recover and continue

### Token Validation
```rust
fn validate_token(token: &Token) -> Result<(), String> {
    match token {
        Token::Identifier(name) => {
            if name.is_empty() {
                Err("Empty identifier".to_string())
            } else {
                Ok(())
            }
        }
        Token::Number(n) => {
            if *n < 0 {
                Err("Negative numbers not supported".to_string())
            } else {
                Ok(())
            }
        }
        _ => Ok(())
    }
}
```

## Performance Considerations

### Memory Usage
- Tokens are stored in a vector for random access
- String tokens are cloned for ownership
- Consider using string interning for large programs

### Speed Optimizations
- `logos` provides fast regex matching
- Whitespace is skipped efficiently
- Token lookups are O(1)

## Summary

The lexer is the foundation of the compiler. It transforms raw text into a structured stream of tokens that the parser can understand. The TFI lexer is efficient, extensible, and provides good error handling, making it a solid foundation for the rest of the compilation process.

Key takeaways:
- Lexers break text into meaningful units
- TFI uses Telugu movie names as keywords
- The `logos` crate provides efficient tokenization
- Whitespace is automatically handled
- The design is extensible for new language features 