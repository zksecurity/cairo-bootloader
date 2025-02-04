use cairo_bootloader::{
    error::Error,
    run::{run_bootloader, RunBootloaderArgs},
};
use clap::Parser;

fn main() -> Result<(), Error> {
    let args = RunBootloaderArgs::try_parse_from(std::env::args()).map_err(Error::Cli)?;

    run_bootloader(args)
}
