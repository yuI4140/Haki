use std::fmt;

#[derive(Debug)]
pub enum HakiError {
    /// The backend name passed by the user is not recognized.
    UnknownBackend(String),
    /// The screenshot format passed by the user is not recognized.
    UnknownFormat(String),
    /// `HOME` env var not set.
    NoHomeDir,
    /// A required external program is missing.
    MissingProgram { program: &'static str, hint: &'static str },
    /// A child process failed to spawn / returned non-zero.
    Command { program: &'static str, source: std::io::Error },
    /// A child process exited non-zero.
    CommandFailed { program: &'static str, stderr: String },
    /// Any other I/O error.
    Io(std::io::Error),
}

impl fmt::Display for HakiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownBackend(b) => write!(
                f,
                "unknown backend '{b}' (expected: wayland | slurp-grim)"
            ),
            Self::UnknownFormat(s) => write!(
                f,
                "unknown screenshot format '{s}' (expected: full-screen | window | region)"
            ),
            Self::NoHomeDir => write!(f, "HOME environment variable is not set"),
            Self::MissingProgram { program, hint } => {
                write!(f, "required program `{program}` not found => {hint}")
            }
            Self::Command { program, source } => {
                write!(f, "failed to run `{program}`: {source}")
            }
            Self::CommandFailed { program, stderr } => {
                write!(f, "`{program}` failed: {}", stderr.trim())
            }
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for HakiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Command { source, .. } => Some(source),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for HakiError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
