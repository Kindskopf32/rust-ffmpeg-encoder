# Rust FFmpeg Encoder

A simple CLI tool to encode videos using FFmpeg with TOML-based configuration.
Made with the help of GLM4.7

## Features

- Configure video and audio codecs via TOML file
- Specify CRF value for video quality
- Set audio bitrate
- Input file validation
- Direct passthrough of FFmpeg output and errors

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd rust-ffmpeg-encoder

# Build in release mode
cargo build --release

# The binary will be available at target/release/rust-ffmpeg-encoder
```

## Requirements

- [FFmpeg](https://ffmpeg.org/) must be installed and available in PATH
  - Ubuntu/Debian: `sudo apt install ffmpeg`
  - macOS: `brew install ffmpeg`
  - Windows: Download from [ffmpeg.org](https://ffmpeg.org/download.html)

## Usage

```bash
# Basic usage (uses ./config.toml)
rust-ffmpeg-encoder input.mp4 output.mp4

# Custom config file
rust-ffmpeg-encoder --config /path/to/config.toml input.mp4 output.mp4

# Show help
rust-ffmpeg-encoder --help
```

## Configuration

Create a `config.toml` file in the same directory or specify a custom path:

```toml
[video]
codec = "libx264"
crf = 23

[audio]
codec = "aac"
bitrate = "128k"
```

### Config Options

#### Video Settings
- `codec`: Video codec (e.g., `libx264`, `libx265`, `libvpx-vp9`)
- `crf`: Constant Rate Factor (0-51, lower is better quality, 23 is default for libx264)

#### Audio Settings
- `codec`: Audio codec (e.g., `aac`, `libmp3lame`, `libopus`)
- `bitrate`: Audio bitrate (e.g., `128k`, `192k`, `320k`)

## Examples

### Encode with H.264
```bash
rust-ffmpeg-encoder input.mkv output.mp4
```

### Use a different config
```bash
rust-ffmpeg-encoder --config high_quality.toml input.mp4 output.mp4
```

## Development

### Running Tests

```bash
# Run all tests (requires FFmpeg installed)
cargo test

# Run specific test
cargo test test_actual_encode

# Run with output
cargo test -- --nocapture
```

### Test Coverage

The project includes:
- Unit tests for config parsing and argument building
- Integration tests that run actual FFmpeg encodes
- Test fixtures for various config scenarios

## Project Structure

```
rust-ffmpeg-encoder/
├── Cargo.toml
├── config.toml.example
├── README.md
├── src/
│   ├── main.rs       # CLI entry point
│   ├── config.rs     # Config file parsing
│   └── ffmpeg.rs     # FFmpeg execution
└── tests/
    ├── fixtures/     # Test config files
    └── integration_tests.rs
```

## Error Handling

- Input file validation: Returns clear error if input file doesn't exist
- Config errors: Detailed messages for missing or invalid config files
- FFmpeg errors: Passes through FFmpeg's exit codes and error messages

