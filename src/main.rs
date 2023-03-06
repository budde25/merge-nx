use clap::Parser;

use crate::nxfile::NxFile;

mod cli;
mod error;
mod filetype;
mod nxfile;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    let nx_file = NxFile::from_path_buf(args.file)?;
    nx_file.merge(args.output, args.delete_parts)?;

    Ok(())
}
