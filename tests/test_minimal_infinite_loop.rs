mod common;
use common::assert_output_file;

#[test]
fn minimal_infinite_loop_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_infinite_loop.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/minimal_infinite_loop/net.dot",
    );
}

#[test]
fn minimal_infinite_loop_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_infinite_loop.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/minimal_infinite_loop/net.lola",
    );
}

#[test]
fn minimal_infinite_loop_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_infinite_loop.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/minimal_infinite_loop/net.pnml",
    );
}
