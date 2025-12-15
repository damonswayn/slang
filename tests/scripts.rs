use std::path::PathBuf;
use std::process::Command;
use std::fs;
use slang::lexer::Lexer;
use slang::parser::Parser;
use slang::runtime::{TestRunSummary, run_tests};

/// Run a script from `test_scripts/` through the compiled `slang` binary
/// and return its trimmed stdout.
fn run_script(script_name: &str) -> String {
    // Path to the compiled binary for this crate, provided by Cargo for tests.
    let bin_path = env!("CARGO_BIN_EXE_slang");

    // Build an absolute path to the script inside `test_scripts/`.
    let script_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "test_scripts",
        script_name,
    ]
    .iter()
    .collect();

    let output = Command::new(bin_path)
        .arg(&script_path)
        .output()
        .expect("failed to invoke slang binary");

    if !output.status.success() {
        panic!(
            "slang process exited with status {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
}

fn run_tests_script(script_name: &str) -> TestRunSummary {
    let script_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "test_scripts",
        script_name,
    ]
    .iter()
    .collect();

    let src = fs::read_to_string(script_path)
    .expect("failed to read file");

    let lexer = Lexer::new(&src);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let output = run_tests(&program);
    return output;
}

#[test]
fn fact_script_produces_expected_result() {
    let output = run_script("fact.sl");
    // 20! = 2432902008176640000
    assert_eq!(output, "2432902008176640000");
}

#[test]
fn test_a_script_produces_expected_result() {
    let output = run_script("test_a.sl");
    // len([2, 3]) == 2, len([1, 2, 3, 4]) == 4, 2 + 4 == 6
    assert_eq!(output, "6");
}

#[test]
fn file_builtins_script_produces_expected_result() {
    let output = run_script("test_file_builtins.sl");
    assert_eq!(output, "\"Hello, world!\"\nnull");
}

#[test]
fn test_objects_script_produces_expected_result() {
    let output = run_script("test_objects.sl");
    assert_eq!(output, "10\n15\n25\n30\n1\n2\n3\n4\n5\n6\n6\nnull");
}

#[test]
fn test_fizzbuzz_script_produces_expected_result() {
    let output = run_script("fizzbuzz.sl");
    assert_eq!(output, "1\n2\n\"Fizz\"\n4\n\"Buzz\"\n\"Fizz\"\n7\n8\n\"Fizz\"\n\"Buzz\"\n11\n\"Fizz\"\n13\n14\n\"FizzBuzz\"\n16\n17\n\"Fizz\"\n19\n\"Buzz\"\n\"Fizz\"\n22\n23\n\"Fizz\"\n\"Buzz\"\n26\n\"Fizz\"\n28\n29\n\"FizzBuzz\"\n31\n32\n\"Fizz\"\n34\n\"Buzz\"\n\"Fizz\"\n37\n38\n\"Fizz\"\n\"Buzz\"\n41\n\"Fizz\"\n43\n44\n\"FizzBuzz\"\n46\n47\n\"Fizz\"\n49\n\"Buzz\"\n\"Fizz\"\n52\n53\n\"Fizz\"\n\"Buzz\"\n56\n\"Fizz\"\n58\n59\n\"FizzBuzz\"\n61\n62\n\"Fizz\"\n64\n\"Buzz\"\n\"Fizz\"\n67\n68\n\"Fizz\"\n\"Buzz\"\n71\n\"Fizz\"\n73\n74\n\"FizzBuzz\"\n76\n77\n\"Fizz\"\n79\n\"Buzz\"\n\"Fizz\"\n82\n83\n\"Fizz\"\n\"Buzz\"\n86\n\"Fizz\"\n88\n89\n\"FizzBuzz\"\n91\n92\n\"Fizz\"\n94\n\"Buzz\"\n\"Fizz\"\n97\n98\n\"Fizz\"\n\"Buzz\"\nnull");
}

#[test]
fn test_higher_order_functions_script_produces_expected_result() {
    let output = run_script("higher_order_funcs.sl");
    assert_eq!(output, "5\n17\nnull");
}

#[test]
fn test_monads_script_produces_expected_result() {
    let output = run_script("monads.sl");
    assert_eq!(output, "5\n\"failure\"\n\"Found value in list at index\"\n2\n\"Value not in list\"\nnull");
}

#[test]
fn test_the_test_suite_script_produces_expected_result() {
    let output: TestRunSummary = run_tests_script("testing_tests.sl");
    assert_eq!(output.total, 4);
    assert_eq!(output.failed, 1);
    assert_eq!(output.output, "PASS: adds two numbers\nPASS: simple boolean assertion\nPASS: testing not equals\nFAIL: this test should fail - Assertion failed: \"This is expected to fail.\"\n\nTest results: 3/4 passed, 1 failed\n");
}

#[test]
fn test_import_namespace_script_produces_expected_result() {
    let output = run_script("test_import_namespace.sl");
    assert_eq!(output, "12");
}

#[test]
fn type_casting_script_produces_expected_result() {
    let output = run_script("type_casting.sl");
    assert_eq!(
        output,
        "42\n3.14\n\"123\"\nfalse\ntrue\ntrue\nfalse\n2.5\n\"type error\"\n\"Some(5)\"\nnull"
    );
}

#[test]
fn pubsub_script_invokes_tagged_functions() {
    let output = run_script("pubsub.sl");
    assert_eq!(output, "12\n35\n144\n144\nnull");
}