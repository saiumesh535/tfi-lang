# Generator (Code Generation) - TFI Language

## What is a Generator?

A generator (or code generator) takes the validated AST and produces target code. In TFI's case, it generates JavaScript code that can be executed in a browser or Node.js environment. Think of it as the "translator" - it converts the structured program representation into executable code.

## How it Works in TFI

The generator traverses the AST and converts each node to equivalent JavaScript code. It follows a recursive pattern, visiting each node and generating the appropriate JavaScript syntax.

## Code Generation Process

### 1. AST Traversal
The generator walks through the AST tree structure, visiting each node.

### 2. Code Translation
Each AST node is translated to equivalent JavaScript code.

### 3. Output Formatting
The generated code is formatted for readability and proper syntax.

## Examples

### Example 1: Simple Variable Declaration

**TFI Source:**
```tfi
rrr x = 42;
```

**AST Input:**
```rust
Statement::Const("x".to_string(), Expression::Number(42))
```

**Generated JavaScript:**
```javascript
const x = 42;
```

**Generation Process:**
1. Match `Statement::Const` pattern
2. Extract identifier "x"
3. Generate expression "42"
4. Combine with `const` keyword and semicolon

### Example 2: Print Statement

**TFI Source:**
```tfi
bahubali("Hello, world!");
```

**AST Input:**
```rust
Statement::Print(vec![Expression::String("Hello, world!".to_string())])
```

**Generated JavaScript:**
```javascript
console.log("Hello, world!");
```

**Generation Process:**
1. Match `Statement::Print` pattern
2. Generate `console.log(` prefix
3. Generate expression arguments
4. Add closing parenthesis and semicolon

### Example 3: Binary Expression

**TFI Source:**
```tfi
rrr result = x + y;
```

**AST Input:**
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

**Generated JavaScript:**
```javascript
const result = (x + y);
```

**Generation Process:**
1. Generate `const result = `
2. Recursively generate left expression "x"
3. Add operator "+"
4. Recursively generate right expression "y"
5. Wrap in parentheses for precedence
6. Add semicolon

### Example 4: If Statement

**TFI Source:**
```tfi
magadheera(x > 5) {
    bahubali("x is greater than 5");
} karthikeya {
    bahubali("x is 5 or less");
}
```

**AST Input:**
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

**Generated JavaScript:**
```javascript
if ((x > 5)) {
console.log("x is greater than 5");
} else {
console.log("x is 5 or less");
}
```

**Generation Process:**
1. Generate `if (` prefix
2. Generate condition expression `(x > 5)`
3. Add `) {` opening
4. Generate then block statements
5. Add `} else {` separator
6. Generate else block statements
7. Add closing `}`

### Example 5: While Loop

**TFI Source:**
```tfi
pokiri(i < 10) {
    bahubali(i);
    pushpa i = i + 1;
}
```

**AST Input:**
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

**Generated JavaScript:**
```javascript
while ((i < 10)) {
console.log(i);
let i = (i + 1);
}
```

**Generation Process:**
1. Generate `while (` prefix
2. Generate condition expression `(i < 10)`
3. Add `) {` opening
4. Generate body statements
5. Add closing `}`

### Example 6: For Loop

**TFI Source:**
```tfi
eega(rrr i = 0; i < 5; i + 1) {
    bahubali(i);
}
```

**AST Input:**
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

**Generated JavaScript:**
```javascript
for (const i = 0; (i < 5); (i + 1)) {
console.log(i);
}
```

**Generation Process:**
1. Generate `for (` prefix
2. Generate initialization statement (without semicolon)
3. Add semicolon separator
4. Generate condition expression
5. Add semicolon separator
6. Generate update expression
7. Add `) {` opening
8. Generate body statements
9. Add closing `}`

## Generator Implementation

### Statement Generation

```rust
pub fn generate_statement(stmt: &Statement) -> String {
    match stmt {
        Statement::Print(expressions) => {
            let args = expressions.iter().map(generate_expression).collect::<Vec<_>>().join(", ");
            format!("console.log({});", args)
        },
        Statement::Const(id, expr) => format!("const {} = {};", id, generate_expression(expr)),
        Statement::Let(id, expr) => format!("let {} = {};", id, generate_expression(expr)),
        Statement::If(cond, then_block, else_block) => {
            let then_code = then_block.iter().map(generate_statement).collect::<Vec<_>>().join("\n");
            let else_code = else_block.as_ref().map(|block| {
                format!(" else {{\n{}\n}}", block.iter().map(generate_statement).collect::<Vec<_>>().join("\n"))
            }).unwrap_or_default();
            format!("if ({}) {{\n{}\n}}{}", generate_expression(cond), then_code, else_code)
        },
        Statement::While(cond, block) => {
            let block_code = block.iter().map(generate_statement).collect::<Vec<_>>().join("\n");
            format!("while ({}) {{\n{}\n}}", generate_expression(cond), block_code)
        },
        Statement::For(init, cond, update, block) => {
            let init_code = generate_statement(init);
            let cond_code = generate_expression(cond);
            let update_code = generate_expression(update);
            let block_code = block.iter().map(generate_statement).collect::<Vec<_>>().join("\n");
            format!("for ({}; {}; {}) {{\n{}\n}}", init_code.trim_end_matches(';'), cond_code, update_code, block_code)
        },
    }
}
```

### Expression Generation

