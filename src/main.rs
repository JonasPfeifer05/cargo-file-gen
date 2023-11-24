use cargo_file_gen::error::FileGenError;
use cargo_file_gen::file_size::FileSize;
use clap::{arg, Parser};
use clap_verbosity_flag::Verbosity;
use color_eyre::eyre::bail;
use log::{debug, error, info, trace};
use std::fs;
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

    check_argument_compatability(&args)?;
    let size = parse_size(&args)?;

    debug!(
        "Starting to generate file with {} bytes!",
        size.byte_amount()
    );

    Ok(())
}

fn check_argument_compatability(args: &Args) -> color_eyre::Result<()> {
    trace!("Calling check_argument_compatability");

    debug!("Checking if the arguments are valid");
    if args.ascii && args.lorem {
        debug!("Encountered ascii and lorem in the same call");
        error!("You cannot provide ascii and lorem at the same time");

        bail!(FileGenError::IncompatibleAsciiLoremError);
    }
    info!("Arguments are compatible");

    trace!("Returning from check_argument_compatability");
    Ok(())
}

fn parse_size(args: &Args) -> color_eyre::Result<FileSize> {
    trace!("Calling parse_size");

    debug!("Starting to parse size");
    let file_size: Result<FileSize, FileGenError> = args.size.clone().try_into();
    let file_size = match file_size {
        Ok(file_size) => {
            info!("Size was valid");
            file_size
        }
        Err(err) => {
            error!("Failed to parse size");
            bail!(err)
        }
    };

    trace!("Returning from check_argument_compatability");
    Ok(file_size)
}
