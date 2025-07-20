# Validator (Semantic Analysis) - TFI Language

## What is a Validator?

A validator performs semantic analysis to ensure the program is logically correct. It checks for issues that can't be caught by syntax analysis alone, such as undefined variables, duplicate declarations, and semantic constraints. Think of it as the "logic checker" - it ensures the program makes sense beyond just having correct syntax.

## How it Works in TFI

The TFI validator performs several types of checks to ensure semantic correctness:

### 1. Variable Scoping
- **Declaration Tracking**: Keeps track of all declared variables
- **Scope Management**: Handles nested scopes in if/while/for blocks
- **Shadowing Rules**: Allows `let` to shadow `const` variables

### 2. Semantic Validation
- **Empty Blocks**: Ensures control structures have non-empty bodies
- **Empty Print**: Ensures `bahubali()` has at least one argument
- **Variable References**: Ensures all variables are declared before use

### 3. Error Reporting
Provides detailed error messages with suggestions for fixing issues.

## Validation Context

The validator uses a context to track program state:

```rust
#[derive(Debug, Default)]
pub struct ValidationContext {
    /// Set of declared variables
    declared_vars: std::collections::HashSet<String>,
    /// Map of variable names to their declaration line
    var_declarations: std::collections::HashMap<String, usize>,
    /// Map of variable names to their declaration type
    var_types: std::collections::HashMap<String, DeclarationType>,
}
```

## Examples

### Example 1: Valid Variable Declaration and Usage

**TFI Source:**
```tfi
rrr x = 10;
bahubali(x);
```

**Validation Process:**
1. ✅ `x` is declared as const on line 1
2. ✅ `x` is used in print statement on line 2
3. ✅ Print statement has at least one argument

**Validation Context:**
```rust
ValidationContext {
    declared_vars: {"x"},
    var_declarations: {"x" -> 1},
    var_types: {"x" -> DeclarationType::Const}
}
```

### Example 2: Undefined Variable Error

**TFI Source:**
```tfi
rrr x = 10;
bahubali(y);  // Error: y is not declared
```

**Validation Error:**
```
⚠️  Validation Error at statement 2
   Variable 'y' is not defined
   💡 Suggestion: Declare the variable first with 'rrr y = value;' or 'pushpa y = value;'
```

**Validation Context:**
```rust
ValidationContext {
    declared_vars: {"x"},
    var_declarations: {"x" -> 1},
    var_types: {"x" -> DeclarationType::Const}
}
```

### Example 3: Variable Shadowing

**TFI Source:**
```tfi
rrr x = 10;
pushpa x = 20;  // Valid: let can shadow const
bahubali(x);
```

**Validation Process:**
1. ✅ `x` is declared as const on line 1
2. ✅ `x` is redeclared as let on line 2 (valid shadowing)
3. ✅ `x` is used in print statement on line 3

**Validation Context:**
```rust
ValidationContext {
    declared_vars: {"x"},
    var_declarations: {"x" -> 2},  // Updated to line 2
    var_types: {"x" -> DeclarationType::Let}  // Updated to let
}
```

### Example 4: Duplicate Declaration Error

**TFI Source:**
```tfi
rrr x = 10;
rrr x = 20;  // Error: x already declared
```

**Validation Error:**
```
⚠️  Validation Error at statement 2
   Variable 'x' is already declared
   💡 Suggestion: Use a different variable name or redeclare with 'pushpa'
```

### Example 5: Empty Block Error

**TFI Source:**
```tfi
magadheera(x > 5) {
    // Empty block - error!
}
```

**Validation Error:**
```
⚠️  Validation Error at statement 1
   magadheera block cannot be empty
   💡 Suggestion: magadheera (condition) { bahubali("action"); }
```

### Example 6: Empty Print Error

**TFI Source:**
```tfi
bahubali();  // Error: no arguments
```

**Validation Error:**
```
⚠️  Validation Error at statement 1
   bahubali() requires at least one argument
   💡 Suggestion: bahubali("Hello, world!");
```

### Example 7: Nested Scopes

