use cargo_file_gen::error::FileGenError;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use color_eyre::eyre::bail;
use log::{debug, error};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    size: String,

    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    #[arg(long)]
    ascii: bool,
    #[arg(long)]
    lorem: bool,

    #[command(flatten)]
    verbose: Verbosity,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("Checking if the arguments are valid");
    if args.ascii && args.lorem {
        debug!("Encountered ascii and lorem in the same call");
        error!("You cannot provide ascii and lorem at the same time");

        bail!(FileGenError::IncompatibleArgumentsException);
    }

    Ok(())
}
