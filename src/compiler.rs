use crate::parser::parse_program;
use crate::validator::validate_program;
use crate::generator::generate_program;

/// Enhanced compilation error types with better context
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
            CompilationError::GenerationError { message, context } => {
                writeln!(f, "🔧 Generation Error")?;
                writeln!(f, "   {}", message)?;
                if let Some(ctx) = context {
                    writeln!(f, "   Context: {}", ctx)?;
                }
                Ok(())
            }
            CompilationError::General { message, context } => {
                writeln!(f, "❌ Compilation Error")?;
                writeln!(f, "   {}", message)?;
                if let Some(ctx) = context {
                    writeln!(f, "   Context: {}", ctx)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for CompilationError {}

/// Compilation result with optional warnings
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

/// Compile TFI source code to JavaScript
pub fn compile(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = compile_with_details(source)?;
    Ok(result.js_code)
}

/// Compile TFI source code to JavaScript with detailed results
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
            line: None, // Placeholder, will be updated by validator
            context: None, // Placeholder, will be updated by validator
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

/// Add warnings for potential issues in the code
fn add_compilation_warnings(statements: &[crate::ast::Statement], result: &mut CompilationResult) {
    for (i, stmt) in statements.iter().enumerate() {
        match stmt {
            crate::ast::Statement::Print(expressions) => {
                if expressions.len() > 5 {
                    result.add_warning(format!(
                        "Statement {}: Print statement has {} arguments, consider breaking it up",
                        i + 1, expressions.len()
                    ));
                }
            }
            crate::ast::Statement::While(_, block) => {
                if block.len() > 10 {
                    result.add_warning(format!(
                        "Statement {}: While loop has {} statements, consider refactoring",
                        i + 1, block.len()
                    ));
                }
            }
            crate::ast::Statement::For(_, _, _, block) => {
                if block.len() > 10 {
                    result.add_warning(format!(
                        "Statement {}: For loop has {} statements, consider refactoring",
                        i + 1, block.len()
                    ));
                }
            }
            _ => {}
        }
    }
}

/// Compile TFI source code with specific options
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

/// Compilation options
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

/// Format JavaScript code with proper indentation
fn format_js_code(js_code: &str) -> String {
    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    let indent_size = 4;
    
    for line in js_code.lines() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }
        
        // Decrease indent for closing braces
        if trimmed.starts_with('}') {
            indent_level = indent_level.saturating_sub(1usize);
        }
        
        // Add indentation
        let indent = " ".repeat(indent_level * indent_size);
        formatted.push_str(&format!("{}{}\n", indent, trimmed));
        
        // Increase indent for opening braces
        if trimmed.ends_with('{') {
            indent_level += 1;
        }
    }
    
    formatted
}

/// Add source comments to JavaScript code
fn add_source_comments(js_code: &str, source: &str) -> String {
    let mut commented = String::new();
    commented.push_str("// Generated from TFI source code\n");
    commented.push_str("// Original source:\n");
    
    for (i, line) in source.lines().enumerate() {
        if !line.trim().is_empty() {
            commented.push_str(&format!("// {}: {}\n", i + 1, line.trim()));
        }
    }
    
    commented.push_str("\n");
    commented.push_str(js_code);
    
    commented
}

/// Get compilation statistics
pub fn get_compilation_stats(source: &str) -> Result<CompilationStats, Box<dyn std::error::Error>> {
    let ast = parse_program(source)?;
    
    let mut stats = CompilationStats::default();
    stats.total_statements = ast.len();
    
    for stmt in &ast {
        count_statement_recursive(stmt, &mut stats);
    }
    
    Ok(stats)
}

/// Recursively count statements in the AST
fn count_statement_recursive(stmt: &crate::ast::Statement, stats: &mut CompilationStats) {
    match stmt {
        crate::ast::Statement::Print(_) => stats.print_statements += 1,
        crate::ast::Statement::Const(_, _) => stats.const_declarations += 1,
        crate::ast::Statement::Let(_, _) => stats.let_declarations += 1,
        crate::ast::Statement::If(_, then_block, else_block) => {
            stats.if_statements += 1;
            for stmt in then_block {
                count_statement_recursive(stmt, stats);
            }
            if let Some(else_block) = else_block {
                for stmt in else_block {
                    count_statement_recursive(stmt, stats);
                }
            }
        }
        crate::ast::Statement::While(_, block) => {
            stats.while_loops += 1;
            for stmt in block {
                count_statement_recursive(stmt, stats);
            }
        }
        crate::ast::Statement::For(_, _, _, block) => {
            stats.for_loops += 1;
            for stmt in block {
                count_statement_recursive(stmt, stats);
            }
        }
    }
}

