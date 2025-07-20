# TFI Language Compiler - Project Brief

## Overview
TFI (Telugu Film Industry) is a custom programming language inspired by Telugu movie names, designed as a learning project to understand compiler construction. The language uses movie names as keywords and compiles to JavaScript.

## Core Requirements

### Language Features
- **Keywords based on Telugu movies:**
  - `bahubali` - print statement
  - `rrr` - const declaration
  - `pushpa` - let declaration
  - `magadheera` - if statement
  - `karthikeya` - else statement
  - `pokiri` - while loop
  - `eega` - for loop

- **Operators:** `+`, `-`, `*`, `/`, `>`, `<`, `>=`, `<=`, `==`, `!=`

- **Data Types:** Numbers, Strings, Identifiers

- **Control Structures:** If-else, while loops, for loops

### Compiler Architecture
- **Lexer:** Tokenizes source code using `logos` crate
- **Parser:** Builds AST using `pest` parser generator
- **Validator:** Performs semantic analysis and variable scoping
- **Generator:** Converts AST to JavaScript code
- **Compiler:** Orchestrates the entire compilation process

### Testing Requirements
- Comprehensive unit tests for each module
- Integration tests for end-to-end compilation
- Error handling and edge cases
- Variable scoping and shadowing rules

## Current Status
✅ **Completed:**
- Modular architecture with separate lexer, parser, validator, generator, and compiler
- Support for all basic language constructs
- Variable scoping with const-to-let shadowing support
- Comprehensive test suite (56 unit tests, 17 integration tests passing)
- Error handling and validation
- Code generation to JavaScript

⚠️ **Remaining Issue:**
- One integration test failing due to complex parsing issue with multiline programs
- The `test_end_to_end_compilation` test has a grammar parsing error that needs investigation

## Technical Decisions
- Used Rust for implementation due to strong type safety and performance
- `logos` for lexing (fast and efficient)
- `pest` for parsing (declarative grammar)
- Modular design for maintainability and testability
- Comprehensive error handling with custom error types 