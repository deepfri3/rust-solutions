
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(),Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

// args is a "Slice"; basically a fixed vector
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    //let expected = fs::read_to_string(outfile)?;
    //let mut cmd = Command::cargo_bin("echor")?;
    //cmd.arg("Hello there").assert().success().stdout(expected);
    //Ok(())
    run(&["Hello there"], &outfile)
}

#[test]
fn hello1n() -> TestResult {
    let outfile = "tests/expected/hello1.n.txt";
    //let expected = fs::read_to_string(outfile)?;
    //let mut cmd = Command::cargo_bin("echor")?;
    //cmd.args(["-n","Hello  there"]).assert().success().stdout(expected);
    //Ok(())
    run(&["-n","Hello  there"], &outfile)
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    //let expected = fs::read_to_string(outfile)?;
    //let mut cmd = Command::cargo_bin("echor")?;
    //cmd.args(["Hello","there"]).assert().success().stdout(expected);
    //Ok(())
    run(&["Hello","there"], &outfile)
}

#[test]
fn hello2n() -> TestResult {
    let outfile = "tests/expected/hello2.n.txt";
    //let expected = fs::read_to_string(outfile)?;
    //let mut cmd = Command::cargo_bin("echor")?;
    //cmd.args(["-n","Hello","there"]).assert().success().stdout(expected);
    //Ok(())
    run(&["-n","Hello","there"], &outfile)
}
