# Compiler (Orchestration) - TFI Language

## What is the Compiler?

The compiler is the main orchestrator that coordinates all the other components. It manages the compilation pipeline and provides a unified interface for converting TFI code to JavaScript. Think of it as the "conductor" - it directs all the other components to work together harmoniously.

## How it Works in TFI

The compiler follows a clear pipeline that transforms TFI source code into executable JavaScript:

```rust
pub fn compile(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Parse the source code
    let ast = parse_program(source)?;
    
    // Step 2: Validate the AST
    validate_program(&ast)?;
    
    // Step 3: Generate JavaScript code
    let js_code = generate_program(&ast);
    
    Ok(js_code)
}
```

## Compilation Pipeline

### 1. Parsing: `source → AST`
The compiler takes raw TFI source code and parses it into an Abstract Syntax Tree.

### 2. Validation: `AST → validated AST`
The AST is validated for semantic correctness (variable scoping, etc.).

### 3. Generation: `validated AST → JavaScript`
The validated AST is converted to JavaScript code.

## Examples

### Example 1: Simple Program Compilation

**TFI Source:**
```tfi
bahubali("Hello, TFI!");
rrr counter = 0;
pokiri(counter < 3) {
    bahubali("Counter:", counter);
    pushpa counter = counter + 1;
}
```

**Compilation Process:**

1. **Parsing Phase:**
   ```rust
   let ast = parse_program(source)?;
   // Creates AST with 3 statements:
   // - Print statement
   // - Const declaration
   // - While loop with body
   ```

2. **Validation Phase:**
   ```rust
   validate_program(&ast)?;
   // Checks:
   // - All variables are declared
   // - Print statements have arguments
   // - Control structures have non-empty bodies
   ```

3. **Generation Phase:**
   ```rust
   let js_code = generate_program(&ast);
   // Produces JavaScript code
   ```

**Generated JavaScript:**
```javascript
console.log("Hello, TFI!");
const counter = 0;
while ((counter < 3)) {
console.log("Counter:", counter);
let counter = (counter + 1);
}
```

### Example 2: Compilation with Errors

**TFI Source:**
```tfi
rrr x = 10;
bahubali(y);  // Error: y is not declared
```

**Compilation Process:**

1. **Parsing Phase:** ✅ Success
   ```rust
   let ast = parse_program(source)?;
   // AST created successfully
   ```

2. **Validation Phase:** ❌ Error
   ```rust
   validate_program(&ast)?;
   // Error: Variable 'y' is not defined
   ```

3. **Generation Phase:** ❌ Not reached

**Error Output:**
```
⚠️  Validation Error at statement 2
   Variable 'y' is not defined
   💡 Suggestion: Declare the variable first with 'rrr y = value;' or 'pushpa y = value;'
```

### Example 3: Compilation with Options

**TFI Source:**
```tfi
rrr x = 42;
bahubali("The answer is", x);
```

**Compilation with Formatting:**
```rust
let options = CompilationOptions::new().with_formatting();
let result = compile_with_options(source, &options)?;
```

**Generated JavaScript (Formatted):**
```javascript
const x = 42;
console.log("The answer is", x);
```

## Compiler Implementation

### Main Compilation Function

```rust
pub fn compile(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = compile_with_details(source)?;
    Ok(result.js_code)
}
```

### Detailed Compilation Function

```rust
pub fn compile_with_details(source: &str) -> Result<CompilationResult, Box<dyn std::error::Error>> {
    // Step 1: Parse the source code
    let ast = parse_program(source).map_err(|e| {
        CompilationError::General {
            message: format!("Failed to parse TFI code: {}", e),
            context: Some("The parser has already printed detailed error information above".to_string()),
        }
    })?;
    
    // Step 2: Validate the AST
    validate_program(&ast).map_err(|e| {
        CompilationError::ValidationError {
            message: format!("Validation failed: {}", e),
            line: None,
            context: None,
            suggestion: None,
        }
    })?;
    
    // Step 3: Generate JavaScript code
    let js_code = generate_program(&ast);
    
    // Step 4: Create compilation result
    let mut result = CompilationResult::new(js_code, ast.len());
    
    // Add warnings for potential issues
    add_compilation_warnings(&ast, &mut result);
    
    Ok(result)
}
```

