mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/minimal_shared_condvar.rs",
    "./examples/results/minimal_shared_condvar/"
);
