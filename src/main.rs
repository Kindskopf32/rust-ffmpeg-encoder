mod config;
mod ffmpeg;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-ffmpeg-encoder")]
struct Cli {
    #[arg(short, long, default_value = "./config.toml")]
    config: PathBuf,

    #[arg(value_name = "INPUT")]
    input: PathBuf,

    #[arg(value_name = "OUTPUT")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        anyhow::bail!("Input file '{}' does not exist", cli.input.display());
    }

    let config = config::load_config(&cli.config)?;

    let args = ffmpeg::build_ffmpeg_args(&config, &cli.input, &cli.output);

    ffmpeg::run_ffmpeg(args)
}
