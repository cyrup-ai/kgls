use kgls::ExitCode;

#[test]
fn test_exit_code_ordering() {
    assert!(ExitCode::OK < ExitCode::MinorIssue);
    assert!(ExitCode::MinorIssue < ExitCode::MajorIssue);
}

#[test]
fn test_exit_code_set_if_greater() {
    let mut code = ExitCode::OK;
    code.set_if_greater(ExitCode::MinorIssue);
    assert_eq!(code, ExitCode::MinorIssue);

    code.set_if_greater(ExitCode::OK);
    assert_eq!(code, ExitCode::MinorIssue);

    code.set_if_greater(ExitCode::MajorIssue);
    assert_eq!(code, ExitCode::MajorIssue);
}

#[test]
fn test_exit_code_to_i32() {
    assert_eq!(i32::from(ExitCode::OK), 0);
    assert_eq!(i32::from(ExitCode::MinorIssue), 1);
    assert_eq!(i32::from(ExitCode::MajorIssue), 2);
}
