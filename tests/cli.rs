use std::process::Command;

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

fn command() -> Command {
    Command::new(get_cargo_bin("example-rust-cli"))
}

#[test]
fn snapshots_cli_metadata() {
    assert_cmd_snapshot!("help", command().arg("--help"));
    assert_cmd_snapshot!("version", command().arg("version"));
}

#[test]
fn snapshots_hello() {
    assert_cmd_snapshot!("hello", command().args(["hello", "--name", "Ben"]));
}

#[test]
fn snapshots_errors() {
    assert_cmd_snapshot!("missing_command", command());
    assert_cmd_snapshot!("missing_name", command().arg("hello"));
}
