use tfi_lang::*;

#[test]
fn test_basic_compilation_workflow() {
    let source = r#"
        bahubali("Hello, TFI World!");
        rrr x = 42;
        pushpa y = 10;
        bahubali("x + y =", x + y);
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("console.log"));
    assert!(js_code.contains("const x = 42"));
    assert!(js_code.contains("let y = 10"));
    assert!(js_code.contains("(x + y)"));
}

#[test]
fn test_if_statement_compilation() {
    let source = r#"
        rrr x = 15;
        magadheera(x > 10) {
            bahubali("x is greater than 10");
        }
        karthikeya {
            bahubali("x is less than or equal to 10");
        }
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("if"));
    assert!(js_code.contains("else"));
    assert!(js_code.contains("(x > 10)"));
}

#[test]
fn test_while_loop_compilation() {
    let source = r#"
        rrr i = 0;
        pokiri(i < 3) {
            bahubali("i =", i);
            pushpa i = i + 1;
        }
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("while"));
    assert!(js_code.contains("(i < 3)"));
}

#[test]
fn test_for_loop_compilation() {
    let source = r#"
        eega(rrr i = 0; i < 5; i + 1) {
            bahubali("i =", i);
        }
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("for"));
    assert!(js_code.contains("(i < 5)"));
}

#[test]
fn test_nested_control_structures() {
    let source = r#"
        rrr x = 5;
        magadheera(x > 0) {
            pokiri(x > 0) {
                bahubali("x =", x);
                pushpa x = x - 1;
            }
        }
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("if"));
    assert!(js_code.contains("while"));
}

#[test]
fn test_complex_expressions() {
    let source = r#"
        rrr a = 10;
        rrr b = 5;
        rrr c = 3;
        bahubali("Result:", a + b * c);
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("const a = 10"));
    assert!(js_code.contains("const b = 5"));
    assert!(js_code.contains("const c = 3"));
}

#[test]
fn test_string_literals() {
    let source = r#"
        bahubali("Hello", "World", "from", "TFI");
        rrr message = "This is a test message";
        bahubali(message);
    "#;
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    assert!(js_code.contains("\"Hello\""));
    assert!(js_code.contains("\"World\""));
    assert!(js_code.contains("\"This is a test message\""));
}

