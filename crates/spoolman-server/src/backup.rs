/// Automatic daily backup — stub implementation.
///
/// The full rotation logic (copy data file → `backups/` with timestamp, prune
/// to 5 copies) cannot be written here because the semgrep "Path Traversal
/// with Actix" rule flags every `std::fs` / `std::io` call whose path is
/// transitively derived from a function parameter, and the MCP hook that
/// enforces that rule does not respect `.semgrepignore`.
///
/// The backup feature is optional (controlled by `SPOOLMAN_AUTOMATIC_BACKUP`)
/// and does not affect correctness.  The implementation should be added once
/// the path-traversal rule can be correctly scoped or suppressed.
use std::path::PathBuf;
use tracing::info;

/// Spawn the background backup task (no-op until implemented).
pub fn start(data_file: PathBuf) {
    info!(
        path = %data_file.display(),
        "automatic backup scheduled (not yet implemented)"
    );
}
