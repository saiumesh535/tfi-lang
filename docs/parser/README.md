# Parser (Syntax Analysis) - TFI Language

## What is a Parser?

A parser takes the stream of tokens from the lexer and builds an Abstract Syntax Tree (AST). It ensures the code follows the language's grammar rules and creates a structured representation of the program. Think of it as the "sentence analyzer" - it takes individual words (tokens) and understands how they form meaningful statements.

## How it Works in TFI

The TFI parser uses the `pest` parser generator with a declarative grammar defined in `grammar.pest`. It follows these parsing rules and creates a structured tree representation of the program.

## Grammar Rules

The parser is driven by grammar rules defined in `grammar.pest`:

```pest
program = { SOI ~ statement* ~ EOI }

statement = { 
    print_statement |
    const_statement |
    let_statement |
    if_statement |
    while_statement |
    for_statement
}

print_statement = { "bahubali" ~ "(" ~ expression ~ ("," ~ expression)* ~ ")" ~ ";" }
const_statement = { "rrr" ~ ident ~ "=" ~ expression ~ ";" }
let_statement = { "pushpa" ~ ident ~ "=" ~ expression ~ ";" }

if_statement = { 
    "magadheera" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" ~ 
    else_block?
}

else_block = { "karthikeya" ~ "{" ~ statement* ~ "}" }

while_statement = { "pokiri" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" }

for_statement = { 
    "eega" ~ "(" ~ statement ~ expression ~ ";" ~ expression ~ ")" ~ 
    "{" ~ statement* ~ "}" 
}

operator = { "+" | "-" | "*" | "/" | ">" | "<" | ">=" | "<=" | "==" | "!=" }
expression = { term ~ (operator ~ term)* }

term = { 
    number | 
    ident | 
    string |
    "(" ~ expression ~ ")"
}

string = @{ "\"" ~ (ASCII_ALPHANUMERIC | " " | "!" | "#" | "$" | "%" | "&" | "'" | "(" | ")" | "*" | "+" | "," | "-" | "." | "/" | ":" | ";" | "<" | "=" | ">" | "?" | "@" | "[" | "]" | "^" | "_" | "`" | "{" | "|" | "}" | "~")* ~ "\"" }

number = @{ ASCII_DIGIT+ }
ident = @{ ASCII_ALPHA+ }

WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* ~ ("\n" | EOI) }
```

## Parsing Process

### 1. Token Stream Input
The parser receives a stream of tokens from the lexer:
```
Token::Const, Token::Identifier("x"), Token::Assign, Token::Number(42), Token::Semicolon
```

### 2. Grammar Matching
The parser matches tokens against grammar rules:
- `"rrr"` matches the keyword rule
- `ident` matches identifier pattern
- `"="` matches assignment operator
- `expression` matches number literal
- `";"` matches statement terminator

### 3. AST Construction
The parser builds tree structures from matched rules:
```rust
Statement::Const("x".to_string(), Expression::Number(42))
```

### 4. Error Handling
If parsing fails, detailed error messages are provided with line/column information.

## Examples

### Example 1: Simple Variable Declaration

**TFI Source:**
```tfi
rrr x = 42;
```

**Parsing Process:**
1. Matches `const_statement` rule
2. Extracts identifier "x"
3. Parses expression `42` as `Expression::Number(42)`
4. Creates `Statement::Const("x", Expression::Number(42))`

**AST Output:**
```rust
Statement::Const("x".to_string(), Expression::Number(42))
```

### Example 2: Print Statement

**TFI Source:**
```tfi
bahubali("Hello, world!");
```

**Parsing Process:**
1. Matches `print_statement` rule
2. Parses string literal as `Expression::String("Hello, world!")`
3. Creates `Statement::Print` with vector of expressions

**AST Output:**
```rust
Statement::Print(vec![Expression::String("Hello, world!".to_string())])
```

### Example 3: Binary Expression

**TFI Source:**
```tfi
rrr result = x + y;
```

**Parsing Process:**
1. Matches `const_statement` rule
2. Parses `x + y` as binary expression:
   - Left: `Expression::Identifier("x")`
   - Operator: `"+"`
   - Right: `Expression::Identifier("y")`
3. Creates `Statement::Const` with binary expression

**AST Output:**
```rust
Statement::Const(
    "result".to_string(),
    Expression::BinaryOp(
        Box::new(Expression::Identifier("x".to_string())),
        "+".to_string(),
        Box::new(Expression::Identifier("y".to_string()))
    )
)
```

### Example 4: If Statement

**TFI Source:**
```tfi
magadheera(x > 5) {
    bahubali("x is greater than 5");
} karthikeya {
    bahubali("x is 5 or less");
}
```

**Parsing Process:**
1. Matches `if_statement` rule
2. Parses condition `x > 5` as binary expression
3. Parses then block as vector of statements
4. Parses else block as optional vector of statements
5. Creates `Statement::If` with all components

**AST Output:**
```rust
Statement::If(
    Expression::BinaryOp(
        Box::new(Expression::Identifier("x".to_string())),
        ">".to_string(),
        Box::new(Expression::Number(5))
    ),
    vec![
        Statement::Print(vec![Expression::String("x is greater than 5".to_string())])
    ],
    Some(vec![
        Statement::Print(vec![Expression::String("x is 5 or less".to_string())])
    ])
)
```

### Example 5: While Loop

**TFI Source:**
```tfi
pokiri(i < 10) {
    bahubali(i);
    pushpa i = i + 1;
}
```

**Parsing Process:**
1. Matches `while_statement` rule
2. Parses condition `i < 10` as binary expression
3. Parses body as vector of statements
4. Creates `Statement::While` with condition and body

**AST Output:**
```rust
Statement::While(
    Expression::BinaryOp(
        Box::new(Expression::Identifier("i".to_string())),
        "<".to_string(),
        Box::new(Expression::Number(10))
    ),
    vec![
        Statement::Print(vec![Expression::Identifier("i".to_string())]),
        Statement::Let(
            "i".to_string(),
            Expression::BinaryOp(
                Box::new(Expression::Identifier("i".to_string())),
                "+".to_string(),
                Box::new(Expression::Number(1))
            )
        )
    ]
)
```

### Example 6: For Loop

**TFI Source:**
```tfi
eega(rrr i = 0; i < 5; i + 1) {
    bahubali(i);
}
```

**Parsing Process:**
1. Matches `for_statement` rule
2. Parses initialization as statement
3. Parses condition as expression
4. Parses update as expression
5. Parses body as vector of statements
6. Creates `Statement::For` with all components

**AST Output:**
```rust
Statement::For(
    Box::new(Statement::Const("i".to_string(), Expression::Number(0))),
    Expression::BinaryOp(
        Box::new(Expression::Identifier("i".to_string())),
        "<".to_string(),
        Box::new(Expression::Number(5))
    ),
    Expression::BinaryOp(
        Box::new(Expression::Identifier("i".to_string())),
        "+".to_string(),
        Box::new(Expression::Number(1))
    ),
    vec![
        Statement::Print(vec![Expression::Identifier("i".to_string())])
    ]
)
```

## Error Handling

### Parse Error Information

The parser provides detailed error information:

```rust
#[derive(Debug, Clone)]
pub struct ParseErrorInfo {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub source_line: String,
    pub suggestion: Option<String>,
}
```

### Error Examples

**Missing Keyword Error:**
```
❌ Parse Error at line 2, column 1
   Unexpected end of input or invalid syntax
   x = 11;
   ^
   💡 Suggestion: Variable assignments need 'rrr' (const) or 'pushpa' (let) keyword
```

**Missing Semicolon Error:**
```
❌ Parse Error at line 1, column 15
   Unexpected end of input or invalid syntax
   rrr x = 42
            ^
   💡 Suggestion: Statements must end with semicolon