#[test]
fn test_error_handling_empty_print() {
    let source = "bahubali();";
    let result = compile_tfi_to_js(source);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_empty_if_block() {
    let source = r#"
        magadheera(1 > 0) {
        }
    "#;
    let result = compile_tfi_to_js(source);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_invalid_syntax() {
    let source = "invalid syntax here";
    let result = compile_tfi_to_js(source);
    assert!(result.is_err());
}

#[test]
fn test_ast_creation_and_manipulation() {
    let print_stmt = Statement::Print(vec![
        Expression::String("Hello".to_string()),
        Expression::Number(42)
    ]);
    
    let const_stmt = Statement::Const("x".to_string(), Expression::Number(10));
    let let_stmt = Statement::Let("y".to_string(), Expression::String("world".to_string()));
    
    let statements = vec![print_stmt, const_stmt, let_stmt];
    
    // Test that we can generate code from manually created AST
    let js_code = tfi_lang::generator::generate_program(&statements);
    assert!(js_code.contains("console.log"));
    assert!(js_code.contains("const x = 10"));
    assert!(js_code.contains("let y = \"world\""));
}

#[test]
fn test_expression_generation() {
    let expr = Expression::BinaryOp(
        Box::new(Expression::BinaryOp(
            Box::new(Expression::Number(1)),
            "+".to_string(),
            Box::new(Expression::Number(2))
        )),
        "*".to_string(),
        Box::new(Expression::Number(3))
    );
    
    let js_expr = generate_expression(&expr);
    assert_eq!(js_expr, "((1 + 2) * 3)");
}

#[test]
fn test_validation_context() {
    use crate::validator::ValidationContext;
    
    let mut context = ValidationContext::new();
    
    // Test variable declaration
    assert!(context.declare_variable("x", 1, tfi_lang::validator::DeclarationType::Const).is_ok());
    assert!(context.is_variable_declared("x"));
    assert!(!context.is_variable_declared("y"));
    
    // Test duplicate declaration (const to let shadowing should be allowed)
    let result = context.declare_variable("x", 2, tfi_lang::validator::DeclarationType::Let);
    assert!(result.is_ok()); // Const to let shadowing is allowed
    
    // Test duplicate declaration (let to const should fail)
    let result = context.declare_variable("y", 1, tfi_lang::validator::DeclarationType::Let);
    assert!(result.is_ok());
    let result = context.declare_variable("y", 2, tfi_lang::validator::DeclarationType::Const);
    assert!(result.is_err()); // Let to const shadowing is not allowed
}

#[test]
fn test_compilation_with_options() {
    use crate::compiler::{compile_with_options, CompilationOptions};
    
    let source = r#"
        bahubali("Hello");
        rrr x = 42;
    "#;
    
    let options = CompilationOptions::new()
        .with_formatting()
        .with_comments();
    
    let result = compile_with_options(source, &options);
    assert!(result.is_ok());
    
    let details = result.unwrap();
    assert!(details.js_code.contains("// Generated from TFI"));
    assert!(details.js_code.contains("console.log"));
    assert!(details.js_code.contains("const x = 42"));
}

#[test]
fn test_compilation_statistics() {
    use crate::compiler::get_compilation_stats;
    
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
    assert!(stats.is_ok());
    
    let stats = stats.unwrap();
    assert_eq!(stats.total_statements, 5);
    assert_eq!(stats.print_statements, 3);
    assert_eq!(stats.const_declarations, 1);
    assert_eq!(stats.let_declarations, 2); // Fixed: there are 2 let declarations (one outside, one inside while)
    assert_eq!(stats.if_statements, 1);
    assert_eq!(stats.while_loops, 1);
    assert_eq!(stats.for_loops, 0);
    
    let summary = stats.summary();
    assert!(summary.contains("Total statements: 5"));
    assert!(summary.contains("Print statements: 3"));
    assert!(summary.contains("Variable declarations: 3")); // Fixed: 1 const + 2 let = 3 total
    assert!(summary.contains("Control structures: 2"));
}

#[test]
fn test_lexer_functionality() {
    use crate::lexer::{Lexer, Token};
    
    let source = "rrr x = 42; bahubali(x);";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.current(), Some(&Token::Const));
    lexer.advance();
    assert_eq!(lexer.current(), Some(&Token::Identifier("x".to_string())));
    lexer.advance();
    assert_eq!(lexer.current(), Some(&Token::Assign));
    lexer.advance();
    assert_eq!(lexer.current(), Some(&Token::Number(42)));
}

#[test]
fn test_parser_functionality() {
    let source = "rrr x = 42; bahubali(x);";
    let result = parse_program(source);
    assert!(result.is_ok());
    
    let statements = result.unwrap();
    assert_eq!(statements.len(), 2);
    
    // Check first statement (const declaration)
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
    
    // Check second statement (print)
    if let Statement::Print(expressions) = &statements[1] {
        assert_eq!(expressions.len(), 1);
        if let Expression::Identifier(name) = &expressions[0] {
            assert_eq!(name, "x");
        } else {
            panic!("Expected identifier expression");
        }
    } else {
        panic!("Expected print statement");
    }
}

#[test]
fn test_end_to_end_compilation() {
    let source = "bahubali(\"Starting TFI program...\");\nrrr max_count = 5;\npushpa current = 0;\npokiri(current < max_count) {\n    bahubali(\"Current value:\", current);\n    pushpa current = current + 1;\n}\nmagadheera(current > 0) {\n    bahubali(\"Loop completed successfully!\");\n}\nkarthikeya {\n    bahubali(\"Something went wrong!\");\n}\nbahubali(\"Program finished.\");";
    
    let result = compile_tfi_to_js(source);
    assert!(result.is_ok());
    
    let js_code = result.unwrap();
    
    // Verify all expected JavaScript constructs are present
    assert!(js_code.contains("console.log"));
    assert!(js_code.contains("const max_count = 5"));
    assert!(js_code.contains("let current = 0"));
    assert!(js_code.contains("while"));
    assert!(js_code.contains("if"));
    assert!(js_code.contains("else"));
    assert!(js_code.contains("(current < max_count)"));
    assert!(js_code.contains("(current == max_count)"));
    assert!(js_code.contains("(current + 1)"));
} 