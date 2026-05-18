use std::io::Write;
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
    let mut child = Command::new(CLI).arg("-o").arg("short").arg("-").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    child.stdin.as_mut().unwrap().write_all(b"0\n").unwrap();
    let command = child.wait_with_output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("ID Type: Snowflake"));
}

#[test]
fn test_compare() {
    let command = Command::new(CLI).arg("-c").arg("1000000000000000000").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    let expected = "\
Date/times of the valid IDs parsed as:
- 1977-05-18T19:10:43.000Z Snowflake: Frostflake
- 1977-07-22T11:22:59.101Z Snowflake: LinkedIn
- 1977-07-22T11:22:59.101Z Snowflake: Flake ID
- 2001-09-09T01:46:40.000Z Unix timestamp: Assuming nanoseconds
- 2003-10-11T22:41:29.550Z Snowflake: Simpleflake
- 2015-06-04T14:48:31.271Z Thread ID (Meta Threads)
- 2015-06-04T14:48:31.271Z Snowflake: Instagram
- 2018-05-25T13:05:53.758Z Snowflake: Twitter
- 2022-07-22T11:22:59.101Z Snowflake: Discord
- 2022-07-22T11:22:59.101Z Snowflake: Spaceflake
- 2027-07-22T11:22:59.101Z TSID
- 2031-07-22T11:22:59.101Z SnowID
- 2033-07-21T16:27:27.750Z Snowflake: Sony
- 2453-07-13T08:31:02.500Z Snowflake: Mastodon";
    let actual: String = output.lines().filter(|line| !line.contains("--- Now ---")).collect::<Vec<_>>().join("\n");
    assert_eq!(actual, expected);
}

#[test]
fn test_unknown_id_type() {
    let command = Command::new(CLI).arg("WHAT%").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert_eq!(output, "Unknown ID type.\n");
}
