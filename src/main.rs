use cargo_file_gen::error::FileGenError;
use cargo_file_gen::file_size::FileSize;
use clap::{arg, Args, Parser};
use clap_verbosity_flag::Verbosity;
use color_eyre::eyre::{bail, Context};
use log::{debug, error, info, trace};
use rand::Rng;
use std::fs;
use std::path::PathBuf;

const AVG_LIPSUM_WORD_LENGTH: f64 = 6.793286;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    FileGen(FeaturesArgs)
}

#[derive(Args, Debug)]
struct FeaturesArgs {
    path: PathBuf,
    size: String,

    #[arg(long)]
    ascii: bool,
    #[arg(long)]
    lorem: bool,

    #[command(flatten)]
    verbose: Verbosity,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let CargoCli::FileGen(args) = CargoCli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    check_path(&args)?;
    check_argument_compatability(&args)?;
    let size = parse_size(&args)?;

    debug!(
        "Starting to generate file with {} bytes!",
        size.byte_amount()
    );

    if let Some(parent) = args.path.parent() {
        fs::create_dir_all(parent)?;
    }

    let data;
    if args.ascii {
        let mut random: Vec<f32> = vec![0f32; size.byte_amount() as usize];
        rand::thread_rng().fill(random.as_mut_slice());

        data = random
            .into_iter()
            .map(|random| 21 + (random * (126.0 - 21.0)) as u8)
            .collect();
    } else if args.lorem {
        info!(
            "Generating array consisting of lorem ipsum with min length of {}",
            size.byte_amount()
        );

        let mut content = String::with_capacity(size.byte_amount() as usize);
        while content.len() < size.byte_amount() as usize {
            debug!("Content wasn't big enough");
            content.push_str(&lipsum::lipsum(
                ((size.byte_amount() as usize - content.len()) as f64 / AVG_LIPSUM_WORD_LENGTH)
                    as usize,
            ));
        }
        data = content.into_bytes();
    } else {
        info!("Generating 0 byte array with length {}", size.byte_amount());
        data = vec![0u8; size.byte_amount() as usize];
    }
    info!("Writing data into file");
    fs::write(args.path, &data[0..(size.byte_amount() as usize)])
        .with_context(|| "Failed to write data to file")?;
    info!("Succeeded to write data to file");

    info!("Program succeeded");
    Ok(())
}

fn check_path(args: &FeaturesArgs) -> color_eyre::Result<()> {
    trace!("Calling check_path");

    debug!("Checking if the path is valid");
    if args.path.extension().is_none() {
        error!("Didnt pass a file as path");
        bail!(FileGenError::DirectoryPassedError)
    }

    info!("Path is valid");

    trace!("Returning from check_path");
    Ok(())
}

fn check_argument_compatability(args: &FeaturesArgs) -> color_eyre::Result<()> {
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

fn parse_size(args: &FeaturesArgs) -> color_eyre::Result<FileSize> {
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
