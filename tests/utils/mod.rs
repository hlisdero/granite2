use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

/// Asserts that the contents of the given output file correspond to the expected file contents
/// after running `granite2` on the given source code file.
///
/// # Panics
///
/// If the command `granite2` is not found, then the function panics.
/// If the output file cannot be opened, then the function panics.
/// If the output file contents cannot be read, then the function panics.
pub fn assert_output_file(
    source_code_file: &str,
    output_folder: &str,
    format: &str,
    output_filename: &str,
    expected_contents_filename: &str,
) {
    let mut cmd = Command::cargo_bin("granite2").expect("Command not found");

    // Current workdir is always the project root folder
    cmd.arg(source_code_file)
        .arg(format!("--output-folder={output_folder}"))
        .arg(format!("--format={format}"))
        .arg("--filename=test");
    cmd.assert().success();

    let file_contents =
        std::fs::read_to_string(output_filename).expect("Could not read output file to string");

    let expected_file_contents = std::fs::read_to_string(expected_contents_filename)
        .expect("Could not read file with expected contents to string");

    assert_eq!(file_contents, expected_file_contents);
    std::fs::remove_file(output_filename).expect("Could not delete output file");
}

/// This macro generates the test code for the three supported file formats.
/// It saves a considerable ammount of boilerplate.
///
/// Receives the relative path from the root folder of the repository
/// to the source code of the program to be tested.
/// Receives also the relative path from the root folder of the repository
/// to the folder where the expected results are to be found.
///
/// The main idea for the implementation was taken from:
/// <https://doc.rust-lang.org/rust-by-example/macros/dry.html>
macro_rules! generate_tests_for_example_program {
    ($program_path:literal, $result_folder_path:literal) => {
        #[test]
        fn generates_correct_dot_output_file() {
            utils::assert_output_file(
                $program_path,
                $result_folder_path,
                "dot",
                concat!($result_folder_path, "/test.dot"),
                concat!($result_folder_path, "/net.dot"),
            );
        }

        #[test]
        fn generates_correct_lola_output_file() {
            utils::assert_output_file(
                $program_path,
                $result_folder_path,
                "lola",
                concat!($result_folder_path, "/test.lola"),
                concat!($result_folder_path, "/net.lola"),
            );
        }

        #[test]
        fn generates_correct_pnml_output_file() {
            utils::assert_output_file(
                $program_path,
                $result_folder_path,
                "pnml",
                concat!($result_folder_path, "/test.pnml"),
                concat!($result_folder_path, "/net.pnml"),
            );
        }
    };
}

/// Exports the previously defined macro.
/// For the idea for the re-export, see:
/// <https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files#31749071>
/// A warning is generated if some test file does NOT use this function.
/// That is because each test is compiled as an independent crate.
/// See more details here: <https://stackoverflow.com/a/67902444>
pub(crate) use generate_tests_for_example_program;
