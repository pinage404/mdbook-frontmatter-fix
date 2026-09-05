//! Helpers for querying `git log` metadata for a specific file.

use std::path::Path;
use std::process::Command;

use crate::error::InjectError;

/// Git metadata for a single file at a particular commit.
#[derive(Debug, Clone)]
pub struct FileCommitInfo {
    /// The commit author's name (not email).
    pub author: String,
    /// The commit date, formatted according to the caller's `date_format`.
    pub date: String,
}

/// Query git for the author and date of a file's most-recent (or first) commit.
///
/// # Arguments
///
/// - `abs_path`: Absolute path to the source file on disk.
/// - `date_format`: `strftime`-style format string, e.g. `"%Y-%m-%d"`.
/// - `use_first_commit`: When `true`, returns the file's *first* commit
///   instead of the latest.
///
/// # Errors
///
/// Returns `Err` if the git subprocess fails to spawn or exits with a
/// non-zero status. Returns `Ok(None)` if the file has no commit history.
pub fn file_commit_info(
    abs_path: &Path,
    date_format: &str,
    use_first_commit: bool,
) -> Result<Option<FileCommitInfo>, InjectError> {
    // Determine the working directory: the file's parent (or cwd as fallback).
    let work_dir = abs_path.parent().unwrap_or_else(|| Path::new("."));

    // Build the git log command.
    //
    // --follow  — track renames across history
    // -1        — only the most recent commit (omitted for --reverse)
    // --format  — custom output: author name NUL date NUL
    let date_arg = format!("--date=format:{date_format}");
    let format_arg = "--format=%aN%x00%ad%x00".to_string();

    let mut cmd = Command::new("git");
    cmd.arg("log")
        .arg("--follow")
        .arg(&date_arg)
        .arg(&format_arg);

    if use_first_commit {
        // Reverse the log so the oldest commit comes first, then take one.
        cmd.arg("--reverse").arg("-1");
    } else {
        cmd.arg("-1");
    }

    cmd.arg("--").arg(abs_path);
    cmd.current_dir(work_dir);

    let output = cmd.output().map_err(|source| InjectError::Io {
        path: abs_path.to_owned(),
        source,
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(InjectError::GitFailure {
            path: abs_path.to_owned(),
            reason: if stderr.is_empty() {
                format!("exit code {}", output.status)
            } else {
                stderr
            },
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Output is "author\0date\0\n"; split on NUL.
    let mut parts = stdout.splitn(3, '\x00');
    let author = parts.next().unwrap_or("").trim().to_string();
    let date = parts.next().unwrap_or("").trim().to_string();

    if author.is_empty() && date.is_empty() {
        // File has no commits yet (untracked or brand-new).
        return Ok(None);
    }

    Ok(Some(FileCommitInfo { author, date }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: querying a non-existent file should return a `GitFailure` or
    /// Ok(None), never panic.
    #[test]
    fn missing_file_does_not_panic() {
        let result = file_commit_info(
            Path::new("/tmp/definitely-does-not-exist.md"),
            "%Y-%m-%d",
            false,
        );
        // Either an error or Ok(None) is acceptable.
        match result {
            Ok(None) | Err(_) => {}
            Ok(Some(_)) => panic!("unexpected success for missing file"),
        }
    }
}
