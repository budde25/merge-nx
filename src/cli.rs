use camino::Utf8PathBuf;
use clap::{command, Parser};

#[derive(Parser)]
#[command(name = "merge-nx", author, version, about, long_about = None)]
pub struct Cli {
    /// A file part
    pub file: Utf8PathBuf,

    /// Output file
    pub output: Option<Utf8PathBuf>,

    /// Delete parts
    #[arg(short, long)]
    pub delete_parts: bool,
}
