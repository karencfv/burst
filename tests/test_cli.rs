use assert_cmd::prelude::*;
use mockito;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn host_not_specified() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("burst")?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "The following required arguments were not provided:",
    ));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--host <host>"));

    Ok(())
}

#[test]
fn cmd_success() -> Result<(), Box<dyn std::error::Error>> {
    let host = &mockito::server_url();
    let mut cmd = Command::cargo_bin("burst")?;

    cmd.arg("--host").arg(host);
    cmd.assert().success();
    Ok(())
}

#[test]
fn cmd_success_with_exact_duration_and_interval() -> Result<(), Box<dyn std::error::Error>> {
    let host = &mockito::server_url();
    let mut cmd = Command::cargo_bin("burst")?;

    cmd.arg("--host").arg(host);
    cmd.arg("-u").arg("spongebob");
    cmd.arg("-p").arg("supersekretpassword");
    cmd.arg("-l").arg("3");
    cmd.arg("-d").arg("2");
    cmd.arg("-i").arg("1");
    cmd.arg("-t").arg("1");
    cmd.arg("-w").arg("5");
    cmd.arg("-e");
    cmd.arg("-v");
    cmd.assert().success().stdout(predicate::str::contains(
        "Sending requests and will exit in 2 seconds...",
    ));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Pausing for 1 second"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Request ID:"));
    Ok(())
}

#[test]
fn cmd_success_with_duration() -> Result<(), Box<dyn std::error::Error>> {
    let host = &mockito::server_url();
    let mut cmd = Command::cargo_bin("burst")?;

    cmd.arg("--host").arg(host);
    cmd.arg("-u").arg("spongebob");
    cmd.arg("-p").arg("supersekretpassword");
    cmd.arg("-l").arg("1");
    cmd.arg("-d").arg("2");
    cmd.arg("-t").arg("1");
    cmd.arg("-w").arg("1");
    cmd.arg("-v");
    cmd.assert().success().stdout(predicate::str::contains(
        "Sending requests for 2 seconds...",
    ));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Request ID:"));
    Ok(())
}

#[test]
fn cmd_success_with_set_load() -> Result<(), Box<dyn std::error::Error>> {
    let host = &mockito::server_url();
    let mut cmd = Command::cargo_bin("burst")?;

    cmd.arg("--host").arg(host);
    cmd.arg("-u").arg("spongebob");
    cmd.arg("-p").arg("supersekretpassword");
    cmd.arg("-l").arg("3");
    cmd.arg("-t").arg("1");
    cmd.arg("-v");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Sending 3 requests..."));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Request ID:"));
    Ok(())
}
