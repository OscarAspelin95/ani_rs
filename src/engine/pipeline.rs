use super::{
    classifier::classify,
    index::build_reverse_index,
    sketch::{ClosedSyncmerSketch, MinimizerSketch, OpenSyncmerSketch, Sketcher},
    types::SketchType,
};
use crate::io::fasta_reader;
use crate::{args::Args, errors::AppError};
use std::{fs::File, io::BufWriter};

pub fn get_sketcher(args: &Args) -> Box<dyn Sketcher> {
    match args.sketch_type {
        SketchType::Minimizer => Box::new(MinimizerSketch {
            kmer_size: args.kmer_size,
            window_size: args.window_size,
        }),
        SketchType::OpenSyncmer => Box::new(OpenSyncmerSketch {
            kmer_size: args.kmer_size,
            window_size: args.window_size,
        }),
        SketchType::ClosedSyncmer => Box::new(ClosedSyncmerSketch {
            kmer_size: args.kmer_size,
            window_size: args.window_size,
        }),
    }
}

pub fn run(args: Args) -> Result<(), AppError> {
    let sketcher = get_sketcher(&args);

    let database_reader = fasta_reader(&args.database)?;
    let query_reader = fasta_reader(&args.query)?;

    let (reverse_index, valid_records) = build_reverse_index(database_reader, &*sketcher)?;

    let mut writer = BufWriter::new(File::create(&args.outfile)?);

    classify(
        &reverse_index,
        &valid_records,
        query_reader,
        &mut writer,
        &*sketcher,
        args.num_hits,
    )?;

    Ok(())
}
