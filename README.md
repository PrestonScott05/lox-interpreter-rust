# Lox Interpreter in the Rust Programming Language - Preston Lumpkins

- this was built for an assignment in CS 403 at the University of Alabama
- I chose rust because I wanted to learn it. 

My interpreter follows the interpreter written in java at www.craftinginterpreters.com. There have been tweaks to closely match its syntax, but many features do not carry over. At the end of the day, my code is going to look very different from the original Java, but that's ok!

### Where I'm at:
Chapter 7 - evaluating expressions
- our interpreter evaluates expressions through our repl according to the requirements in craftinginterpreters

### File Structure

 lox-interpreter-rust/
    ├── Cargo.toml          # package manifest (no external dependencies)
    ├── README.md
    └── src/
        ├── main.rs         # entry point: runs a script file or the REPL; 
        ├── token_type.rs   # TokenType enum: every kind of token Lox recognizes
        ├── token.rs        # Token struct and Literal enum (the values the scanner pulls out of source text)
        ├── scanner.rs      # Scanner: turns source text into a Vec<Token> (Ch. 4)
        ├── expression.rs   # Expression enum (the AST) and an AST printer used in tests (Ch. 5)
        ├── parser.rs       # recursive descent Parser: Vec<Token> -> Expression, with error reporting (Ch. 6)
        ├── value.rs        # Value enum: runtime Lox values, printed the way jlox prints them (Ch. 7)
        └── interpreter.rs  # Interpreter: evaluates an Expression to a Value; RuntimeError (Ch. 7)

**Pipeline:** source text → `Scanner` → tokens → `Parser` → `Expression` (AST) → `Interpreter` → `Value`

### Build
cargo build

### Run
Run a Lox script file:

    cargo run -- <script.lox> 

Or start the interactive REPL (no arguments):
    cargo run

### Test
cargo test

**SOME TESTS WERE WRITTEN WITH THE ASSISTANCE OF CLAUDE SONNET 5**

#### Test Output: 
cargo test
   Compiling lox v0.1.0 (D:\academics\CS-403\projects\lox-interpreter-rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.93s                                  
     Running unittests src\main.rs (target\debug\deps\lox-aee9884658ea698a.exe)

running 20 tests
test expression::tests::prints_nested_tree ... ok
test interpreter::tests::string_concat ... ok
test interpreter::tests::arithmetic_and_precedence ... ok
test parser::tests::full_precedence_chain ... ok
test parser::tests::missing_right_paren_is_an_error ... ok
test interpreter::tests::comparison ... ok
test parser::tests::binary_operators_are_left_associative ... ok
test interpreter::tests::display_strips_trailing_zero ... ok
test parser::tests::missing_operand_is_an_error ... ok
test interpreter::tests::equality ... ok
test interpreter::tests::nan_equals_itself_like_jlox ... ok
test parser::tests::parses_literals ... ok
test parser::tests::parses_unary_and_grouping ... ok
test interpreter::tests::runtime_errors ... ok
test scanner::tests::reports_unterminated_string ... ok
test interpreter::tests::truthiness ... ok
test scanner::tests::scans_single_char_operators ... ok
test value::tests::matches_jlox_number_output ... ok
test scanner::tests::scans_number_with_value ... ok
test parser::tests::factor_binds_tighter_than_term ... ok
s
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


### Progress:
- Chapter 4: Scanning - Completed
- Chapter 5: Representing Code - Completed
- Chapter 6: Parsing Expressions - Completed
- Chapter 7: Evaluating Expressions - Completed
- Chapter 8: next!

