# TFI Language

A Telugu Film Industry inspired programming language that compiles to JavaScript. TFI uses iconic Telugu movie names as keywords to create a unique and culturally relevant programming experience.

## Features

- **Movie-themed Keywords**: Uses Telugu movie names as programming keywords
- **JavaScript Output**: Compiles to clean, readable JavaScript code
- **Modular Architecture**: Well-structured, testable codebase
- **Comprehensive Testing**: Extensive unit and integration tests
- **Error Handling**: Detailed error messages and validation
- **Command Line Interface**: Easy-to-use CLI with various options

## Keywords

| TFI Keyword | JavaScript Equivalent | Description |
|-------------|----------------------|-------------|
| `bahubali` | `console.log()` | Print statements |
| `rrr` | `const` | Constant declarations |
| `pushpa` | `let` | Variable declarations |
| `magadheera` | `if` | Conditional statements |
| `karthikeya` | `else` | Else clauses |
| `pokiri` | `while` | While loops |
| `eega` | `for` | For loops |

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/tfi-lang.git
cd tfi-lang

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Compile a TFI file
tfi-lang main.tfi

# Compile with formatting and comments
tfi-lang --format --comments program.tfi

# Show help
tfi-lang --help
```

### Command Line Options

- `-f, --format`: Format the output JavaScript code
- `-c, --comments`: Add source comments to output
- `-s, --strict`: Enable strict mode
- `-m, --minify`: Minify the output
- `-h, --help`: Show help message
- `-v, --version`: Show version information

## Examples

### Hello World

```tfi
bahubali("Hello, TFI World!");
```

Compiles to:
```javascript
console.log("Hello, TFI World!");
```

### Variables and Arithmetic

```tfi
rrr x = 10;
pushpa y = 5;
bahubali("Sum:", x + y);
bahubali("Product:", x * y);
```

Compiles to:
```javascript
const x = 10;
let y = 5;
console.log("Sum:", (x + y));
console.log("Product:", (x * y));
```

### Conditional Statements

```tfi
rrr age = 18;
magadheera(age >= 18) {
    bahubali("You are an adult");
}
karthikeya {
    bahubali("You are a minor");
}
```

Compiles to:
```javascript
const age = 18;
if ((age >= 18)) {
    console.log("You are an adult");
} else {
    console.log("You are a minor");
}
```

### Loops

```tfi
// While loop
rrr i = 0;
pokiri(i < 5) {
    bahubali("Count:", i);
    pushpa i = i + 1;
}

// For loop
eega(rrr j = 0; j < 3; j + 1) {
    bahubali("For count:", j);
}
```

Compiles to:
```javascript
// While loop
const i = 0;
while ((i < 5)) {
    console.log("Count:", i);
    let i = (i + 1);
}

// For loop
for (const j = 0; (j < 3); (j + 1)) {
    console.log("For count:", j);
}
```

### Complex Program

```tfi
bahubali("TFI Calculator");

rrr a = 10;
rrr b = 5;

magadheera(a > b) {
    bahubali("a is greater than b");
    bahubali("Difference:", a - b);
}
karthikeya {
    bahubali("b is greater than or equal to a");
}

rrr sum = 0;
pushpa i = 1;
pokiri(i <= 5) {
    pushpa sum = sum + i;
    pushpa i = i + 1;
}

bahubali("Sum of 1 to 5:", sum);
```

## Project Structure

```
tfi-lang/
├── src/
│   ├── lib.rs          # Library entry point and public API
│   ├── main.rs         # Command line interface
│   ├── ast.rs          # Abstract Syntax Tree definitions
│   ├── lexer.rs        # Tokenization and lexical analysis
│   ├── parser.rs       # Syntax parsing with pest
│   ├── generator.rs    # JavaScript code generation
│   ├── validator.rs    # Semantic validation
│   └── compiler.rs     # Compilation orchestration
├── tests/
│   └── integration_tests.rs  # Integration tests
├── grammar.pest        # Pest grammar definition
├── main.tfi           # Example TFI program
├── Cargo.toml         # Project configuration
└── README.md          # This file
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_compilation_workflow
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check for errors without building
cargo check
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run benchmarks
cargo bench
```

## Architecture

The TFI language compiler follows a traditional compiler architecture:

1. **Lexical Analysis** (`lexer.rs`): Converts source code into tokens
2. **Syntax Analysis** (`parser.rs`): Builds an Abstract Syntax Tree (AST)
3. **Semantic Analysis** (`validator.rs`): Validates the program semantics
4. **Code Generation** (`generator.rs`): Converts AST to JavaScript
5. **Compilation** (`compiler.rs`): Orchestrates the entire process

### Key Components

- **AST**: Represents the program structure with `Statement` and `Expression` enums
- **Lexer**: Uses `logos` for efficient tokenization
- **Parser**: Uses `pest` for parsing expression grammar
- **Validator**: Performs semantic checks like variable scoping
- **Generator**: Produces clean, readable JavaScript code

## Error Handling

The compiler provides detailed error messages for:

- Syntax errors (invalid TFI syntax)
- Semantic errors (undefined variables, empty blocks)
- Validation errors (duplicate declarations)
- Compilation errors (general compilation issues)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust coding conventions
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Telugu cinema and its cultural impact
- Built with Rust's excellent ecosystem
- Uses pest for parsing and logos for lexing
- Thanks to the Rust community for excellent tooling

## Future Enhancements

- [ ] Support for functions and procedures
- [ ] Arrays and data structures
- [ ] More operators and expressions
- [ ] Standard library functions
- [ ] REPL (Read-Eval-Print Loop)
- [ ] IDE support and syntax highlighting
- [ ] Performance optimizations
- [ ] More movie-themed keywords

---

**TFI Language** - Where Telugu cinema meets programming! 🎬💻 