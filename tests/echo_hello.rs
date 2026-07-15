use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn echo_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_mash"))
        .args(["-c", "echo 'hello world'"])
        .output()
        .expect("failed to execute bash");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "hello world\n");
}

#[test]
fn repl_echo_works() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mash"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        writeln!(stdin, "echo hello").unwrap();
        writeln!(stdin, "exit").unwrap();
    }

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("hello"));
}
