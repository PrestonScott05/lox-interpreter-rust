# Lox Interpreter in the Rust Programming Language - Preston Lumpkins

- this was built for an assignment in CS 403 at the University of Alabama
- I chose rust because I wanted to learn it. 

My interpreter follows the interpreter written in java at www.craftinginterpreters.com. There have been tweaks to closely match its syntax, but many features do not carry over. At the end of the day, my code is going to look very different from the original Java, but that's ok!

Note: Also, using cargo fmt, you can get some nice code conventions. While reviewing code, if lines are smushed, that's why!

### Where I'm at:
All of Part II of the crafting interpreters book (Chapters 4-13) is complete. this is a full tree-walk interpreeter for Lox, as it's specified in crafting interpreteres. Chapter 13 adds superclasses, methods inherited through the chain, and the super.method() calls, which are resolved statically to the superclass of the class where they're written. 
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
         - resolver.rs #...

**Pipeline:** source text → `Scanner` → tokens → `Parser` → `Expression` (AST) → `Interpreter` → `Value`

### Build
Prerequisites: Rust **1.85 or newer** (the crate uses `edition = "2024"`; developed with Rust 1.87). Install Rust from https://rustup.rs. There are no other dependencies.

from the project root: 
    cargo build

### Run
Run a Lox script file:

    cargo run -- <script.lox> 

Or start the interactive REPL (no arguments):
    cargo run

### Test
cargo test

***SOME TESTS AND BLOCKS OF CODE THAT WERE UNABLE TO BE TRANSLATED WERE WRITTEN WITH THE ASSISTANCE OF CLAUDE SONNET 5***

