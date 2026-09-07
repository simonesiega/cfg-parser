use std::process::{Command, Output};

fn run(args: &[&str], environment_input: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cfg-parser"));
    command.args(args).env("RUST_LOG", "off");

    match environment_input {
        Some(input) => {
            command.env("CFGPARSER_INPUT", input);
        }
        None => {
            command.env_remove("CFGPARSER_INPUT");
        }
    }

    command.output().expect("cfg-parser binary should run")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be UTF-8")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be UTF-8")
}

#[test]
fn evaluates_an_expression_from_one_argument() {
    let output = run(&["2 + 3 * 4 ="], None);

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), "Result: 14.000\n");
}

#[test]
fn joins_multiple_arguments_into_one_expression() {
    let output = run(&["(1", "+", "2)(3", "+", "4)", "="], None);

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), "Result: 21.000\n");
}

#[test]
fn reads_an_expression_from_the_environment() {
    let output = run(&[], Some("27 $ 3 ="));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), "Result: 3.000\n");
}

#[test]
fn command_line_arguments_take_precedence_over_the_environment() {
    let output = run(&["6 * 7 ="], Some("1 + 1 ="));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), "Result: 42.000\n");
}

#[test]
fn an_empty_environment_value_uses_the_default_expression() {
    let output = run(&[], Some("   "));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), "Result: -693.333\n");
}

#[test]
fn invalid_input_returns_a_failure_status_without_a_result() {
    let output = run(&["1 / 0 ="], None);

    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(
        stderr(&output).contains("DivisionByZero"),
        "unexpected stderr: {}",
        stderr(&output)
    );
}
