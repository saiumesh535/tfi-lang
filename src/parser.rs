use crate::ast::{Statement, Expression};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyLanguageParser;

/// Enhanced error information for better error messages
#[derive(Debug, Clone)]
pub struct ParseErrorInfo {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub source_line: String,
    pub suggestion: Option<String>,
}

/// Parse a complete TFI program into a vector of statements
pub fn parse_program(input: &str) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    let pairs = MyLanguageParser::parse(Rule::program, input).map_err(|e| {
        // Print enhanced error message
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
    
    if statements.is_empty() {
        let error_info = ParseErrorInfo {
            message: "No valid statements found. Check your syntax.".to_string(),
            line: 1,
            column: 1,
            source_line: input.lines().next().unwrap_or("").to_string(),
            suggestion: Some("Make sure your TFI file contains valid statements like 'bahubali(\"Hello\");' or 'rrr x = 10;'".to_string()),
        };
        eprintln!("{}", format_parse_error(&error_info));
        return Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { 
                message: error_info.message
            },
            pest::Span::new(input, 0, input.len()).unwrap(),
        ));
    }
    
    Ok(statements)
}

/// Create error info from pest error with basic information
fn create_error_info_from_pest(error: &pest::error::Error<Rule>, source: &str) -> ParseErrorInfo {
    // Extract basic error information
    let error_str = error.to_string();
    let lines: Vec<&str> = error_str.lines().collect();
    
    // Try to extract line and column from error message
    let mut line = 1;
    let mut column = 1;
    let mut source_line = source.lines().next().unwrap_or("").to_string();
    
    // Parse line:column from pest error format like " --> 2:1"
    if lines.len() >= 1 {
        let error_line = lines[0].trim();
        if error_line.starts_with("-->") {
            let parts: Vec<&str> = error_line.split(':').collect();
            if parts.len() >= 2 {
                if let Some(line_str) = parts[0].split_whitespace().last() {
                    if let Ok(l) = line_str.parse::<usize>() {
                        line = l;
                        source_line = source.lines().nth(line - 1).unwrap_or("").to_string();
                    }
                }
                if let Some(col_str) = parts[1].split_whitespace().next() {
                    if let Ok(c) = col_str.parse::<usize>() {
                        column = c;
                    }
                }
            }
        }
    }
    
    // Generate helpful message and suggestion
    let message = if error_str.contains("EOI") {
        "Unexpected end of input or invalid syntax".to_string()
    } else if error_str.contains("statement") {
        "Invalid statement syntax".to_string()
    } else {
        "Syntax error".to_string()
    };
    
    let suggestion = generate_generic_suggestion(&source_line);
    
    ParseErrorInfo {
        message,
        line,
        column,
        source_line,
        suggestion,
    }
}



/// Generate generic suggestions based on source line content
fn generate_generic_suggestion(source_line: &str) -> Option<String> {
    if source_line.trim().is_empty() {
        Some("Add a valid TFI statement like 'bahubali(\"Hello\");'".to_string())
    } else if source_line.contains('=') && !source_line.contains("rrr") && !source_line.contains("pushpa") {
        Some("Variable assignments need 'rrr' (const) or 'pushpa' (let) keyword".to_string())
    } else if source_line.contains("bahubali") && !source_line.contains('(') {
        Some("bahubali statements need parentheses: bahubali(\"message\");".to_string())
    } else if source_line.contains("magadheera") && !source_line.contains('(') {
        Some("magadheera statements need parentheses: magadheera(condition) { ... }".to_string())
    } else if source_line.contains("pokiri") && !source_line.contains('(') {
        Some("pokiri statements need parentheses: pokiri(condition) { ... }".to_string())
    } else if source_line.contains("eega") && !source_line.contains('(') {
        Some("eega statements need parentheses: eega(init; condition; update) { ... }".to_string())
    } else {
        Some("Check your syntax and make sure all statements end with semicolons".to_string())
    }
}