#### Test Output: 
pslum@PrestonsLaptop /cygdrive/d/academics/CS-403/projects/lox-interpreter-rust
$ cargo test
   Compiling lox v0.1.0 (D:\academics\CS-403\projects\lox-interpreter-rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.03s                                      
     Running unittests src\main.rs (target\debug\deps\lox-aee9884658ea698a.exe)

running 152 tests
test expression::tests::prints_nested_tree ... ok
test interpreter::tests::calling_a_non_function_errors ... ok
test interpreter::tests::arithmetic_and_precedence ... ok
test interpreter::tests::assigning_undeclared_variable_errors ... ok
test interpreter::tests::assignment_in_block_updates_outer ... ok
test interpreter::tests::assignment_updates_variable ... ok
test interpreter::tests::block_locals_disappear ... ok
test interpreter::tests::assignment_in_a_closure_targets_the_right_scope ... ok
test interpreter::tests::display_strips_trailing_zero ... ok
test interpreter::tests::closures_bind_to_the_variable_in_scope_when_declared ... ok
test interpreter::tests::class_arity_comes_from_init ... ok
test interpreter::tests::equality ... ok
test interpreter::tests::early_return_in_init_still_returns_this ... ok
test interpreter::tests::environment_restored_after_return_from_block ... ok
test interpreter::tests::fields_can_be_set_and_read ... ok
test interpreter::tests::functions_print_their_name ... ok
test interpreter::tests::fields_shadow_methods ... ok
test interpreter::tests::clock_returns_a_number ... ok
test interpreter::tests::for_with_expression_initializer_uses_outer_var ... ok
test interpreter::tests::chained_assignment ... ok
test interpreter::tests::assignment_is_an_expression ... ok
test interpreter::tests::environment_restored_after_error_in_block ... ok
test interpreter::tests::calling_init_directly_returns_this ... ok
test interpreter::tests::fibonacci_from_the_book ... ok
test interpreter::tests::calls_a_function_with_arguments ... ok
test interpreter::tests::comparison ... ok
test interpreter::tests::bound_methods_remember_their_instance ... ok
test interpreter::tests::for_loop_scopes_its_variable ... ok
test interpreter::tests::closures_keep_their_environment ... ok
test interpreter::tests::dangling_else_runs_correctly ... ok
test interpreter::tests::each_closure_gets_its_own_variables ... ok
test interpreter::tests::error_in_while_condition_stops_the_loop ... ok
test interpreter::tests::functions_stored_in_fields_can_be_called ... ok
test interpreter::tests::global_functions_can_use_globals_defined_later ... ok
test interpreter::tests::globals_can_be_redefined ... ok
test interpreter::tests::if_else_branches ... ok
test interpreter::tests::init_is_inherited ... ok
test interpreter::tests::init_runs_with_arguments ... ok
test interpreter::tests::initializer_can_read_other_variables ... ok
test interpreter::tests::instances_and_classes_compare_by_identity ... ok
test interpreter::tests::local_classes_can_inherit ... ok
test interpreter::tests::locals_shadow_globals_inside_functions ... ok
test interpreter::tests::logical_returns_operands ... ok
test interpreter::tests::logical_short_circuits ... ok
test interpreter::tests::methods_are_found_through_a_long_chain ... ok
test interpreter::tests::methods_can_be_called ... ok
test interpreter::tests::methods_can_refer_to_their_own_class ... ok
test interpreter::tests::nan_equals_itself_like_jlox ... ok
test interpreter::tests::nested_closures_reach_outer_locals ... ok
test interpreter::tests::plain_function_named_init_is_not_special ... ok
test interpreter::tests::prints_classes_and_instances ... ok
test interpreter::tests::property_runtime_errors ... ok
test interpreter::tests::return_breaks_out_of_loops ... ok
test interpreter::tests::return_values_and_implicit_nil ... ok
test interpreter::tests::runtime_errors ... ok
test interpreter::tests::set_checks_the_object_before_evaluating_the_value ... ok
test interpreter::tests::set_is_an_expression ... ok
test interpreter::tests::shadowing_does_not_leak ... ok
test interpreter::tests::string_concat ... ok
test interpreter::tests::subclass_init_can_call_super_init ... ok
test interpreter::tests::subclass_methods_override ... ok
test interpreter::tests::subclasses_inherit_methods ... ok
test interpreter::tests::super_calls_the_superclass_method ... ok
test interpreter::tests::super_is_resolved_statically ... ok
test interpreter::tests::super_method_can_be_stored_and_called_later ... ok
test interpreter::tests::super_methods_are_bound_to_this ... ok
test interpreter::tests::superclass_must_be_a_class ... ok
test interpreter::tests::this_inside_a_closure_inside_a_method ... ok
test interpreter::tests::truthiness ... ok
test interpreter::tests::this_reads_fields_on_the_instance ... ok
test interpreter::tests::undefined_super_method_is_an_error ... ok
test interpreter::tests::undefined_variable_in_initializer_errors ... ok
test interpreter::tests::undefined_variable_is_a_runtime_error ... ok
test interpreter::tests::uninitialized_var_isa_nil ... ok
test interpreter::tests::variables_persist_betweenstatements ... ok
test interpreter::tests::while_loop ... ok
test interpreter::tests::wrong_argument_count_errors ... ok
test parser::tests::assignment_is_right_associative ... ok
test parser::tests::assignment_still_works_over_logical ... ok
test parser::tests::bare_return_has_no_value ... ok
test parser::tests::binary_operators_are_left_associative ... ok
test parser::tests::call_binds_tighter_than_unary ... ok
test parser::tests::call_result_is_not_an_assignment_target ... ok
test parser::tests::class_syntax_errors ... ok
test parser::tests::dangling_else_binds_to_nearest_if ... ok
test parser::tests::empty_for_is_just_a_while_true ... ok
test parser::tests::factor_binds_tighter_than_term ... ok
test parser::tests::for_desugars_to_block_with_while ... ok
test parser::tests::full_precedence_chain ... ok
test parser::tests::invalid_assignment_targets ... ok
test parser::tests::logical_precedence ... ok
test parser::tests::missing_operand_is_an_error ... ok
test parser::tests::missing_paren_after_if_is_an_error ... ok
test parser::tests::missing_right_paren_is_an_error ... ok
test parser::tests::parses_assignment ... ok
test parser::tests::parses_block ... ok
test parser::tests::parses_calls ... ok
test parser::tests::parses_class_with_methods ... ok
test parser::tests::parses_empty_class ... ok
test parser::tests::parses_function_declaration ... ok
test parser::tests::parses_get_chains_and_method_calls ... ok
test parser::tests::parses_if_else ... ok
test parser::tests::parses_literals ... ok
test parser::tests::parses_print_statement ... ok
test parser::tests::parses_super_calls ... ok
test parser::tests::parses_superclass ... ok
test parser::tests::parses_this ... ok
test parser::tests::parses_unary_and_grouping ... ok
test parser::tests::parses_var_with_initializer ... ok
test parser::tests::parses_var_without_initializer ... ok
test parser::tests::parses_variable_expression ... ok
test parser::tests::parses_while ... ok
test parser::tests::property_assignment_becomes_set ... ok
test parser::tests::recovers_after_error_and_keepsparsing ... ok
test parser::tests::super_syntax_errors ... ok
test parser::tests::unclosed_block_is_an_error ... ok
test parser::tests::var_missing_name_is_error ... ok
test resolver::tests::assignment_gets_resolved_too ... ok
test resolver::tests::bare_return_in_init_is_fine ... ok
test resolver::tests::class_cant_inherit_from_itself ... ok
test parser::tests::too_many_arguments_is_an_error ... ok
test resolver::tests::duplicate_local_is_an_error ... ok
test resolver::tests::duplicate_parameters_are_an_error ... ok
test resolver::tests::error_message_format_matches_jlox ... ok
test resolver::tests::global_initializer_can_mention_itself ... ok
test resolver::tests::globals_are_not_in_the_map ... ok
test resolver::tests::globals_can_be_redeclared ... ok
test resolver::tests::local_in_enclosing_scope_is_depth_one ... ok
test resolver::tests::local_in_same_scope_is_depth_zero ... ok
test resolver::tests::parameters_resolve_inside_the_body ... ok
test resolver::tests::plain_function_named_init_can_return_a_value ... ok
test resolver::tests::reading_local_in_its_own_initializer_is_an_error ... ok
test resolver::tests::reports_every_error_not_just_the_first ... ok
test resolver::tests::return_inside_a_function_is_fine ... ok
test resolver::tests::returning_a_value_from_a_normal_method_is_fine ... ok
test resolver::tests::returning_a_value_from_init_is_an_error ... ok
test resolver::tests::shadowing_in_an_inner_scope_is_fine ... ok
test resolver::tests::super_in_a_subclass_is_fine ... ok
test resolver::tests::super_outside_a_class_is_an_error ... ok
test resolver::tests::super_resolves_two_scopes_above_the_method_body ... ok
test resolver::tests::super_without_a_superclass_is_an_error ... ok
test resolver::tests::this_at_top_level_is_an_error ... ok
test resolver::tests::this_in_a_function_nested_in_a_method_is_fine ... ok
test resolver::tests::this_in_a_plain_function_is_an_error ... ok
test resolver::tests::this_inside_a_method_is_fine ... ok
test resolver::tests::this_resolves_one_scope_above_the_method_body ... ok
test resolver::tests::top_level_return_is_an_error ... ok
test scanner::tests::reports_unterminated_string ... ok
test scanner::tests::scans_number_with_value ... ok
test scanner::tests::scans_single_char_operators ... ok
test value::tests::matches_jlox_number_output ... ok
test interpreter::tests::recursion_fib ... ok

test result: ok. 152 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

### Limitations
- Infinite recursion crashes the program
- Memory for closures and instances that refer to themselves is never freed 
- The book's optional end-of-chapter challenges are not implemented.
- The interpreter has not been run against the book's official test suite; the tests here are based on the book's examples and error messages.

### Progress:
- Chapter 4: Scanning - Completed
- Chapter 5: Representing Code - Completed
- Chapter 6: Parsing Expressions - Completed
- Chapter 7: Evaluating Expressions - Completed
- Chapter 8: statements - Completed
- Chapter 9: Control Flow - Completed
- Chapter 10: Functions - Completed
- Chapter 11: Resolving and Binding - Completedd
- Chapter 12: Classes - Completed
- Chapter 13: Inheritance - Completed
- JLOX: COMPLETE!!


