mod args;
mod errors;
mod io;
mod engine;

use args::Args;
use clap::Parser;
use engine::run;

use crate::errors::AppError;

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let _ = run(args)?;
    Ok(())
}
