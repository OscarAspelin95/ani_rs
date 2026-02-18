use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum SketchType {
    Minimizer,
    OpenSyncmer,
    ClosedSyncmer,
}
