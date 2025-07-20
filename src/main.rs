
use std::fs;
use std::env;
use std::path::Path;
use tfi_lang::compiler::{compile, compile_with_options, CompilationOptions, get_compilation_stats};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let (input_file, output_file, options) = parse_arguments(&args)?;
    
    // Validate input file
    if !input_file.ends_with(".tfi") {
        eprintln!("Error: Input file must have a .tfi extension (e.g., main.tfi)");
        std::process::exit(1);
    }
    
    // Read source file
    let source = fs::read_to_string(&input_file)?;
    
    // Compile with options
    let result = if options.format_output || options.add_comments {
        compile_with_options(&source, &options)?
    } else {
        compile(&source).map(|js_code| {
            tfi_lang::compiler::CompilationResult::new(js_code, 0)
        })?
    };
    
    // Write output
    fs::write(&output_file, &result.js_code)?;
    println!("Compiled successfully! Output written to: {}", output_file);
    
    // Print warnings if any
    if result.has_warnings() {
        eprintln!("Compilation warnings:");
        for warning in &result.warnings {
            eprintln!("  {}", warning);
        }
    }
    
    // Print compilation stats
    if let Ok(stats) = get_compilation_stats(&source) {
        println!("{}", stats.summary());
    }
    
    // Execute the generated JavaScript
    let output = std::process::Command::new("node")
        .arg(&output_file)
        .output()?;
    
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

/// Parse command line arguments
fn parse_arguments(args: &[String]) -> Result<(String, String, CompilationOptions), Box<dyn std::error::Error>> {
    let mut input_file = "main.tfi".to_string();
    let mut output_file = String::new();
    let mut options = CompilationOptions::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 1; // Skip the next argument since we consumed it
                } else {
                    return Err("--output option requires a file path".into());
                }
            }
            "--format" | "-f" => {
                options = options.with_formatting();
            }
            "--comments" | "-c" => {
                options = options.with_comments();
            }
            "--strict" | "-s" => {
                options = options.with_strict_mode();
            }
            "--minify" | "-m" => {
                options = options.with_minification();
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--version" | "-v" => {
                println!("TFI Language Compiler v1.0.0");
                std::process::exit(0);
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg).into());
            }
            _ => {
                if input_file == "main.tfi" {
                    input_file = args[i].clone();
                } else {
                    return Err("Multiple input files specified".into());
                }
            }
        }
        i += 1;
    }
    
    // Generate default output file if not specified
    if output_file.is_empty() {
        output_file = generate_default_output_file(&input_file);
    }
    
    Ok((input_file, output_file, options))
}

/// Generate a default output file name based on the input file
fn generate_default_output_file(input_file: &str) -> String {
    let path = Path::new(input_file);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    format!("{}.js", stem)
}

/// Print usage information
fn print_usage() {
    println!("TFI Language Compiler");
    println!();
    println!("Usage: tfi-lang [OPTIONS] [FILE]");
    println!();
    println!("Arguments:");
    println!("  FILE                    Input TFI file (default: main.tfi)");
    println!();
    println!("Options:");
    println!("  -o, --output FILE       Output JavaScript file (default: <input>.js)");
    println!("  -f, --format            Format the output JavaScript code");
    println!("  -c, --comments          Add source comments to output");
    println!("  -s, --strict            Enable strict mode");
    println!("  -m, --minify            Minify the output");
    println!("  -h, --help              Show this help message");
    println!("  -v, --version           Show version information");
    println!();
    println!("Examples:");
    println!("  tfi-lang main.tfi                           # Output: main.js");
    println!("  tfi-lang --output app.js main.tfi           # Output: app.js");
    println!("  tfi-lang -o dist/script.js program.tfi      # Output: dist/script.js");
    println!("  tfi-lang --format --comments script.tfi     # Output: script.js");
    println!("  tfi-lang -f -c -s -o minified.js app.tfi    # Output: minified.js");
}
