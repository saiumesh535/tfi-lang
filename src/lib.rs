pub mod lexer;
pub mod ast;
pub mod parser;
pub mod generator;
pub mod validator;
pub mod compiler;

pub use ast::{Statement, Expression};
pub use compiler::compile;
pub use parser::parse_program;
pub use validator::validate_program;
pub use generator::{generate_statement, generate_expression};

/// Main compilation function that takes TFI source code and returns JavaScript
pub fn compile_tfi_to_js(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    compiler::compile(source)
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
        
        let result = compile_tfi_to_js(source);
        assert!(result.is_ok());
        
        let js_code = result.unwrap();
        assert!(js_code.contains("console.log"));
        assert!(js_code.contains("const x = 10"));
        assert!(js_code.contains("let y = 5"));
    }

    #[test]
    fn test_if_statement() {
        let source = r#"
            rrr x = 10;
            magadheera(x > 5) {
                bahubali("x is greater than 5");
            }
        "#;
        
        let result = compile_tfi_to_js(source);
        assert!(result.is_ok());
        
        let js_code = result.unwrap();
        assert!(js_code.contains("if"));
        assert!(js_code.contains("console.log"));
    }

    #[test]
    fn test_while_loop() {
        let source = r#"
            rrr i = 0;
            pokiri(i < 3) {
                bahubali(i);
                pushpa j = i + 1;
            }
        "#;
        
        let result = compile_tfi_to_js(source);
        if let Err(e) = &result {
            eprintln!("While loop compilation error: {}", e);
        }
        assert!(result.is_ok());
        
        let js_code = result.unwrap();
        assert!(js_code.contains("while"));
    }

    #[test]
    fn test_for_loop() {
        let source = r#"
            eega(rrr i = 0; i < 3; i + 1) {
                bahubali(i);
            }
        "#;
        
        let result = compile_tfi_to_js(source);
        assert!(result.is_ok());
        
        let js_code = result.unwrap();
        assert!(js_code.contains("for"));
    }

    #[test]
    fn test_empty_print_error() {
        let source = "bahubali();";
        let result = compile_tfi_to_js(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_if_block_error() {
        let source = r#"
            magadheera(1 > 0) {
            }
        "#;
        let result = compile_tfi_to_js(source);
        assert!(result.is_err());
    }




} 