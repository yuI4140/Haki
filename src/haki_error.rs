use std::fmt;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum HakiError {
    #[error("unknown backend '{0}' (expected: wayland | slurp-grim)")]
    UnknownBackend(String),

    #[error("unknown screenshot format '{0}'
        (expected: full-screen | window | region)")]
    UnknownFormat(String),

    #[error("HOME environment variable is not set")]
    NoHomeDir,
    #[error("Wayland socket is not found")]
    WaylandSocketNotFound,

    #[error("required program `{program}` not found => {hint}")]
    MissingProgram { program: &'static str, hint: &'static str },

    #[error("failed to run `{program}`: {source}")]
    Command { program: &'static str, source: std::io::Error },

    #[error("'{program}' failed: {stderr}")]
    CommandFailed { program: &'static str, stderr: String },

    #[error("I/O error: {0}")]
    Io(std::io::Error),
}
