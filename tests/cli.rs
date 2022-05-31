use assert_cmd::Command;
use std::fs;
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_message() -> TestResult {
    let input = Path::new("tests").join("input.xml");
    let input = fs::read_to_string(input)?;
    let mut cmd = Command::cargo_bin("extractbyxpath")?;
    cmd.write_stdin(input)
        .arg("message")
        .assert()
        .success()
        .stdout("OK");
    Ok(())
}

#[test]
fn test_image1() -> TestResult {
    let input = Path::new("tests").join("input.xml");
    let input = fs::read_to_string(input)?;
    let mut cmd = Command::cargo_bin("extractbyxpath")?;
    cmd.write_stdin(input)
        .arg("image[1]")
        .assert()
        .success()
        .stdout("image_1.jpg");
    Ok(())
}

#[test]
fn test_category_piyo() -> TestResult {
    let input = Path::new("tests").join("input.xml");
    let input = fs::read_to_string(input)?;
    let mut cmd = Command::cargo_bin("extractbyxpath")?;
    cmd.write_stdin(input)
        .arg("categoryInfo[2]/categoryName")
        .assert()
        .success()
        .stdout("piyo");
    Ok(())
}

