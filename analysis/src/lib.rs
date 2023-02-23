pub use self::search::{analyze, Analysis, AnalysisConfig, PersistentState, Statistics};
pub use self::transposition_table::{TranspositionTable, TranspositionTableEntry};

pub mod evaluation;
mod plies;
mod search;
mod transposition_table;
mod util;

pub fn version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("(unknown version)")
}
