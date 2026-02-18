use crate::errors::AppError;
use crate::ultra_ani::sketch::Sketcher;
use bio::io::fasta::{Reader, Record};
use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::collections::HashSet;
use std::io::Write;
use std::time::Duration;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    sync::{Arc, Mutex},
};

pub fn classify(
    reverse_index: &DashMap<u64, FixedBitSet, FxBuildHasher>,
    valid_records: &[Record],
    query_reader: Reader<BufReader<File>>,
    writer: Arc<Mutex<BufWriter<File>>>,
    sketcher: Box<dyn Sketcher>,
) -> Result<(), AppError> {
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template(
        "{spinner:.blue} [{elapsed_precise}]",
    )?);

    query_reader.records().par_bridge().for_each(|record| {
        // -- FOR A GIVEN RECORD.
        if let Ok(r) = record {
            let query_hashes: HashSet<u64> = sketcher.sketch(r.seq());
            let num_query_hashes = query_hashes.len();

            // Iintialize a vec where the index is the db id and the value is how many query hashes the db seq contains so far
            let mut hits: Vec<usize> = vec![0; valid_records.len()];

            // For each query hash...
            query_hashes.iter().for_each(|hash| {
                // get bitset if exists.
                reverse_index.entry(*hash).and_modify(|bitset| {
                    // for each db id -> increase it's count
                    for idx in bitset.ones() {
                        hits[idx] += 1;
                    }
                });
            });

            // For now, we only take the best hit.
            if let Some((best_idx, best_value)) =
                hits.into_iter().enumerate().max_by_key(|(_, value)| *value)
            {
                let mut w = writer.lock().expect("Mutex lock failed.");

                let query_id = r.id();
                let db_id = valid_records[best_idx].id();
                let score = best_value as f64 / num_query_hashes as f64;
                writeln!(w, "{}\t{}\t{}\t{}", query_id, db_id, best_value, score)
                    .expect("Failed to write results to file.")
            }
        }
    });

    let mut writer = Arc::into_inner(writer)
        .expect("Failed to extract inner value from Arc<Mutex<writer>>")
        .into_inner()
        .expect("Corrupt mutex");
    writer.flush()?;

    spinner.finish();

    Ok(())
}
