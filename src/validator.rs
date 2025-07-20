use crate::ast::{Statement, Expression};

/// Validation error types
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

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyPrintStatement(line) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   bahubali() requires at least one argument")?;
                writeln!(f, "   💡 Suggestion: bahubali(\"Hello, world!\");")
            }
            ValidationError::EmptyIdentifier(line, stmt_type) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   {} declaration requires a valid identifier", stmt_type)?;
                writeln!(f, "   💡 Suggestion: {} variable_name = value;", stmt_type)
            }
            ValidationError::EmptyBlock(line, stmt_type) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   {} block cannot be empty", stmt_type)?;
                writeln!(f, "   💡 Suggestion: {} (condition) {{ bahubali(\"action\"); }}", stmt_type)
            }
            ValidationError::InvalidExpression(line, msg) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   {}", msg)
            }
            ValidationError::DuplicateVariable(name, line) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   Variable '{}' is already declared", name)?;
                writeln!(f, "   💡 Suggestion: Use a different variable name or redeclare with 'pushpa'")
            }
            ValidationError::UndefinedVariable(name, line) => {
                writeln!(f, "⚠️  Validation Error at statement {}", line)?;
                writeln!(f, "   Variable '{}' is not defined", name)?;
                writeln!(f, "   💡 Suggestion: Declare the variable first with 'rrr {} = value;' or 'pushpa {} = value;'", name, name)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Variable declaration type
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationType {
    Const,
    Let,
}

/// Validation context for tracking variables and other state
#[derive(Debug, Default)]
pub struct ValidationContext {
    /// Set of declared variables
    declared_vars: std::collections::HashSet<String>,
    /// Map of variable names to their declaration line
    var_declarations: std::collections::HashMap<String, usize>,
    /// Map of variable names to their declaration type
    var_types: std::collections::HashMap<String, DeclarationType>,
}

impl ValidationContext {
    /// Create a new validation context
    pub fn new() -> Self {
        Self {
            declared_vars: std::collections::HashSet::new(),
            var_declarations: std::collections::HashMap::new(),
            var_types: std::collections::HashMap::new(),
        }
    }
    
    /// Declare a variable
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
    
    /// Check if a variable is declared
    pub fn is_variable_declared(&self, name: &str) -> bool {
        self.declared_vars.contains(name)
    }
    
    /// Get all declared variables
    pub fn get_declared_variables(&self) -> &std::collections::HashSet<String> {
        &self.declared_vars
    }
}

/// Validate a complete TFI program
pub fn validate_program(statements: &[Statement]) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = ValidationContext::new();
    
    for (i, stmt) in statements.iter().enumerate() {
        validate_statement(stmt, i + 1, &mut context)?;
    }
    
    Ok(())
}

