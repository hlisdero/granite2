mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/repeated_function_call.rs",
    "./examples/results/repeated_function_call/"
);
