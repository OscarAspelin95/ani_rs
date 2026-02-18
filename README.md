# ani_rs
Ultra fast approximate genome sequence similarity, inspired by fastANI. Uses SIMD-accelerated minimizers or syncmers to build a reverse index of a database FASTA and query against it.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.90.0

## Installation

### Building from Source
For best performance, compile from source with CPU-specific optimizations. Clone the repository or download the source code, enter the ani_rs directory, and run:<br>
`cargo build --release`.

The generated binary is available in `target/release/ani_rs`.

## Usage
Run with:<br>
`ani_rs --query <query.fasta> --database <database.fasta> --outfile <out.tsv> --sketch-type <minimizer|open-syncmer|closed-syncmer>`

Accepted query/database file extensions: `.fasta`, `.fa`, `.fsa`, `.fna`.

Optional arguments:
<pre>
<b>--kmer-size</b> [15] - Kmer size for hashing.
<b>--window-size</b> [7] - Window size. Larger values = fewer hashes => faster but less sensitive.
<b>--num-hits</b> [5] - Number of top database hits to report per query. Hits with zero score are excluded.
</pre>

## Output
Results are written as a TSV file with the following columns:

| Column | Description |
|---|---|
| `query_id` | Query sequence identifier |
| `subject_id` | Database sequence identifier |
| `shared_hashes` | Number of shared hashes between query and subject |
| `score` | Fraction of query hashes shared with subject (0.0 - 1.0) |

## Memory Usage
`ani_rs` loads all database sequences into memory to build the reverse index. For very large databases, memory usage scales with the number of unique hashes. Increasing the window size will reduce the number of generated hashes at the expense of decreased sensitivity.