/// Validate a single statement
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
        Statement::If(cond, then_block, else_block) => {
            validate_expression(cond, line, context)?;
            
            if then_block.is_empty() {
                return Err(ValidationError::EmptyBlock(line, "magadheera".to_string()));
            }
            
            // Create a new scope for the if block
            let mut if_context = ValidationContext::new();
            if_context.declared_vars.extend(context.declared_vars.clone());
            if_context.var_declarations.extend(context.var_declarations.clone());
            if_context.var_types.extend(context.var_types.clone());
            
            for stmt in then_block {
                validate_statement(stmt, line, &mut if_context)?;
            }
            
            if let Some(else_block) = else_block {
                if else_block.is_empty() {
                    return Err(ValidationError::EmptyBlock(line, "karthikeya".to_string()));
                }
                
                // Create a new scope for the else block
                let mut else_context = ValidationContext::new();
                else_context.declared_vars.extend(context.declared_vars.clone());
                else_context.var_declarations.extend(context.var_declarations.clone());
                else_context.var_types.extend(context.var_types.clone());
                
                for stmt in else_block {
                    validate_statement(stmt, line, &mut else_context)?;
                }
            }
        }
        Statement::While(cond, block) => {
            validate_expression(cond, line, context)?;
            
            if block.is_empty() {
                return Err(ValidationError::EmptyBlock(line, "pokiri".to_string()));
            }
            
            // Create a new scope for the while block
            let mut while_context = ValidationContext::new();
            while_context.declared_vars.extend(context.declared_vars.clone());
            while_context.var_declarations.extend(context.var_declarations.clone());
            while_context.var_types.extend(context.var_types.clone());
            
            for stmt in block {
                validate_statement(stmt, line, &mut while_context)?;
            }
        }
        Statement::For(init, cond, update, block) => {
            validate_statement(init, line, context)?;
            validate_expression(cond, line, context)?;
            validate_expression(update, line, context)?;
            
            if block.is_empty() {
                return Err(ValidationError::EmptyBlock(line, "eega".to_string()));
            }
            
            // Create a new scope for the for block
            let mut for_context = ValidationContext::new();
            for_context.declared_vars.extend(context.declared_vars.clone());
            for_context.var_declarations.extend(context.var_declarations.clone());
            for_context.var_types.extend(context.var_types.clone());
            
            for stmt in block {
                validate_statement(stmt, line, &mut for_context)?;
            }
        }
    }
    
    Ok(())
}

/// Validate an expression
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
        Expression::BinaryOp(left, op, right) => {
            validate_expression(left, line, context)?;
            validate_expression(right, line, context)?;
            
            // Validate operator
            match op.as_str() {
                "+" | "-" | "*" | "/" | ">" | "<" | ">=" | "<=" | "==" | "!=" => Ok(()),
                _ => Err(ValidationError::InvalidExpression(line, format!("Unknown operator: {}", op)))
            }
        }
    }
}

