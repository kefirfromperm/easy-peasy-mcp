use std::process::Command;

#[test]
fn check_lists_the_files() {
    let output = Command::new(env!("CARGO_BIN_EXE_app")).arg("--check").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.ends_with(" files\n"));
}
