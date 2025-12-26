use assert_cmd::cargo::cargo_bin_cmd;
use std::path::PathBuf;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = cargo_bin_cmd!("rust-ffmpeg-encoder");
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_missing_input_file() {
    let config = PathBuf::from("tests/fixtures/config_valid.toml");
    let mut cmd = cargo_bin_cmd!("rust-ffmpeg-encoder");
    cmd.args([
        "--config", config.to_str().unwrap(),
        "nonexistent.mp4", "output.mp4"
    ]);
    cmd.assert().failure().stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_missing_config_file() {
    let mut cmd = cargo_bin_cmd!("rust-ffmpeg-encoder");
    cmd.args([
        "--config", "nonexistent.toml",
        "input.mp4", "output.mp4"
    ]);
    cmd.assert().failure();
}

#[test]
fn test_missing_config_arg_uses_default() {
    let mut cmd = cargo_bin_cmd!("rust-ffmpeg-encoder");
    cmd.args(["nonexistent.mp4", "output.mp4"]);
    cmd.assert().failure();
}

#[test]
fn test_actual_encode() {
    let output_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&output_dir).ok();

    let input = output_dir.join("test_input.mp4");
    let output = output_dir.join("test_output.mp4");

    let create_input = std::process::Command::new("ffmpeg")
        .args([
            "-f", "lavfi",
            "-i", "sine=frequency=1000:duration=2",
            "-f", "lavfi",
            "-i", "testsrc=duration=2:size=320x240:rate=30",
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-crf", "23",
            "-c:a", "aac",
            "-b:a", "128k",
            "-shortest",
            "-y",
            input.to_str().unwrap(),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    assert!(create_input.unwrap().success(), "Failed to create test input video");

    let config = PathBuf::from("tests/fixtures/config_valid.toml");
    let mut cmd = cargo_bin_cmd!("rust-ffmpeg-encoder");
    cmd.args([
        "--config", config.to_str().unwrap(),
        input.to_str().unwrap(),
        output.to_str().unwrap(),
    ]);
    cmd.assert().success();
    assert!(output.exists(), "Output file was not created");

    let check_output = std::process::Command::new("ffmpeg")
        .args(["-i", output.to_str().unwrap()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output();

    let check_result = check_output.unwrap();
    let stderr = String::from_utf8_lossy(&check_result.stderr);
    assert!(stderr.contains("Video: h264") || stderr.contains("Video: H264"), "Video codec not h264");
    assert!(stderr.contains("Audio: aac") || stderr.contains("Audio: AAC"), "Audio codec not aac");
}
