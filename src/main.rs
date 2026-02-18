mod args;
mod errors;
mod io;
mod ultra_ani;

use args::Args;
use clap::Parser;
use ultra_ani::run;

use crate::errors::AppError;

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let _ = run(args)?;
    Ok(())
}