/// Validate a program with detailed error reporting
pub fn validate_program_detailed(statements: &[Statement]) -> Result<(), Vec<ValidationError>> {
    let mut context = ValidationContext::new();
    let mut errors = Vec::new();
    
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Statement, Expression};

    #[test]
    fn test_validate_empty_print_error() {
        let stmt = Statement::Print(vec![]);
        let mut context = ValidationContext::new();
        let result = validate_statement(&stmt, 1, &mut context);
        assert!(result.is_err());
        
        if let Err(ValidationError::EmptyPrintStatement(line)) = result {
            assert_eq!(line, 1);
        } else {
            panic!("Expected EmptyPrintStatement error");
        }
    }

    #[test]
    fn test_validate_empty_identifier_error() {
        let stmt = Statement::Const("".to_string(), Expression::Number(42));
        let mut context = ValidationContext::new();
        let result = validate_statement(&stmt, 1, &mut context);
        assert!(result.is_err());
        
        if let Err(ValidationError::EmptyIdentifier(line, stmt_type)) = result {
            assert_eq!(line, 1);
            assert_eq!(stmt_type, "rrr");
        } else {
            panic!("Expected EmptyIdentifier error");
        }
    }

    #[test]
    fn test_validate_empty_if_block_error() {
        let stmt = Statement::If(
            Expression::Number(1),
            vec![],
            None
        );
        let mut context = ValidationContext::new();
        let result = validate_statement(&stmt, 1, &mut context);
        assert!(result.is_err());
        
        if let Err(ValidationError::EmptyBlock(line, stmt_type)) = result {
            assert_eq!(line, 1);
            assert_eq!(stmt_type, "magadheera");
        } else {
            panic!("Expected EmptyBlock error");
        }
    }

    #[test]
    fn test_validate_duplicate_variable_error() {
        let statements = vec![
            Statement::Let("x".to_string(), Expression::Number(1)),
            Statement::Const("x".to_string(), Expression::Number(2)),
        ];
        let result = validate_program(&statements);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_undefined_variable_error() {
        let stmt = Statement::Print(vec![Expression::Identifier("undefined_var".to_string())]);
        let mut context = ValidationContext::new();
        let result = validate_statement(&stmt, 1, &mut context);
        assert!(result.is_err());
        
        if let Err(ValidationError::UndefinedVariable(name, line)) = result {
            assert_eq!(name, "undefined_var");
            assert_eq!(line, 1);
        } else {
            panic!("Expected UndefinedVariable error");
        }
    }

    #[test]
    fn test_validate_valid_program() {
        let statements = vec![
            Statement::Const("x".to_string(), Expression::Number(10)),
            Statement::Let("y".to_string(), Expression::Number(5)),
            Statement::Print(vec![
                Expression::String("sum".to_string()),
                Expression::BinaryOp(
                    Box::new(Expression::Identifier("x".to_string())),
                    "+".to_string(),
                    Box::new(Expression::Identifier("y".to_string()))
                )
            ])
        ];
        
        let result = validate_program(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_context_methods() {
        let mut context = ValidationContext::new();
        
        // Test variable declaration
        assert!(context.declare_variable("x", 1, DeclarationType::Const).is_ok());
        assert!(context.is_variable_declared("x"));
        assert!(!context.is_variable_declared("y"));
        
        // Test duplicate declaration (let to const should fail)
        let result = context.declare_variable("x", 2, DeclarationType::Const);
        assert!(result.is_err());
        
        if let Err(ValidationError::DuplicateVariable(name, line)) = result {
            assert_eq!(name, "x");
            assert_eq!(line, 1);
        } else {
            panic!("Expected DuplicateVariable error");
        }
    }

    #[test]
    fn test_validate_expression() {
        let mut context = ValidationContext::new();
        context.declare_variable("x", 1, DeclarationType::Const).unwrap();
        
        // Valid expressions
        let valid_expr = Expression::BinaryOp(
            Box::new(Expression::Identifier("x".to_string())),
            "+".to_string(),
            Box::new(Expression::Number(5))
        );
        assert!(validate_expression(&valid_expr, 1, &context).is_ok());
        
        // Valid operator (now that * is supported)
        let valid_expr = Expression::BinaryOp(
            Box::new(Expression::Number(1)),
            "*".to_string(),
            Box::new(Expression::Number(2))
        );
        assert!(validate_expression(&valid_expr, 1, &context).is_ok());
        
        // Invalid operator
        let invalid_expr = Expression::BinaryOp(
            Box::new(Expression::Number(1)),
            "&".to_string(),
            Box::new(Expression::Number(2))
        );
        let result = validate_expression(&invalid_expr, 1, &context);
        assert!(result.is_err());
        
        if let Err(ValidationError::InvalidExpression(_, msg)) = result {
            assert!(msg.contains("Unknown operator"));
        } else {
            panic!("Expected InvalidExpression error");
        }
    }

    #[test]
    fn test_validate_detailed() {
        let statements = vec![
            Statement::Print(vec![]), // Error 1
            Statement::Const("x".to_string(), Expression::Number(1)),
            Statement::Const("x".to_string(), Expression::Number(2)), // Error 2
            Statement::Print(vec![Expression::Identifier("undefined".to_string())]), // Error 3
        ];
        
        let result = validate_program_detailed(&statements);
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert_eq!(errors.len(), 3);
            
            // Check that we have the expected error types
            let error_types: Vec<&str> = errors.iter()
                .map(|e| match e {
                    ValidationError::EmptyPrintStatement(_) => "EmptyPrintStatement",
                    ValidationError::DuplicateVariable(_, _) => "DuplicateVariable",
                    ValidationError::UndefinedVariable(_, _) => "UndefinedVariable",
                    _ => "Other",
                })
                .collect();
            
            assert!(error_types.contains(&"EmptyPrintStatement"));
            assert!(error_types.contains(&"DuplicateVariable"));
            assert!(error_types.contains(&"UndefinedVariable"));
        } else {
            panic!("Expected error list");
        }
    }
} 