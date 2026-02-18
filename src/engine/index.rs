use crate::engine::sketch::Sketcher;
use crate::errors::AppError;
use bio::io::fasta::{Reader, Record};
use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn build_reverse_index(
    database_reader: Reader<BufReader<File>>,
    sketcher: &dyn Sketcher,
) -> Result<(DashMap<u64, FixedBitSet, FxBuildHasher>, Vec<Record>), AppError> {
    let valid_records: Vec<Record> = database_reader
        .records()
        .filter_map(|record| record.ok())
        .collect();

    let num_records = valid_records.len();
    let map = DashMap::with_capacity_and_hasher(num_records, FxBuildHasher);

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]")?);

    valid_records.par_iter().enumerate().for_each(|(i, r)| {
        let hashes = sketcher.sketch(r.seq());

        for h in &hashes {
            map.entry(*h)
                .and_modify(|bitset: &mut FixedBitSet| bitset.set(i, true))
                .or_insert_with(|| {
                    let mut bitset = FixedBitSet::with_capacity(num_records);
                    bitset.set(i, true);
                    bitset
                });
        }
    });

    spinner.finish();

    Ok((map, valid_records))
}
