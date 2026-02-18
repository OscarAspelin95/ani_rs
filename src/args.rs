use crate::engine::types::SketchType;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, help = "Path to query FASTA file")]
    pub query: PathBuf,

    #[arg(short, long, help = "Path to database FASTA file (to query against).")]
    pub database: PathBuf,

    #[arg(short, long, help = "Kmer size", default_value_t = 15)]
    pub kmer_size: usize,

    #[arg(short, long, help = "Window size", default_value_t = 7)]
    pub window_size: usize,

    #[arg(short, long, help = "Output file")]
    pub outfile: PathBuf,

    #[arg(short, long, help = "What kmer hash type to use", value_enum)]
    pub sketch_type: SketchType,

    #[arg(short, long, help = "Number of top hits to report per query", default_value_t = 5)]
    pub num_hits: usize,
}