```rust
pub fn generate_expression(expr: &Expression) -> String {
    match expr {
        Expression::Number(n) => n.to_string(),
        Expression::Identifier(id) => id.clone(),
        Expression::String(s) => format!("\"{}\"", s),
        Expression::BinaryOp(left, op, right) => {
            format!("({} {} {})", generate_expression(left), op, generate_expression(right))
        },
    }
}
```

### Program Generation

```rust
pub fn generate_program(statements: &[Statement]) -> String {
    statements.iter().map(generate_statement).collect::<Vec<_>>().join("\n")
}
```

## Key Features

### 1. Clean Output
The generator produces readable JavaScript code:
- Proper indentation
- Consistent formatting
- Clear structure

### 2. Proper Formatting
Generated code maintains structure:
- Block statements are properly nested
- Expressions are parenthesized for precedence
- Statements are properly terminated

### 3. Expression Handling
Binary expressions are handled correctly:
- Operator precedence is preserved with parentheses
- Complex expressions are properly structured
- Type safety is maintained

### 4. Control Flow
Control structures are generated correctly:
- If/else statements with proper blocks
- While loops with condition and body
- For loops with initialization, condition, and update

## Common Patterns

### Pattern 1: Recursive Generation
```rust
fn generate_expression(expr: &Expression) -> String {
    match expr {
        Expression::BinaryOp(left, op, right) => {
            format!("({} {} {})", 
                generate_expression(left), 
                op, 
                generate_expression(right))
        }
        // ... other cases
    }
}
```

### Pattern 2: Block Generation
```rust
fn generate_block(statements: &[Statement]) -> String {
    statements.iter()
        .map(generate_statement)
        .collect::<Vec<_>>()
        .join("\n")
}
```

### Pattern 3: Conditional Generation
```rust
fn generate_if_statement(cond: &Expression, then_block: &[Statement], else_block: Option<&[Statement]>) -> String {
    let then_code = generate_block(then_block);
    let else_code = else_block.map(|block| {
        format!(" else {{\n{}\n}}", generate_block(block))
    }).unwrap_or_default();
    
    format!("if ({}) {{\n{}\n}}{}", generate_expression(cond), then_code, else_code)
}
```

## Advanced Features

### 1. Formatted Output

```rust
pub fn generate_formatted_statement(stmt: &Statement, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);
    let code = generate_statement(stmt);
    
    // Add indentation to each line
    code.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}
```

### 2. Minified Output

```rust
pub fn generate_minified_program(statements: &[Statement]) -> String {
    statements.iter()
        .map(|stmt| generate_statement(stmt).replace('\n', ""))
        .collect::<Vec<_>>()
        .join("")
}
```

### 3. Source Maps

```rust
pub fn generate_with_source_maps(statements: &[Statement], source: &str) -> (String, String) {
    let js_code = generate_program(statements);
    let source_map = create_source_map(statements, source);
    (js_code, source_map)
}
```

## Testing the Generator

The generator includes comprehensive tests:

```rust
#[test]
fn test_generate_number_expression() {
    let expr = Expression::Number(42);
    assert_eq!(generate_expression(&expr), "42");
}

#[test]
fn test_generate_binary_expression() {
    let expr = Expression::BinaryOp(
        Box::new(Expression::Number(5)),
        "+".to_string(),
        Box::new(Expression::Number(3))
    );
    assert_eq!(generate_expression(&expr), "(5 + 3)");
}

#[test]
fn test_generate_print_statement() {
    let stmt = Statement::Print(vec![
        Expression::String("Hello".to_string()),
        Expression::Number(42)
    ]);
    assert_eq!(generate_statement(&stmt), "console.log(\"Hello\", 42);");
}

#[test]
fn test_generate_if_statement() {
    let stmt = Statement::If(
        Expression::BinaryOp(
            Box::new(Expression::Identifier("x".to_string())),
            ">".to_string(),
            Box::new(Expression::Number(0))
        ),
        vec![
            Statement::Print(vec![Expression::String("positive".to_string())])
        ],
        None
    );
    
    let expected = r#"if ((x > 0)) {
console.log("positive");
}"#;
    assert_eq!(generate_statement(&stmt), expected);
}
```

## Performance Considerations

### Memory Usage
- String concatenation is efficient
- Minimal memory overhead
- No unnecessary allocations

### Speed Optimizations
- Recursive generation is fast
- String operations are optimized
- No complex transformations

## Error Handling

### 1. Invalid AST Handling
```rust
fn generate_statement(stmt: &Statement) -> Result<String, GenerationError> {
    match stmt {
        Statement::Print(expressions) => {
            if expressions.is_empty() {
                return Err(GenerationError::EmptyPrint);
            }
            let args = expressions.iter()
                .map(generate_expression)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("console.log({});", args.join(", ")))
        }
        // ... other cases
    }
}
```

### 2. Expression Validation
```rust
fn validate_expression(expr: &Expression) -> Result<(), String> {
    match expr {
        Expression::BinaryOp(left, op, right) => {
            validate_expression(left)?;
            validate_expression(right)?;
            if !is_valid_operator(op) {
                return Err(format!("Invalid operator: {}", op));
            }
            Ok(())
        }
        // ... other cases
    }
}
```

## Summary

The generator is the final step in the compilation process, converting the validated AST into executable JavaScript code. It ensures proper syntax, formatting, and structure while maintaining the semantic meaning of the original TFI program.

Key takeaways:
- Generators convert AST to target code
- Recursive traversal is the standard approach
- Proper formatting improves readability
- Error handling ensures robust generation
- Performance is important for large programs 