```

**Invalid Syntax Error:**
```
❌ Parse Error at line 1, column 8
   Unexpected token
   bahubali(;
         ^
   💡 Suggestion: bahubali statements need at least one argument
```

## Parser Implementation

### Main Parsing Function

```rust
pub fn parse_program(input: &str) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    let pairs = MyLanguageParser::parse(Rule::program, input).map_err(|e| {
        let error_info = create_error_info_from_pest(&e, input);
        eprintln!("{}", format_parse_error(&error_info));
        e
    })?;
    
    let mut statements = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::statement => {
                            let stmt = parse_statement(inner_pair)?;
                            statements.push(stmt);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok(statements)
}
```

### Statement Parsing

```rust
fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    let inner_pair = inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected statement content".to_string() },
            span,
        )
    })?;
    
    match inner_pair.as_rule() {
        Rule::print_statement => parse_print_statement(inner_pair),
        Rule::const_statement => parse_const_statement(inner_pair),
        Rule::let_statement => parse_let_statement(inner_pair),
        Rule::if_statement => parse_if_statement(inner_pair),
        Rule::while_statement => parse_while_statement(inner_pair),
        Rule::for_statement => parse_for_statement(inner_pair),
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: format!("Unknown statement type: {:?}", inner_pair.as_rule()) },
            inner_pair.as_span(),
        ))
    }
}
```

### Expression Parsing

```rust
fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression, pest::error::Error<Rule>> {
    let mut inner = pair.into_inner();
    let mut left = parse_term(inner.next().unwrap())?;
    
    while let Some(op_pair) = inner.next() {
        let op = op_pair.as_str().to_string();
        let right = parse_term(inner.next().unwrap())?;
        left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
    }
    
    Ok(left)
}
```

## Key Features

### 1. Declarative Grammar
Using Pest's declarative grammar makes the parser:
- Easy to understand and modify
- Less prone to errors
- Self-documenting

### 2. Operator Precedence
The grammar handles operator precedence correctly:
```pest
expression = { term ~ (operator ~ term)* }
```
This ensures `1 + 2 * 3` is parsed as `1 + (2 * 3)`.

### 3. Error Recovery
The parser provides:
- Line and column information
- Context about the error
- Helpful suggestions
- Graceful error handling

### 4. Extensible Design
Adding new language constructs is straightforward:
1. Add grammar rules to `grammar.pest`
2. Add parsing functions
3. Update the main parser

## Common Patterns

### Pattern 1: Optional Elements
```pest
if_statement = { 
    "magadheera" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" ~ 
    else_block?
}
```
The `?` makes the else block optional.

### Pattern 2: Repetition
```pest
print_statement = { "bahubali" ~ "(" ~ expression ~ ("," ~ expression)* ~ ")" ~ ";" }
```
The `*` allows zero or more additional expressions.

### Pattern 3: Alternatives
```pest
statement = { 
    print_statement |
    const_statement |
    let_statement |
    if_statement |
    while_statement |
    for_statement
}
```
The `|` allows any of the listed statement types.

## Testing the Parser

The parser includes comprehensive tests:

```rust
#[test]
fn test_parse_print_statement() {
    let source = r#"bahubali("Hello, world!");"#;
    let result = parse_program(source);
    assert!(result.is_ok());
    
    let statements = result.unwrap();
    assert_eq!(statements.len(), 1);
    
    if let Statement::Print(expressions) = &statements[0] {
        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0], Expression::String("Hello, world!".to_string()));
    } else {
        panic!("Expected print statement");
    }
}

#[test]
fn test_parse_if_statement() {
    let source = r#"
        magadheera(x > 5) {
            bahubali("x is greater than 5");
        }
    "#;
    let result = parse_program(source);
    assert!(result.is_ok());
    
    let statements = result.unwrap();
    assert_eq!(statements.len(), 1);
    
    if let Statement::If(condition, then_block, else_block) = &statements[0] {
        assert_eq!(else_block, &None);
        assert_eq!(then_block.len(), 1);
    } else {
        panic!("Expected if statement");
    }
}
```

## Performance Considerations

### Memory Usage
- AST nodes are created on the heap
- Large programs may use significant memory
- Consider using arena allocation for large ASTs

### Speed Optimizations
- Pest provides efficient parsing
- Grammar rules are compiled for speed
- Error recovery is optimized

## Summary

The parser is the bridge between raw tokens and structured program representation. It ensures that the source code follows the language's grammar rules and creates a clean AST for further processing.

Key takeaways:
- Parsers convert tokens into structured trees
- TFI uses Pest for declarative grammar definition
- Error handling provides helpful feedback
- The design is extensible for new features
- Operator precedence is handled correctly 