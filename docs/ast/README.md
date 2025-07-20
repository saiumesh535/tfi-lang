# Abstract Syntax Tree (AST) - TFI Language

## What is an AST?

An Abstract Syntax Tree is a tree representation of the source code's structure. It abstracts away syntax details (like parentheses and semicolons) and focuses on the logical structure of the program. The AST is the heart of the compiler - it's what gets passed between the parser, validator, and generator.

## AST Structure in TFI

### Statement Nodes

```rust
pub enum Statement {
    /// Print statement: bahubali(expr1, expr2, ...)
    Print(Vec<Expression>),
    /// Const declaration: rrr name = value
    Const(String, Expression),
    /// Let declaration: pushpa name = value
    Let(String, Expression),
    /// If statement: magadheera(condition) { ... } karthikeya { ... }
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    /// While loop: pokiri(condition) { ... }
    While(Expression, Vec<Statement>),
    /// For loop: eega(init; condition; update) { ... }
    For(Box<Statement>, Expression, Expression, Vec<Statement>),
}
```

### Expression Nodes

```rust
pub enum Expression {
    /// Numeric literal
    Number(i32),
    /// Variable identifier
    Identifier(String),
    /// String literal
    String(String),
    /// Binary operation: left op right
    BinaryOp(Box<Expression>, String, Box<Expression>),
}
```

## Examples

### Example 1: Simple Variable Declaration

**TFI Source:**
```tfi
rrr x = 42;
```

**AST Representation:**
```rust
Statement::Const("x".to_string(), Expression::Number(42))
```

**Visual Tree:**
```
Statement::Const
├── Identifier: "x"
└── Expression::Number(42)
```

### Example 2: Print Statement

**TFI Source:**
```tfi
bahubali("Hello, world!");
```

**AST Representation:**
```rust
Statement::Print(vec![Expression::String("Hello, world!".to_string())])
```

**Visual Tree:**
```
Statement::Print
└── Expression::String("Hello, world!")
```

### Example 3: Binary Expression

**TFI Source:**
```tfi
rrr result = x + y;
```

**AST Representation:**
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

**Visual Tree:**
```
Statement::Const
├── Identifier: "result"
└── Expression::BinaryOp
    ├── Left: Expression::Identifier("x")
    ├── Operator: "+"
    └── Right: Expression::Identifier("y")
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

**AST Representation:**
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

**Visual Tree:**
```
Statement::If
├── Condition: Expression::BinaryOp
│   ├── Left: Expression::Identifier("x")
│   ├── Operator: ">"
│   └── Right: Expression::Number(5)
├── Then Block: Vec<Statement>
│   └── Statement::Print
│       └── Expression::String("x is greater than 5")
└── Else Block: Option<Vec<Statement>>
    └── Statement::Print
        └── Expression::String("x is 5 or less")
```

### Example 5: While Loop

**TFI Source:**
```tfi
pokiri(i < 10) {
    bahubali(i);
    pushpa i = i + 1;
}
```

**AST Representation:**
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

**Visual Tree:**
```
Statement::While
├── Condition: Expression::BinaryOp
│   ├── Left: Expression::Identifier("i")
│   ├── Operator: "<"
│   └── Right: Expression::Number(10)
└── Body: Vec<Statement>
    ├── Statement::Print
    │   └── Expression::Identifier("i")
    └── Statement::Let
        ├── Identifier: "i"
        └── Expression::BinaryOp
            ├── Left: Expression::Identifier("i")
            ├── Operator: "+"
            └── Right: Expression::Number(1)
```

### Example 6: For Loop

**TFI Source:**
```tfi
eega(rrr i = 0; i < 5; i + 1) {
    bahubali(i);
}
```

**AST Representation:**
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

**Visual Tree:**
```
Statement::For
├── Initialization: Statement::Const
│   ├── Identifier: "i"
│   └── Expression::Number(0)
├── Condition: Expression::BinaryOp
│   ├── Left: Expression::Identifier("i")
│   ├── Operator: "<"
│   └── Right: Expression::Number(5)
├── Update: Expression::BinaryOp
│   ├── Left: Expression::Identifier("i")
│   ├── Operator: "+"
│   └── Right: Expression::Number(1)
└── Body: Vec<Statement>
    └── Statement::Print
        └── Expression::Identifier("i")
```

## Key Benefits of AST

### 1. Language Agnostic
The AST can be used to generate any target language, not just JavaScript. You could easily modify the generator to output Python, C++, or any other language.

### 2. Optimization Friendly
The tree structure makes it easy to:
- Analyze program flow
- Detect unused variables
- Optimize expressions
- Transform code

### 3. Debugging Support
The AST provides a clear representation of program structure, making it easier to:
- Understand what the code does
- Debug compilation issues
- Generate meaningful error messages

### 4. Type Safety
Using Rust enums ensures that:
- All possible statement types are handled
- Expression types are properly matched
- Compile-time checking prevents runtime errors

## AST Traversal

The AST is traversed in different ways by different components:

- **Validator**: Walks the tree to check semantic correctness
- **Generator**: Walks the tree to produce target code
- **Analyzer**: Walks the tree to gather statistics or perform optimizations

## Common Patterns

### Pattern 1: Expression Evaluation
```rust
fn evaluate_expression(expr: &Expression) -> i32 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Identifier(id) => get_variable_value(id),
        Expression::BinaryOp(left, op, right) => {
            let left_val = evaluate_expression(left);
            let right_val = evaluate_expression(right);
            match op.as_str() {
                "+" => left_val + right_val,
                "-" => left_val - right_val,
                "*" => left_val * right_val,
                "/" => left_val / right_val,
                _ => panic!("Unknown operator: {}", op),
            }
        }
    }
}
```

### Pattern 2: Statement Execution
```rust
fn execute_statement(stmt: &Statement) {
    match stmt {
        Statement::Print(expressions) => {
            let values: Vec<String> = expressions.iter()
                .map(|expr| format_expression(expr))
                .collect();
            println!("{}", values.join(" "));
        }
        Statement::Const(name, expr) => {
            let value = evaluate_expression(expr);
            declare_constant(name, value);
        }
        Statement::Let(name, expr) => {
            let value = evaluate_expression(expr);
            declare_variable(name, value);
        }
        // ... other statements
    }
}
```

## Testing the AST

The AST module includes comprehensive tests to ensure correctness:

```rust
#[test]
fn test_statement_types() {
    let print_stmt = Statement::Print(vec![Expression::Number(42)]);
    assert_eq!(print_stmt.statement_type(), "Print");

    let const_stmt = Statement::Const("x".to_string(), Expression::Number(10));
    assert_eq!(const_stmt.statement_type(), "Const");
}

#[test]
fn test_expression_equality() {
    let expr1 = Expression::Number(42);
    let expr2 = Expression::Number(42);
    let expr3 = Expression::Number(43);
    
    assert_eq!(expr1, expr2);
    assert_ne!(expr1, expr3);
}
```

## Summary

The AST is the central data structure that connects all components of the compiler. It provides a clean, structured representation of the source code that can be easily analyzed, validated, and transformed into target code. The use of Rust enums ensures type safety and makes the code more maintainable. 