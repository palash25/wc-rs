use std::process::Command;

#[test]
fn simple_wc() {
    let result = Command::new("cargo")
    .arg("build")
    .arg("--release")
    .output()
    .expect("Could not run");
    assert_eq!(result.status.code(), Some(0));


    let result = Command::new("target/release/wc-rs")
    .arg("tests/a.txt")
    .output()
    .expect("Could not run");
    assert_eq!(result.status.code(), Some(0));
    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "2\t4\t25\t\u{1b}[2;32mtests/a.txt\u{1b}[0m\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");


    let result = Command::new("target/release/wc-rs")
    .arg("tests/empty.txt")
    .output()
    .expect("Could not run");
    assert_eq!(result.status.code(), Some(0));
    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "0\t0\t0\t\u{1b}[2;32mtests/empty.txt\u{1b}[0m\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");

}

