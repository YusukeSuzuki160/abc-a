use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"254
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "54\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"101

"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "01\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
5 1 1
100 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

