use crate::config::Config;

pub fn build_ffmpeg_args(
    config: &Config,
    input: &std::path::Path,
    output: &std::path::Path,
) -> Vec<String> {
    vec![
        "-i".to_string(),
        input.display().to_string(),
        "-c:v".to_string(),
        config.video.codec.clone(),
        "-crf".to_string(),
        config.video.crf.to_string(),
        "-c:a".to_string(),
        config.audio.codec.clone(),
        "-b:a".to_string(),
        config.audio.bitrate.clone(),
        output.display().to_string(),
    ]
}

pub fn run_ffmpeg(args: Vec<String>) -> anyhow::Result<()> {
    let status = std::process::Command::new("ffmpeg")
        .args(&args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to execute ffmpeg: {}", e))?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, VideoConfig, AudioConfig};

    fn create_test_config() -> Config {
        Config {
            video: VideoConfig {
                codec: "libx264".to_string(),
                crf: 23,
            },
            audio: AudioConfig {
                codec: "aac".to_string(),
                bitrate: "128k".to_string(),
            },
        }
    }

    #[test]
    fn test_build_ffmpeg_args() {
        let config = create_test_config();
        let input = std::path::Path::new("input.mp4");
        let output = std::path::Path::new("output.mp4");
        
        let args = build_ffmpeg_args(&config, input, output);
        
        assert_eq!(args[0], "-i");
        assert_eq!(args[1], "input.mp4");
        assert_eq!(args[2], "-c:v");
        assert_eq!(args[3], "libx264");
        assert_eq!(args[4], "-crf");
        assert_eq!(args[5], "23");
        assert_eq!(args[6], "-c:a");
        assert_eq!(args[7], "aac");
        assert_eq!(args[8], "-b:a");
        assert_eq!(args[9], "128k");
        assert_eq!(args[10], "output.mp4");
    }

    #[test]
    fn test_build_ffmpeg_args_paths() {
        let config = create_test_config();
        let input = std::path::Path::new("/tmp/test input.mp4");
        let output = std::path::Path::new("/tmp/test output.mp4");
        
        let args = build_ffmpeg_args(&config, input, output);
        
        assert_eq!(args[1], "/tmp/test input.mp4");
        assert_eq!(args[10], "/tmp/test output.mp4");
    }
}
