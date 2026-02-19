use super::{classifier::classify, types::SketchType};
use crate::io::fasta_reader;
use crate::{args::Args, errors::AppError};
use bio::io::fasta::Record;
use bio_utils_rs::simd_sketch::{
    ClosedSyncmerSketch, MinimizerSketch, OpenSyncmerSketch, Sketcher, build_reverse_index,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
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

    let valid_records: Vec<Record> = database_reader.records().filter_map(|r| r.ok()).collect();

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template(
        "Loading database and building reverse index {spinner:.blue} [{elapsed_precise}]",
    )?);

    let seqs: Vec<&[u8]> = valid_records.iter().map(|r| r.seq()).collect();
    let reverse_index = build_reverse_index(&seqs, &*sketcher);

    spinner.finish();

    let mut writer = BufWriter::new(File::create(&args.outfile)?);

    classify(
        &reverse_index,
        &valid_records,
        query_reader,
        &mut writer,
        &*sketcher,
        args.num_hits,
        args.min_score,
    )?;

    Ok(())
}
