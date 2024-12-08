use std::process::{Command, Stdio};

const CLI: &str = "target/debug/uuinfo";

#[test]
fn test_version() {
    let command = Command::new(CLI).arg("--version").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("uuinfo "));
}

#[test]
fn test_output_card_default() {
    let command = Command::new(CLI).arg("0").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("┏━"));
    assert!(output.ends_with("━┛\n"));
}

#[test]
fn test_output_card() {
    let command = Command::new(CLI).arg("-o").arg("card").arg("0").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("┏━"));
    assert!(output.ends_with("━┛\n"));
}

#[test]
fn test_output_json() {
    let command = Command::new(CLI).arg("-o").arg("json").arg("0").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("{"));
    assert!(output.ends_with("}\n"));
}

#[test]
fn test_output_short() {
    let command = Command::new(CLI).arg("-o").arg("short").arg("0").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("ID Type: Snowflake"));
}

#[test]
fn test_output_binary() {
    let command = Command::new(CLI).arg("-o").arg("binary").arg("0").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert_eq!(output, "\0\0\0\0\0\0\0\0");
}

#[test]
fn test_stdin() {
    let echo = Command::new("echo").arg("0").stdout(Stdio::piped()).spawn().unwrap();
    let command = Command::new(CLI).arg("-o").arg("short").arg("-").stdin(Stdio::from(echo.stdout.unwrap())).output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("ID Type: Snowflake"));
}

#[test]
fn test_unknown_id_type() {
    let command = Command::new(CLI).arg("WHAT%").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert_eq!(output, "Unknown ID type.\n");
}
