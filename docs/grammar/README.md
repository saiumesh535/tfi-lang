# Grammar Definition - TFI Language

## What is the Grammar?

The grammar file defines the syntax rules for the TFI language using Pest's declarative grammar format. It specifies how tokens should be arranged to form valid programs. Think of it as the "rulebook" - it defines what constitutes valid TFI syntax.

## Grammar File: `grammar.pest`

The TFI language grammar is defined in the `grammar.pest` file using Pest's declarative syntax:

```pest
program = { SOI ~ statement* ~ EOI }

statement = { 
    print_statement |
    const_statement |
    let_statement |
    if_statement |
    while_statement |
    for_statement
}

print_statement = { "bahubali" ~ "(" ~ expression ~ ("," ~ expression)* ~ ")" ~ ";" }
const_statement = { "rrr" ~ ident ~ "=" ~ expression ~ ";" }
let_statement = { "pushpa" ~ ident ~ "=" ~ expression ~ ";" }

if_statement = { 
    "magadheera" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" ~ 
    else_block?
}

else_block = { "karthikeya" ~ "{" ~ statement* ~ "}" }

while_statement = { "pokiri" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" }

for_statement = { 
    "eega" ~ "(" ~ statement ~ expression ~ ";" ~ expression ~ ")" ~ 
    "{" ~ statement* ~ "}" 
}

operator = { "+" | "-" | "*" | "/" | ">" | "<" | ">=" | "<=" | "==" | "!=" }
expression = { term ~ (operator ~ term)* }

term = { 
    number | 
    ident | 
    string |
    "(" ~ expression ~ ")"
}

string = @{ "\"" ~ (ASCII_ALPHANUMERIC | " " | "!" | "#" | "$" | "%" | "&" | "'" | "(" | ")" | "*" | "+" | "," | "-" | "." | "/" | ":" | ";" | "<" | "=" | ">" | "?" | "@" | "[" | "]" | "^" | "_" | "`" | "{" | "|" | "}" | "~")* ~ "\"" }

number = @{ ASCII_DIGIT+ }
ident = @{ ASCII_ALPHA+ }

WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* ~ ("\n" | EOI) }
```

## Grammar Rules Explained

### 1. Program Structure

```pest
program = { SOI ~ statement* ~ EOI }
```

- **SOI**: Start of Input
- **statement***: Zero or more statements
- **EOI**: End of Input

**Example:**
```tfi
rrr x = 10;
bahubali("Hello");
```

### 2. Statement Types

```pest
statement = { 
    print_statement |
    const_statement |
    let_statement |
    if_statement |
    while_statement |
    for_statement
}
```

This defines the six types of statements in TFI:
- Print statements (`bahubali`)
- Const declarations (`rrr`)
- Let declarations (`pushpa`)
- If statements (`magadheera`)
- While loops (`pokiri`)
- For loops (`eega`)

### 3. Print Statement

```pest
print_statement = { "bahubali" ~ "(" ~ expression ~ ("," ~ expression)* ~ ")" ~ ";" }
```

**Components:**
- `"bahubali"`: Keyword
- `"("`: Opening parenthesis
- `expression`: First expression (required)
- `("," ~ expression)*`: Zero or more additional expressions
- `")"`: Closing parenthesis
- `";"`: Semicolon

**Examples:**
```tfi
bahubali("Hello");                    // One expression
bahubali("Hello", "World");           // Two expressions
bahubali("Count:", 42, x);            // Three expressions
```

### 4. Variable Declarations

#### Const Declaration
```pest
const_statement = { "rrr" ~ ident ~ "=" ~ expression ~ ";" }
```

**Components:**
- `"rrr"`: Const keyword
- `ident`: Variable identifier
- `"="`: Assignment operator
- `expression`: Value expression
- `";"`: Semicolon

**Examples:**
```tfi
rrr x = 42;
rrr name = "John";
rrr result = x + y;
```

#### Let Declaration
```pest
let_statement = { "pushpa" ~ ident ~ "=" ~ expression ~ ";" }
```

**Components:**
- `"pushpa"`: Let keyword
- `ident`: Variable identifier
- `"="`: Assignment operator
- `expression`: Value expression
- `";"`: Semicolon

**Examples:**
```tfi
pushpa counter = 0;
pushpa temp = x * 2;
pushpa message = "Hello";
```

### 5. Control Structures

#### If Statement
```pest
if_statement = { 
    "magadheera" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" ~ 
    else_block?
}

