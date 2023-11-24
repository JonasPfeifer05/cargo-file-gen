use anyhow::bail;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("Checking if the arguments are valid");
    if args.ascii && args.lorem {
        debug!("Encountered ascii and valid in the same call");
        error!("You cannot provide ascii and lorem at the same time");

        bail!("Got --ascii and --lorem at the same time!");
    }

    Ok(())
}