### Compilation Options

```rust
#[derive(Debug, Clone, Default)]
pub struct CompilationOptions {
    /// Format the output JavaScript code
    pub format_output: bool,
    /// Add source comments to the output
    pub add_comments: bool,
    /// Enable strict mode
    pub strict_mode: bool,
    /// Minify the output
    pub minify: bool,
}

impl CompilationOptions {
    /// Create default compilation options
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Enable formatting
    pub fn with_formatting(mut self) -> Self {
        self.format_output = true;
        self
    }
    
    /// Enable comments
    pub fn with_comments(mut self) -> Self {
        self.add_comments = true;
        self
    }
    
    /// Enable strict mode
    pub fn with_strict_mode(mut self) -> Self {
        self.strict_mode = true;
        self
    }
    
    /// Enable minification
    pub fn with_minification(mut self) -> Self {
        self.minify = true;
        self
    }
}
```

## Error Handling

### Compilation Error Types

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationError {
    /// Parsing error with source context
    ParseError {
        message: String,
        line: usize,
        column: usize,
        source_line: String,
        suggestion: Option<String>,
    },
    /// Validation error with context
    ValidationError {
        message: String,
        line: Option<usize>,
        context: Option<String>,
        suggestion: Option<String>,
    },
    /// Generation error
    GenerationError {
        message: String,
        context: Option<String>,
    },
    /// General compilation error
    General {
        message: String,
        context: Option<String>,
    },
}
```

### Error Display

```rust
impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationError::ParseError { message, line, column, source_line, suggestion } => {
                writeln!(f, "❌ Parse Error at line {}, column {}", line, column)?;
                writeln!(f, "   {}", message)?;
                writeln!(f, "   {}", source_line)?;
                write!(f, "   {}^", " ".repeat(*column - 1))?;
                if let Some(sugg) = suggestion {
                    writeln!(f, "\n   💡 Suggestion: {}", sugg)?;
                }
                Ok(())
            }
            CompilationError::ValidationError { message, line, context, suggestion } => {
                writeln!(f, "⚠️  Validation Error")?;
                if let Some(l) = line {
                    writeln!(f, "   at line {}", l)?;
                }
                writeln!(f, "   {}", message)?;
                if let Some(ctx) = context {
                    writeln!(f, "   Context: {}", ctx)?;
                }
                if let Some(sugg) = suggestion {
                    writeln!(f, "   💡 Suggestion: {}", sugg)?;
                }
                Ok(())
            }
            // ... other cases
        }
    }
}
```

## Compilation Results

### Result Structure

```rust
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Generated JavaScript code
    pub js_code: String,
    /// Compilation warnings
    pub warnings: Vec<String>,
    /// Number of statements compiled
    pub statement_count: usize,
}

impl CompilationResult {
    /// Create a new compilation result
    pub fn new(js_code: String, statement_count: usize) -> Self {
        Self {
            js_code,
            warnings: Vec::new(),
            statement_count,
        }
    }
    
    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    
    /// Get the number of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}
```

### Compilation Statistics

```rust
pub struct CompilationStats {
    /// Total number of statements
    pub total_statements: usize,
    /// Number of print statements
    pub print_statements: usize,
    /// Number of const declarations
    pub const_declarations: usize,
    /// Number of let declarations
    pub let_declarations: usize,
    /// Number of if statements
    pub if_statements: usize,
    /// Number of while loops
    pub while_loops: usize,
    /// Number of for loops
    pub for_loops: usize,
}

impl CompilationStats {
    pub fn total_declarations(&self) -> usize {
        self.const_declarations + self.let_declarations
    }
    
    pub fn total_control_structures(&self) -> usize {
        self.if_statements + self.while_loops + self.for_loops
    }
    
