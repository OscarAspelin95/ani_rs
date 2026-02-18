use crate::errors::AppError;
use bio::io::fasta::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const VALID_FASTA_EXTENSIONS: &[&str] = &["fasta", "fa", "fsa", "fna"];

fn validate_fasta_extension(f: &Path) -> Result<(), AppError> {
    let ext = f.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !VALID_FASTA_EXTENSIONS.contains(&ext) {
        return Err(AppError::InvalidFileExtension(f.display().to_string()));
    }
    Ok(())
}

pub fn fasta_reader(f: &Path) -> Result<Reader<BufReader<File>>, AppError> {
    if !f.is_file() {
        return Err(AppError::FileNotFoundError(f.display().to_string()));
    }
    validate_fasta_extension(f)?;
    let reader = Reader::from_file(f)
        .map_err(|e| AppError::FastaReadError(format!("{}: {}", f.display(), e)))?;
    Ok(reader)
}
