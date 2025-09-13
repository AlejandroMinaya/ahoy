use std::{fs::File, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    program: PathBuf,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let program = File::open(&args.program)?;

    Ok(())
}