    pub fn summary(&self) -> String {
        format!(
            "Compilation Summary:\n\
             - Total statements: {}\n\
             - Print statements: {}\n\
             - Variable declarations: {}\n\
             - Control structures: {}",
            self.total_statements,
            self.print_statements,
            self.total_declarations(),
            self.total_control_structures()
        )
    }
}
```

## Advanced Features

### 1. Compilation with Options

```rust
pub fn compile_with_options(
    source: &str,
    options: &CompilationOptions
) -> Result<CompilationResult, Box<dyn std::error::Error>> {
    let mut result = compile_with_details(source)?;
    
    // Apply options
    if options.format_output {
        result.js_code = format_js_code(&result.js_code);
    }
    
    if options.add_comments {
        result.js_code = add_source_comments(&result.js_code, source);
    }
    
    Ok(result)
}
```

### 2. Code Formatting

```rust
fn format_js_code(js_code: &str) -> String {
    let mut formatted = String::new();
    let mut indent_level = 0;
    
    for line in js_code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }
        
        // Decrease indent for closing braces
        if trimmed.starts_with('}') {
            indent_level = indent_level.saturating_sub(1);
        }
        
        // Add indentation
        formatted.push_str(&"    ".repeat(indent_level));
        formatted.push_str(trimmed);
        formatted.push('\n');
        
        // Increase indent for opening braces
        if trimmed.ends_with('{') {
            indent_level += 1;
        }
    }
    
    formatted
}
```

### 3. Source Comments

```rust
fn add_source_comments(js_code: &str, source: &str) -> String {
    let mut commented = String::new();
    commented.push_str("// Generated from TFI source code\n");
    commented.push_str("// Original source:\n");
    
    for (i, line) in source.lines().enumerate() {
        if !line.trim().is_empty() {
            commented.push_str(&format!("// {}: {}\n", i + 1, line));
        }
    }
    
    commented.push_str("\n");
    commented.push_str(js_code);
    commented
}
```

## Testing the Compiler

The compiler includes comprehensive tests:

```rust
#[test]
fn test_basic_compilation() {
    let source = r#"
        bahubali("Hello, world!");
        rrr x = 10;
        pushpa y = 5;
        bahubali("The value of x is", x);
        bahubali(x + y);
    "#;
    
    let result = compile(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("console.log"));
    assert!(js_code.contains("const x = 10"));
    assert!(js_code.contains("let y = 5"));
}

#[test]
fn test_compilation_with_details() {
    let source = "rrr x = 42; bahubali(x);";
    let result = compile_with_details(source);
    assert!(result.is_ok());
    
    let details = result.unwrap();
    assert_eq!(details.statement_count, 2);
    assert!(!details.has_warnings());
}

#[test]
fn test_compilation_with_options() {
    let source = "rrr x = 42;";
    let options = CompilationOptions::new().with_formatting().with_comments();
    let result = compile_with_options(source, &options);
    assert!(result.is_ok());
    
    let details = result.unwrap();
    assert!(details.js_code.contains("// Generated from TFI"));
}
```

## Performance Considerations

### Memory Usage
- AST is shared between phases
- Minimal memory overhead
- Efficient error handling

### Speed Optimizations
- Early error detection
- Efficient pipeline
- Optional features

## Common Patterns

### Pattern 1: Error Propagation
```rust
let ast = parse_program(source).map_err(|e| {
    CompilationError::General {
        message: format!("Failed to parse TFI code: {}", e),
        context: Some("Parser error".to_string()),
    }
})?;
```

### Pattern 2: Result Handling
```rust
match compile(source) {
    Ok(js_code) => {
        println!("Compilation successful!");
        println!("Generated JavaScript:\n{}", js_code);
    }
    Err(e) => {
        eprintln!("Compilation failed: {}", e);
    }
}
```

### Pattern 3: Options Builder
```rust
let options = CompilationOptions::new()
    .with_formatting()
    .with_comments()
    .with_strict_mode();
```

## Summary

The compiler orchestrates the entire compilation process, providing a unified interface for converting TFI code to JavaScript. It handles errors gracefully, provides detailed feedback, and supports various compilation options.

Key takeaways:
- Compilers coordinate multiple phases
- Error handling is crucial for user experience
- Options provide flexibility
- Statistics help with analysis
- Performance matters for large programs 