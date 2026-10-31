# Lox Interpreter in the Rust Programming Language - Preston Lumpkins

- this was built for an assignment in CS 403 at the University of Alabama
- I chose rust because I wanted to learn it. 

My interpreter follows the interpreter written in java at www.craftinginterpreters.com. There have been tweaks to closely match its syntax, but many features do not carry over. At the end of the day, my code is going to look very different from the original Java, but that's ok!

Note: Also, using cargo fmt, you can get some nice code conventions. While reviewing code, if lines are smushed, that's why!

### Where I'm at:
Chapter 8 (Statements and State) is complete: the interpreter runs scripts and a REPL with `print` and expression statements, global and block-scoped variables, and assignment, as specified in *Crafting Interpreters*.

### File Structure

 - lox-interpreter-rust/
    - Cargo.toml          # package manifest (no external dependencies)
    - README.md
    - src/
         - main.rs         # entry point: runs a script file or the REPL; 
         - token_type.rs   # TokenType enum: every kind of token Lox recognizes
         - token.rs        # Token struct and Literal enum (the values the scanner pulls out of source text)
         - scanner.rs      # Scanner: turns source text into a Vec<Token> 
         - expression.rs   # Expression enum and AST printer used in tests
         - parser.rs       # recursive descent Parser: Vec<Token> -> Expression
         - value.rs        # Value enum: runtime Lox values
         - interpreter.rs  # Interpreter: evaluates an Expression to a Value; RuntimeError (Ch. 7)
         - stmt.rs # Statement:..
         - environment.rs #...

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

running 45 tests
test expression::tests::prints_nested_tree ... ok
test interpreter::tests::arithmetic_and_precedence ... ok
test interpreter::tests::chained_assignment ... ok
test interpreter::tests::assignment_in_block_updates_outer ... ok
test interpreter::tests::environment_restored_after_error_in_block ... ok
test interpreter::tests::equality ... ok
test interpreter::tests::assigning_undeclared_variable_errors ... ok
test interpreter::tests::block_locals_disappear ... ok
test interpreter::tests::undefined_variable_is_A_runtime_error ... ok
test interpreter::tests::display_strips_trailing_zero ... ok
test interpreter::tests::globals_can_be_redefined ... ok
test interpreter::tests::assignment_updates_variable ... ok
test interpreter::tests::shadowing_does_not_leak ... ok
test interpreter::tests::runtime_errors ... ok
test interpreter::tests::initializer_can_read_other_variables ... ok
test interpreter::tests::string_concat ... ok
test interpreter::tests::truthiness ... ok
test interpreter::tests::undefined_variable_in_initializer_errors ... ok
test interpreter::tests::uninitialized_var_isa_nil ... ok
test interpreter::tests::variables_persist_betweenstatements ... ok
test parser::tests::binary_operators_are_left_associative ... ok
test parser::tests::factor_binds_tighter_than_term ... ok
test interpreter::tests::nan_equals_itself_like_jlox ... ok
test parser::tests::missing_operand_is_an_error ... ok
test parser::tests::invalid_assignment_targets ... ok
test parser::tests::missing_right_paren_is_an_error ... ok
test parser::tests::parses_block ... ok
test interpreter::tests::comparison ... ok
test parser::tests::assignment_is_right_associative ... ok
test parser::tests::full_precedence_chain ... ok
test interpreter::tests::assignment_is_an_expression ... ok
test parser::tests::parses_assignment ... ok
test parser::tests::parses_literals ... ok
test parser::tests::parses_print_statement ... ok
test parser::tests::parses_unary_and_grouping ... ok
test parser::tests::parses_var_with_initializer ... ok
test parser::tests::parses_var_without_initializer ... ok
test parser::tests::parses_variable_expression ... ok
test parser::tests::recovers_after_error_and_keepsparsing ... ok
test parser::tests::unclosed_block_is_an_error ... ok
test parser::tests::var_missing_name_is_error ... ok
test scanner::tests::reports_unterminated_string ... ok
test scanner::tests::scans_number_with_value ... ok
test scanner::tests::scans_single_char_operators ... ok
test value::tests::matches_jlox_number_output ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


### Progress:
- Chapter 4: Scanning - Completed
- Chapter 5: Representing Code - Completed
- Chapter 6: Parsing Expressions - Completed
- Chapter 7: Evaluating Expressions - Completed
- Chapter 8: statements
- Next: Chapter 9!

