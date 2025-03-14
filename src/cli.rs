use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of post to submit to smol (becomes slug of post)
    file_path: PathBuf,
}

pub fn parse_args() -> PathBuf {
    let args = Args::parse();
    args.file_path
}