else_block = { "karthikeya" ~ "{" ~ statement* ~ "}" }
```

**Components:**
- `"magadheera"`: If keyword
- `"(" ~ expression ~ ")"`: Condition in parentheses
- `"{" ~ statement* ~ "}"`: Then block
- `else_block?`: Optional else block

**Examples:**
```tfi
magadheera(x > 5) {
    bahubali("x is greater than 5");
}

magadheera(x > 5) {
    bahubali("x is greater than 5");
} karthikeya {
    bahubali("x is 5 or less");
}
```

#### While Loop
```pest
while_statement = { "pokiri" ~ "(" ~ expression ~ ")" ~ "{" ~ statement* ~ "}" }
```

**Components:**
- `"pokiri"`: While keyword
- `"(" ~ expression ~ ")"`: Condition in parentheses
- `"{" ~ statement* ~ "}"`: Loop body

**Examples:**
```tfi
pokiri(i < 10) {
    bahubali(i);
    pushpa i = i + 1;
}
```

#### For Loop
```pest
for_statement = { 
    "eega" ~ "(" ~ statement ~ expression ~ ";" ~ expression ~ ")" ~ 
    "{" ~ statement* ~ "}" 
}
```

**Components:**
- `"eega"`: For keyword
- `"("`: Opening parenthesis
- `statement`: Initialization statement
- `expression`: Condition expression
- `";"`: Semicolon separator
- `expression`: Update expression
- `")"`: Closing parenthesis
- `"{" ~ statement* ~ "}"`: Loop body

**Examples:**
```tfi
eega(rrr i = 0; i < 5; i + 1) {
    bahubali(i);
}
```

### 6. Expressions

#### Expression Structure
```pest
expression = { term ~ (operator ~ term)* }
```

This defines expressions as a series of terms connected by operators, with left-to-right associativity.

#### Term Definition
```pest
term = { 
    number | 
    ident | 
    string |
    "(" ~ expression ~ ")"
}
```

Terms can be:
- Numbers: `42`, `100`, `0`
- Identifiers: `x`, `y`, `counter`
- Strings: `"Hello"`, `"World"`
- Parenthesized expressions: `(x + y)`

#### Operators
```pest
operator = { "+" | "-" | "*" | "/" | ">" | "<" | ">=" | "<=" | "==" | "!=" }
```

Supported operators:
- Arithmetic: `+`, `-`, `*`, `/`
- Comparison: `>`, `<`, `>=`, `<=`, `==`, `!=`

**Expression Examples:**
```tfi
42                    // Number term
x                     // Identifier term
"Hello"               // String term
(x + y)               // Parenthesized expression
x + y * z             // Binary expression
(a > b) && (c < d)    // Complex expression
```

### 7. Lexical Elements

#### Numbers
```pest
number = @{ ASCII_DIGIT+ }
```

- `@{ ... }`: Atomic rule (no whitespace handling)
- `ASCII_DIGIT+`: One or more digits

**Examples:** `0`, `42`, `100`, `999`

#### Identifiers
```pest
ident = @{ ASCII_ALPHA+ }
```

- `ASCII_ALPHA+`: One or more alphabetic characters

**Examples:** `x`, `y`, `counter`, `myVariable`

#### Strings
```pest
string = @{ "\"" ~ (ASCII_ALPHANUMERIC | " " | "!" | "#" | "$" | "%" | "&" | "'" | "(" | ")" | "*" | "+" | "," | "-" | "." | "/" | ":" | ";" | "<" | "=" | ">" | "?" | "@" | "[" | "]" | "^" | "_" | "`" | "{" | "|" | "}" | "~")* ~ "\"" }
```

**Components:**
- `"\"`: Opening quote
- `(...)*`: Zero or more allowed characters
- `"\"`: Closing quote

**Examples:** `"Hello"`, `"Hello, World!"`, `"x = 42"`

### 8. Whitespace and Comments

#### Whitespace
```pest
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
```

- `_`: Silent rule (ignored in AST)
- Handles spaces, tabs, carriage returns, and newlines

#### Comments
```pest
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* ~ ("\n" | EOI) }
```

- `"//"`: Comment start
- `(!"\n" ~ ANY)*`: Any characters except newline
- `("\n" | EOI)`: Newline or end of input

**Examples:**
```tfi
// This is a comment
rrr x = 42; // Inline comment
```

## Grammar Concepts

### 1. Repetition