/// Compilation statistics
#[derive(Debug, Clone, Default)]
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
    /// Get the total number of variable declarations
    pub fn total_declarations(&self) -> usize {
        self.const_declarations + self.let_declarations
    }
    
    /// Get the total number of control structures
    pub fn total_control_structures(&self) -> usize {
        self.if_statements + self.while_loops + self.for_loops
    }
    
    /// Get a summary string
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let source = r#"
            bahubali("Hello");
            rrr x = 42;
        "#;
        
        let result = compile_with_details(source);
        assert!(result.is_ok());
        
        let details = result.unwrap();
        assert_eq!(details.statement_count, 2);
        assert!(!details.has_warnings());
        assert!(details.js_code.contains("console.log"));
    }

    #[test]
    fn test_compilation_with_options() {
        let source = "bahubali(\"Hello\");";
        
        let options = CompilationOptions::new()
            .with_formatting()
            .with_comments();
        
        let result = compile_with_options(source, &options);
        assert!(result.is_ok());
        
        let details = result.unwrap();
        assert!(details.js_code.contains("// Generated from TFI"));
        assert!(details.js_code.contains("console.log"));
    }

    #[test]
    fn test_compilation_stats() {
        let source = r#"
            bahubali("Hello");
            rrr x = 10;
            pushpa y = 5;
            magadheera(x > 5) {
                bahubali("x is greater than 5");
            }
            pokiri(y < 10) {
                bahubali(y);
                pushpa y = y + 1;
            }
        "#;
        
        let stats = get_compilation_stats(source);
        if let Err(e) = &stats {
            eprintln!("Compilation stats error: {}", e);
        }
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        eprintln!("Stats: {:?}", stats);
        assert_eq!(stats.total_statements, 5);
        assert_eq!(stats.print_statements, 3); // Fixed: there are 3 print statements
        assert_eq!(stats.const_declarations, 1);
        assert_eq!(stats.let_declarations, 2); // Fixed: there are 2 let declarations (one outside, one inside while)
        assert_eq!(stats.if_statements, 1);
        assert_eq!(stats.while_loops, 1);
        assert_eq!(stats.for_loops, 0);
    }

    #[test]
    fn test_compilation_error_handling() {
        let source = "invalid syntax here";
        let result = compile(source);
        assert!(result.is_err());
        
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(error_msg.contains("Failed to parse TFI code"));
        } else {
            panic!("Expected compilation error");
        }
    }

    #[test]
    fn test_compilation_options_builder() {
        let options = CompilationOptions::new()
            .with_formatting()
            .with_comments()
            .with_strict_mode();
        
        assert!(options.format_output);
        assert!(options.add_comments);
        assert!(options.strict_mode);
        assert!(!options.minify);
    }

    #[test]
    fn test_compilation_stats_methods() {
        let mut stats = CompilationStats::default();
        stats.total_statements = 10;
        stats.print_statements = 3;
        stats.const_declarations = 2;
        stats.let_declarations = 1;
        stats.if_statements = 2;
        stats.while_loops = 1;
        stats.for_loops = 1;
        
        assert_eq!(stats.total_declarations(), 3);
        assert_eq!(stats.total_control_structures(), 4);
        
        let summary = stats.summary();
        assert!(summary.contains("Total statements: 10"));
        assert!(summary.contains("Print statements: 3"));
        assert!(summary.contains("Variable declarations: 3"));
        assert!(summary.contains("Control structures: 4"));
    }

    #[test]
    fn test_format_js_code() {
        let js_code = "if (x > 0) {\nconsole.log(x);\n}";
        let formatted = format_js_code(js_code);
        
        assert!(formatted.contains("if (x > 0) {"));
        assert!(formatted.contains("console.log(x);"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_add_source_comments() {
        let js_code = "console.log('hello');";
        let source = "bahubali(\"hello\");";
        let commented = add_source_comments(js_code, source);
        
        assert!(commented.contains("// Generated from TFI source code"));
        assert!(commented.contains("// 1: bahubali(\"hello\");"));
        assert!(commented.contains("console.log('hello');"));
    }


} 