/// Format parse error with nice formatting
fn format_parse_error(error_info: &ParseErrorInfo) -> String {
    let mut output = String::new();
    output.push_str(&format!("❌ Parse Error at line {}, column {}\n", error_info.line, error_info.column));
    output.push_str(&format!("   {}\n", error_info.message));
    output.push_str(&format!("   {}\n", error_info.source_line));
    output.push_str(&format!("   {}^\n", " ".repeat(error_info.column - 1)));
    
    if let Some(ref suggestion) = error_info.suggestion {
        output.push_str(&format!("   💡 Suggestion: {}\n", suggestion));
    }
    
    output
}

/// Parse a single statement from a pest pair
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

/// Parse a print statement: bahubali(expr1, expr2, ...)
fn parse_print_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let inner = pair.into_inner();
    let mut expressions = vec![];
    
    for pair in inner {
        if pair.as_rule() == Rule::expression {
            expressions.push(parse_expression(pair)?);
        }
    }
    
    if expressions.is_empty() {
        return Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "bahubali() requires at least one argument".to_string() },
            span,
        ));
    }
    
    Ok(Statement::Print(expressions))
}

/// Parse a const declaration: rrr name = value
fn parse_const_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    
    let ident = inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected identifier in rrr declaration".to_string() },
            span,
        )
    })?.as_str().to_string();
    
    let expr = parse_expression(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected expression in rrr declaration".to_string() },
            span,
        )
    })?)?;
    
    Ok(Statement::Const(ident, expr))
}

/// Parse a let declaration: pushpa name = value
fn parse_let_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    
    let ident = inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected identifier in pushpa declaration".to_string() },
            span,
        )
    })?.as_str().to_string();
    
    let expr = parse_expression(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected expression in pushpa declaration".to_string() },
            span,
        )
    })?)?;
    
    Ok(Statement::Let(ident, expr))
}

/// Parse an if statement: magadheera(condition) { ... } karthikeya { ... }
fn parse_if_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    
    let cond = parse_expression(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected condition in magadheera statement".to_string() },
            span,
        )
    })?)?;
    
    let mut then_statements = vec![];
    let mut else_statements = None;
    
    for pair in inner {
        match pair.as_rule() {
            Rule::statement => then_statements.push(parse_statement(pair)?),
            Rule::WHITESPACE => {}
            Rule::else_block => {
                // Parse the else block
                let mut else_block = vec![];
                for stmt_pair in pair.into_inner() {
                    if stmt_pair.as_rule() == Rule::statement {
                        else_block.push(parse_statement(stmt_pair)?);
                    }
                }
                else_statements = Some(else_block);
            }
            _ => {}
        }
    }
    
    Ok(Statement::If(cond, then_statements, else_statements))
}

/// Parse a while loop: pokiri(condition) { ... }
fn parse_while_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    
    let cond = parse_expression(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected condition in pokiri statement".to_string() },
            span,
        )
    })?)?;
    
    let mut statements = vec![];
    for pair in inner {
        if pair.as_rule() == Rule::statement {
            statements.push(parse_statement(pair)?);
        }
    }
    
    Ok(Statement::While(cond, statements))
}

/// Parse a for loop: eega(init; condition; update) { ... }
fn parse_for_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    
    let init = parse_statement(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected initialization in eega statement".to_string() },
            span,
        )
    })?)?;
    
    let cond = parse_expression(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected condition in eega statement".to_string() },
            span,
        )
    })?)?;
    
    let update = parse_expression(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected update expression in eega statement".to_string() },
            span,
        )
    })?)?;
    
    let mut statements = vec![];
    for pair in inner {
        if pair.as_rule() == Rule::statement {
            statements.push(parse_statement(pair)?);
        }
    }
    
    Ok(Statement::For(Box::new(init), cond, update, statements))
}

/// Parse an expression
fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    let mut left = parse_term(inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected term in expression".to_string() },
            span,
        )
    })?)?;

    while let Some(op_pair) = inner.next() {
        if op_pair.as_rule() == Rule::operator {
            let op = op_pair.as_str().to_string();
            let right = parse_term(inner.next().ok_or_else(|| {
                pest::error::Error::new_from_span(
                    pest::error::ErrorVariant::CustomError { message: "Expected right operand".to_string() },
                    span,
                )
            })?)?;
            left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
        } else {
            return Err(pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError { message: format!("Unexpected pair in expression: {:?}", op_pair.as_rule()) },
                op_pair.as_span(),
            ));
        }
    }

    Ok(left)
}

