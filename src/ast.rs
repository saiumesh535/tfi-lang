/// Abstract Syntax Tree nodes for the TFI language
#[derive(Debug, Clone, PartialEq)]
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

/// Expression nodes for the TFI language
#[derive(Debug, Clone, PartialEq)]
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

impl Statement {
    /// Get the statement type as a string for debugging
    pub fn statement_type(&self) -> &'static str {
        match self {
            Statement::Print(_) => "Print",
            Statement::Const(_, _) => "Const",
            Statement::Let(_, _) => "Let",
            Statement::If(_, _, _) => "If",
            Statement::While(_, _) => "While",
            Statement::For(_, _, _, _) => "For",
        }
    }
}

impl Expression {
    /// Get the expression type as a string for debugging
    pub fn expression_type(&self) -> &'static str {
        match self {
            Expression::Number(_) => "Number",
            Expression::Identifier(_) => "Identifier",
            Expression::String(_) => "String",
            Expression::BinaryOp(_, _, _) => "BinaryOp",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_types() {
        let print_stmt = Statement::Print(vec![Expression::Number(42)]);
        assert_eq!(print_stmt.statement_type(), "Print");

        let const_stmt = Statement::Const("x".to_string(), Expression::Number(10));
        assert_eq!(const_stmt.statement_type(), "Const");

        let if_stmt = Statement::If(
            Expression::Number(1),
            vec![Statement::Print(vec![Expression::String("hello".to_string())])],
            None
        );
        assert_eq!(if_stmt.statement_type(), "If");
    }

    #[test]
    fn test_expression_types() {
        let num_expr = Expression::Number(42);
        assert_eq!(num_expr.expression_type(), "Number");

        let id_expr = Expression::Identifier("x".to_string());
        assert_eq!(id_expr.expression_type(), "Identifier");

        let str_expr = Expression::String("hello".to_string());
        assert_eq!(str_expr.expression_type(), "String");

        let bin_expr = Expression::BinaryOp(
            Box::new(Expression::Number(1)),
            "+".to_string(),
            Box::new(Expression::Number(2))
        );
        assert_eq!(bin_expr.expression_type(), "BinaryOp");
    }

    #[test]
    fn test_expression_equality() {
        let expr1 = Expression::Number(42);
        let expr2 = Expression::Number(42);
        let expr3 = Expression::Number(43);
        
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_statement_equality() {
        let stmt1 = Statement::Print(vec![Expression::Number(42)]);
        let stmt2 = Statement::Print(vec![Expression::Number(42)]);
        let stmt3 = Statement::Print(vec![Expression::Number(43)]);
        
        assert_eq!(stmt1, stmt2);
        assert_ne!(stmt1, stmt3);
    }
} 