| Symbol | Meaning | Example |
|--------|---------|---------|
| `*` | Zero or more | `statement*` |
| `+` | One or more | `ASCII_DIGIT+` |
| `?` | Zero or one | `else_block?` |

### 2. Alternatives

| Symbol | Meaning | Example |
|--------|---------|---------|
| `\|` | Or | `"+" \| "-" \| "*"` |

### 3. Grouping

| Symbol | Meaning | Example |
|--------|---------|---------|
| `{ ... }` | Group | `{ term ~ operator ~ term }` |

### 4. Atomic Rules

| Symbol | Meaning | Example |
|--------|---------|---------|
| `@{ ... }` | Atomic (no whitespace) | `@{ ASCII_DIGIT+ }` |

### 5. Silent Rules

| Symbol | Meaning | Example |
|--------|---------|---------|
| `_{ ... }` | Silent (ignored in AST) | `_{ " " \| "\t" }` |

## Grammar Validation

### Valid Programs

```tfi
// Simple program
bahubali("Hello, world!");

// Variable declarations
rrr x = 10;
pushpa y = 5;

// Control structures
magadheera(x > y) {
    bahubali("x is greater");
} karthikeya {
    bahubali("y is greater or equal");
}

// Loops
pokiri(i < 10) {
    bahubali(i);
    pushpa i = i + 1;
}

eega(rrr i = 0; i < 5; i + 1) {
    bahubali(i);
}
```

### Invalid Programs

```tfi
// Missing semicolon
rrr x = 42

// Missing parentheses
bahubali "Hello";

// Missing braces
magadheera(x > 5)
    bahubali("x is greater");

// Invalid operator
x @ y;

// Missing quotes
bahubali(Hello);
```

## Grammar Extensions

### Adding New Keywords

To add a new keyword (e.g., `function`):

1. **Add to statement alternatives:**
```pest
statement = { 
    print_statement |
    const_statement |
    let_statement |
    if_statement |
    while_statement |
    for_statement |
    function_statement
}
```

2. **Define the new statement:**
```pest
function_statement = { "function" ~ ident ~ "(" ~ parameter_list ~ ")" ~ "{" ~ statement* ~ "}" }
```

### Adding New Operators

To add a new operator (e.g., `%` for modulo):

```pest
operator = { "+" | "-" | "*" | "/" | "%" | ">" | "<" | ">=" | "<=" | "==" | "!=" }
```

### Adding New Data Types

To add floating-point numbers:

```pest
number = @{ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
```

## Testing the Grammar

### Grammar Validation Tests

```rust
#[test]
fn test_valid_program() {
    let source = r#"
        rrr x = 10;
        bahubali("Hello", x);
    "#;
    let result = MyLanguageParser::parse(Rule::program, source);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_syntax() {
    let source = "rrr x = 10"; // Missing semicolon
    let result = MyLanguageParser::parse(Rule::program, source);
    assert!(result.is_err());
}
```

### Grammar Coverage Tests

```rust
#[test]
fn test_all_statement_types() {
    let statements = vec![
        "bahubali(\"Hello\");",
        "rrr x = 42;",
        "pushpa y = 10;",
        "magadheera(x > 5) { bahubali(\"yes\"); }",
        "pokiri(i < 10) { bahubali(i); }",
        "eega(rrr i = 0; i < 5; i + 1) { bahubali(i); }"
    ];
    
    for stmt in statements {
        let result = MyLanguageParser::parse(Rule::statement, stmt);
        assert!(result.is_ok(), "Failed to parse: {}", stmt);
    }
}
```

## Performance Considerations

### Grammar Optimization

1. **Use atomic rules** for lexical elements:
```pest
number = @{ ASCII_DIGIT+ }  // Atomic
```

2. **Minimize backtracking** by ordering alternatives:
```pest
// Good: Specific before general
term = { number | ident | string | "(" ~ expression ~ ")" }

// Bad: General before specific
term = { "(" ~ expression ~ ")" | number | ident | string }
```

3. **Use silent rules** for whitespace:
```pest
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
```

## Summary

The grammar file defines the syntax rules for the TFI language using Pest's declarative format. It specifies how tokens should be arranged to form valid programs and provides the foundation for the parser.

Key takeaways:
- Grammar rules define valid syntax
- Pest provides declarative grammar definition
- Repetition and alternatives are powerful concepts
- Atomic and silent rules optimize performance
- Grammar extensions are straightforward
- Testing ensures grammar correctness 