/// Parse a term (number, identifier, string, or parenthesized expression)
fn parse_term(pair: pest::iterators::Pair<Rule>) -> Result<Expression, pest::error::Error<Rule>> {
    let span = pair.as_span();
    let mut inner = pair.into_inner();
    let inner_pair = inner.next().ok_or_else(|| {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Expected term content".to_string() },
            span,
        )
    })?;
    
    match inner_pair.as_rule() {
        Rule::number => {
            let num = inner_pair.as_str().parse().unwrap();
            Ok(Expression::Number(num))
        }
        Rule::ident => {
            let ident = inner_pair.as_str().to_string();
            Ok(Expression::Identifier(ident))
        }
        Rule::string => {
            // Remove the surrounding quotes
            let s = inner_pair.as_str();
            let s = s[1..s.len()-1].to_string();
            Ok(Expression::String(s))
        }
        Rule::expression => parse_expression(inner_pair),
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown term type".to_string() },
            inner_pair.as_span(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Statement, Expression};

    #[test]
    fn test_parse_print_statement() {
        let source = r#"bahubali("Hello, world!");"#;
        let result = parse_program(source);
        assert!(result.is_ok());
        
        let statements = result.unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::Print(expressions) = &statements[0] {
            assert_eq!(expressions.len(), 1);
            if let Expression::String(s) = &expressions[0] {
                assert_eq!(s, "Hello, world!");
            } else {
                panic!("Expected string expression");
            }
        } else {
            panic!("Expected print statement");
        }
    }

    #[test]
    fn test_parse_const_declaration() {
        let source = "rrr x = 42;";
        let result = parse_program(source);
        assert!(result.is_ok());
        
        let statements = result.unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::Const(name, expr) = &statements[0] {
            assert_eq!(name, "x");
            if let Expression::Number(n) = expr {
                assert_eq!(*n, 42);
            } else {
                panic!("Expected number expression");
            }
        } else {
            panic!("Expected const statement");
        }
    }

    #[test]
    fn test_parse_let_declaration() {
        let source = "pushpa y = 10;";
        let result = parse_program(source);
        assert!(result.is_ok());
        
        let statements = result.unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::Let(name, expr) = &statements[0] {
            assert_eq!(name, "y");
            if let Expression::Number(n) = expr {
                assert_eq!(*n, 10);
            } else {
                panic!("Expected number expression");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let source = "rrr result = 5 + 3;";
        let result = parse_program(source);
        assert!(result.is_ok());
        
        let statements = result.unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::Const(_, expr) = &statements[0] {
            if let Expression::BinaryOp(left, op, right) = expr {
                assert_eq!(op, "+");
                if let Expression::Number(n) = **left {
                    assert_eq!(n, 5);
                } else {
                    panic!("Expected left operand to be number");
                }
                if let Expression::Number(n) = **right {
                    assert_eq!(n, 3);
                } else {
                    panic!("Expected right operand to be number");
                }
            } else {
                panic!("Expected binary operation");
            }
        } else {
            panic!("Expected const statement");
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let source = r#"
            magadheera(1 > 0) {
                bahubali("true");
            }
        "#;
        let result = parse_program(source);
        assert!(result.is_ok());
        
        let statements = result.unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::If(cond, then_block, else_block) = &statements[0] {
            assert_eq!(then_block.len(), 1);
            assert!(else_block.is_none());
            
            if let Expression::BinaryOp(left, op, right) = cond {
                assert_eq!(op, ">");
                if let Expression::Number(n) = **left {
                    assert_eq!(n, 1);
                } else {
                    panic!("Expected left operand to be number");
                }
                if let Expression::Number(n) = **right {
                    assert_eq!(n, 0);
                } else {
                    panic!("Expected right operand to be number");
                }
            } else {
                panic!("Expected binary operation in condition");
            }
        } else {
            panic!("Expected if statement");
        }
    }

    #[test]
    fn test_parse_empty_program_error() {
        let source = "";
        let result = parse_program(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_syntax_error() {
        let source = "invalid syntax here";
        let result = parse_program(source);
        assert!(result.is_err());
    }
} 