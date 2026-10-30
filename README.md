# Lox Interpreter in the Rust Programming Language - Preston Lumpkins

- this was built for an assignment in CS 403 at the University of Alabama
- I chose rust because I wanted to learn it. 


### Build
cargo build

### Run
Run a Lox script file:

    cargo run -- <script.lox> 

Or start the interactive REPL (no arguments):
    cargo run

### Test
cargo test

#### Test Output: 
 cargo test
   Compiling lox v0.1.0 (D:\academics\CS-403\projects\lox-interpreter-rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.16s                                                                                          
     Running unittests src\main.rs (target\debug\deps\lox-aee9884658ea698a.exe)

running 11 tests
test expression::tests::prints_nested_tree ... ok
test parser::tests::missing_right_paren_is_an_error ... ok
test scanner::tests::reports_unterminated_string ... ok
test parser::tests::full_precedence_chain ... ok
test parser::tests::binary_operators_are_left_associative ... ok
test parser::tests::missing_operand_is_an_error ... ok
test scanner::tests::scans_number_with_value ... ok
test parser::tests::factor_binds_tighter_than_term ... ok
test parser::tests::parses_unary_and_grouping ... ok
test scanner::tests::scans_single_char_operators ... ok
test parser::tests::parses_literals ... ok


### Progress:
- Chapter 4: Scanning - Completed
- Chapter 5: Representing Code - Completed
- Chapter 6: Parsing Expressions - Completed
- Next - Chapter 7

