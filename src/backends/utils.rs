use std::process::Command;
use crate::haki_error::HakiError;
pub fn program_exists(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
pub fn check_slurp_grim() -> Result<(), HakiError> {
    if !program_exists("slurp") {
        return Err(HakiError::MissingProgram {
            program: "slurp",
            hint: "install slurp on your distro",
        });
    }
    if !program_exists("grim") {
        return Err(HakiError::MissingProgram {
            program: "grim",
            hint: "install grim on your distro",
        });
    }
    Ok(())
}
