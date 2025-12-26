use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoConfig {
    pub codec: String,
    pub crf: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioConfig {
    pub codec: String,
    pub bitrate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub video: VideoConfig,
    pub audio: AudioConfig,
}

pub fn load_config(path: &std::path::Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file '{}': {}", path.display(), e))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_config_valid() {
        let path = PathBuf::from("tests/fixtures/config_valid.toml");
        let config = load_config(&path).unwrap();
        assert_eq!(config.video.codec, "libx264");
        assert_eq!(config.video.crf, 23);
        assert_eq!(config.audio.codec, "aac");
        assert_eq!(config.audio.bitrate, "128k");
    }

    #[test]
    fn test_load_config_missing_file() {
        let path = PathBuf::from("tests/fixtures/nonexistent.toml");
        let result = load_config(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_invalid_toml() {
        let path = PathBuf::from("tests/fixtures/config_invalid_toml.toml");
        let result = load_config(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_missing_fields() {
        let path = PathBuf::from("tests/fixtures/config_missing_audio.toml");
        let result = load_config(&path);
        assert!(result.is_err());
    }
}