**TFI Source:**
```tfi
rrr x = 10;
magadheera(x > 5) {
    pushpa y = 20;
    bahubali(x, y);
}
bahubali(y);  // Error: y not in scope
```

**Validation Process:**
1. ✅ `x` is declared as const on line 1
2. ✅ `x` is used in condition on line 2
3. ✅ `y` is declared as let in if block on line 3
4. ✅ `x` and `y` are used in print on line 4
5. ❌ `y` is used outside its scope on line 6

**Validation Error:**
```
⚠️  Validation Error at statement 4
   Variable 'y' is not defined
   💡 Suggestion: Declare the variable first with 'rrr y = value;' or 'pushpa y = value;'
```

## Error Types

The validator defines several error types:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Empty print statement
    EmptyPrintStatement(usize),
    /// Empty identifier in declaration
    EmptyIdentifier(usize, String),
    /// Empty block in control structure
    EmptyBlock(usize, String),
    /// Invalid expression type
    InvalidExpression(usize, String),
    /// Duplicate variable declaration
    DuplicateVariable(String, usize),
    /// Undefined variable reference
    UndefinedVariable(String, usize),
}
```

## Validation Implementation

### Main Validation Function

```rust
pub fn validate_program(statements: &[Statement]) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = ValidationContext::new();
    
    for (i, stmt) in statements.iter().enumerate() {
        validate_statement(stmt, i + 1, &mut context)?;
    }
    
    Ok(())
}
```

### Statement Validation

```rust
fn validate_statement(
    stmt: &Statement, 
    line: usize, 
    context: &mut ValidationContext
) -> Result<(), ValidationError> {
    match stmt {
        Statement::Print(expressions) => {
            if expressions.is_empty() {
                return Err(ValidationError::EmptyPrintStatement(line));
            }
            
            for expr in expressions {
                validate_expression(expr, line, context)?;
            }
        }
        Statement::Const(name, expr) => {
            if name.is_empty() {
                return Err(ValidationError::EmptyIdentifier(line, "rrr".to_string()));
            }
            
            context.declare_variable(name, line, DeclarationType::Const)?;
            validate_expression(expr, line, context)?;
        }
        Statement::Let(name, expr) => {
            if name.is_empty() {
                return Err(ValidationError::EmptyIdentifier(line, "pushpa".to_string()));
            }
            
            context.declare_variable(name, line, DeclarationType::Let)?;
            validate_expression(expr, line, context)?;
        }
        // ... other statement types
    }
    
    Ok(())
}
```

### Expression Validation

```rust
fn validate_expression(
    expr: &Expression, 
    line: usize, 
    context: &ValidationContext
) -> Result<(), ValidationError> {
    match expr {
        Expression::Number(_) => Ok(()),
        Expression::String(_) => Ok(()),
        Expression::Identifier(name) => {
            if !context.is_variable_declared(name) {
                return Err(ValidationError::UndefinedVariable(name.clone(), line));
            }
            Ok(())
        }
        Expression::BinaryOp(left, _, right) => {
            validate_expression(left, line, context)?;
            validate_expression(right, line, context)?;
            Ok(())
        }
    }
}
```

## Variable Scoping Rules

### 1. Declaration Types

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationType {
    Const,
    Let,
}
```

### 2. Shadowing Rules

- **Const → Let**: Allowed (let can shadow const)
- **Let → Let**: Not allowed (duplicate declaration)
- **Const → Const**: Not allowed (duplicate declaration)
- **Let → Const**: Not allowed (const cannot shadow let)

### 3. Scope Management

```rust
impl ValidationContext {
    pub fn declare_variable(&mut self, name: &str, line: usize, decl_type: DeclarationType) -> Result<(), ValidationError> {
        if self.declared_vars.contains(name) {
            let original_line = self.var_declarations.get(name).unwrap_or(&0);
            let original_type = self.var_types.get(name).unwrap_or(&DeclarationType::Let);
            
            // Allow redeclaration if the original is const and new is let (shadowing)
            if *original_type == DeclarationType::Const && decl_type == DeclarationType::Let {
                // This is valid shadowing
                self.var_declarations.insert(name.to_string(), line);
                self.var_types.insert(name.to_string(), decl_type);
                return Ok(());
            }
            
            return Err(ValidationError::DuplicateVariable(name.to_string(), *original_line));
        }
        
        self.declared_vars.insert(name.to_string());
        self.var_declarations.insert(name.to_string(), line);
        self.var_types.insert(name.to_string(), decl_type);
        Ok(())
    }
}
```

