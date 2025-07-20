use crate::ast::{Statement, Expression};

/// Generate JavaScript code from a TFI statement
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

/// Generate JavaScript code from a TFI expression
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

/// Generate complete JavaScript program from a vector of statements
pub fn generate_program(statements: &[Statement]) -> String {
    statements.iter().map(generate_statement).collect::<Vec<_>>().join("\n")
}

/// Generate formatted JavaScript code with proper indentation
pub fn generate_formatted_statement(stmt: &Statement, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);
    let code = generate_statement(stmt);
    
    // Add indentation to each line
    code.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate formatted JavaScript program
pub fn generate_formatted_program(statements: &[Statement]) -> String {
    statements.iter()
        .map(|stmt| generate_formatted_statement(stmt, 0))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Statement, Expression};

    #[test]
    fn test_generate_number_expression() {
        let expr = Expression::Number(42);
        assert_eq!(generate_expression(&expr), "42");
    }

    #[test]
    fn test_generate_identifier_expression() {
        let expr = Expression::Identifier("x".to_string());
        assert_eq!(generate_expression(&expr), "x");
    }

    #[test]
    fn test_generate_string_expression() {
        let expr = Expression::String("hello".to_string());
        assert_eq!(generate_expression(&expr), "\"hello\"");
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
    fn test_generate_complex_binary_expression() {
        let expr = Expression::BinaryOp(
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number(1)),
                "+".to_string(),
                Box::new(Expression::Number(2))
            )),
            "*".to_string(),
            Box::new(Expression::Number(3))
        );
        assert_eq!(generate_expression(&expr), "((1 + 2) * 3)");
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
    fn test_generate_const_statement() {
        let stmt = Statement::Const("x".to_string(), Expression::Number(10));
        assert_eq!(generate_statement(&stmt), "const x = 10;");
    }

    #[test]
    fn test_generate_let_statement() {
        let stmt = Statement::Let("y".to_string(), Expression::String("hello".to_string()));
        assert_eq!(generate_statement(&stmt), "let y = \"hello\";");
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

    #[test]
    fn test_generate_if_else_statement() {
        let stmt = Statement::If(
            Expression::BinaryOp(
                Box::new(Expression::Identifier("x".to_string())),
                ">".to_string(),
                Box::new(Expression::Number(0))
            ),
            vec![
                Statement::Print(vec![Expression::String("positive".to_string())])
            ],
            Some(vec![
                Statement::Print(vec![Expression::String("negative".to_string())])
            ])
        );
        
        let expected = r#"if ((x > 0)) {
console.log("positive");
} else {
console.log("negative");
}"#;
        assert_eq!(generate_statement(&stmt), expected);
    }

    #[test]
    fn test_generate_while_statement() {
        let stmt = Statement::While(
            Expression::BinaryOp(
                Box::new(Expression::Identifier("i".to_string())),
                "<".to_string(),
                Box::new(Expression::Number(10))
            ),
            vec![
                Statement::Print(vec![Expression::Identifier("i".to_string())]),
                Statement::Let("i".to_string(), Expression::BinaryOp(
                    Box::new(Expression::Identifier("i".to_string())),
                    "+".to_string(),
                    Box::new(Expression::Number(1))
                ))
            ]
        );
        
        let expected = r#"while ((i < 10)) {
console.log(i);
let i = (i + 1);
}"#;
        assert_eq!(generate_statement(&stmt), expected);
    }

    #[test]
    fn test_generate_for_statement() {
        let stmt = Statement::For(
            Box::new(Statement::Let("i".to_string(), Expression::Number(0))),
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
        );
        
        let expected = r#"for (let i = 0; (i < 5); (i + 1)) {
console.log(i);
}"#;
        assert_eq!(generate_statement(&stmt), expected);
    }

    #[test]
    fn test_generate_program() {
        let statements = vec![
            Statement::Const("x".to_string(), Expression::Number(10)),
            Statement::Let("y".to_string(), Expression::Number(5)),
            Statement::Print(vec![Expression::String("sum".to_string()), Expression::BinaryOp(
                Box::new(Expression::Identifier("x".to_string())),
                "+".to_string(),
                Box::new(Expression::Identifier("y".to_string()))
            )])
        ];
        
        let expected = r#"const x = 10;
let y = 5;
console.log("sum", (x + y));"#;
        assert_eq!(generate_program(&statements), expected);
    }

    #[test]
    fn test_generate_formatted_statement() {
        let stmt = Statement::If(
            Expression::Number(1),
            vec![
                Statement::Print(vec![Expression::String("true".to_string())])
            ],
            None
        );
        
                let expected = r#"    if (1) {
    console.log("true");
    }"#;
        assert_eq!(generate_formatted_statement(&stmt, 1), expected);
    }
} 