## Key Features

### 1. Comprehensive Checking
The validator catches semantic errors early:
- Undefined variables
- Duplicate declarations
- Empty blocks and statements
- Invalid expressions

### 2. Helpful Messages
Error messages include:
- Exact line numbers
- Clear descriptions
- Actionable suggestions
- Context information

### 3. Scope Awareness
The validator understands:
- Variable scoping rules
- Shadowing behavior
- Nested block scopes
- Declaration order

### 4. Performance
Efficient validation using:
- Hash sets for fast lookups
- Minimal memory usage
- Early error detection

## Common Patterns

### Pattern 1: Variable Declaration Tracking
```rust
let mut context = ValidationContext::new();
context.declare_variable("x", 1, DeclarationType::Const)?;
assert!(context.is_variable_declared("x"));
```

### Pattern 2: Expression Validation
```rust
fn validate_expression(expr: &Expression, context: &ValidationContext) -> Result<(), ValidationError> {
    match expr {
        Expression::Identifier(name) => {
            if !context.is_variable_declared(name) {
                return Err(ValidationError::UndefinedVariable(name.clone(), line));
            }
            Ok(())
        }
        // ... other cases
    }
}
```

### Pattern 3: Block Scope Management
```rust
// Create new scope for block
let mut block_context = ValidationContext::new();
block_context.declared_vars.extend(context.declared_vars.clone());
block_context.var_declarations.extend(context.var_declarations.clone());
block_context.var_types.extend(context.var_types.clone());

// Validate block statements
for stmt in block {
    validate_statement(stmt, line, &mut block_context)?;
}
```

## Testing the Validator

The validator includes comprehensive tests:

```rust
#[test]
fn test_validate_empty_print_error() {
    let source = vec![Statement::Print(vec![])];
    let result = validate_program(&source);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("bahubali() requires at least one argument"));
    }
}

#[test]
fn test_validate_undefined_variable_error() {
    let source = vec![
        Statement::Print(vec![Expression::Identifier("x".to_string())])
    ];
    let result = validate_program(&source);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("Variable 'x' is not defined"));
    }
}

#[test]
fn test_validate_valid_program() {
    let source = vec![
        Statement::Const("x".to_string(), Expression::Number(10)),
        Statement::Print(vec![Expression::Identifier("x".to_string())])
    ];
    let result = validate_program(&source);
    assert!(result.is_ok());
}
```

## Error Recovery

### 1. Multiple Error Collection
The validator can collect multiple errors:
```rust
pub fn validate_program_detailed(statements: &[Statement]) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut context = ValidationContext::new();
    
    for (i, stmt) in statements.iter().enumerate() {
        if let Err(e) = validate_statement(stmt, i + 1, &mut context) {
            errors.push(e);
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

### 2. Error Suggestions
Each error includes helpful suggestions:
```rust
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UndefinedVariable(name, line) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   Variable '{}' is not defined", name)?;
                writeln!(f, "   💡 Suggestion: Declare the variable first with 'rrr {} = value;' or 'pushpa {} = value;'", name, name)
            }
            // ... other cases
        }
    }
}
```

## Performance Considerations

### Memory Usage
- Hash sets provide O(1) lookups
- Context is shared across validation
- Minimal memory overhead

### Speed Optimizations
- Early error detection
- Efficient scope management
- Fast variable lookups

## Summary

The validator ensures semantic correctness by checking variable scoping, declaration rules, and semantic constraints. It provides detailed error messages and helpful suggestions for fixing issues.

Key takeaways:
- Validators check logical correctness
- Variable scoping is crucial for semantic analysis
- Error messages should be helpful and actionable
- Shadowing rules must be clearly defined
- Performance is important for large programs 