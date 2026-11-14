use std::env;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::str;
use display_info::DisplayInfo;
mod backends;
use backends::Backend;
mod haki_error;
use clap::Parser;

use crate::haki_error::HakiError;
#[derive(Parser)]
struct Cli {
    /// The backend used to take the screenshot
    /// 
    /// Kind of backend: wayland,slurp-grim
    #[arg(short, long, default_value_t = "wayland".to_string())]
    backendkind: String,

    /// The kind of screenshot to take
    ///
    /// One of: full-screen, window, region
    #[arg(short, long, required_unless_present = "list_displays")]
    screenshot_format: Option<String>,

    /// Show all displays on the system
    #[arg(short, long, conflicts_with = "screenshot_format")]
    list_displays: bool,

    /// Index of the display to capture
    ///
    /// The index can be seen with the `--see-displays` option
    #[arg(short, long, default_value_t = 1)]
    display_index: u8,
}
//const VERSION: &str = "V0.1.6";
fn check_config() -> bool {
    if let Ok(home_env) = env::var("HOME") {
        let mut path = PathBuf::from(home_env);
        path.push(".config/haki/config.toml"); // removed leading slash so it joins correctly
        if !path.exists() {
            return false;
        }
    }
    true
}
fn parse_config_if_exists()  {
     if check_config() {
        todo!("implement the config part");
    } 
}
fn main() -> Result<(),HakiError>{
    parse_config_if_exists();
    let args = Cli::parse();
    if args.list_displays {
        let displays=DisplayInfo::all().unwrap(); 
        for i in 0..displays.len() {
           let d=displays.get(i); 
           if let Some(d) = d {
               println!("{} => {}: {}x{} (primary: {})",
               i+1,d.name, d.width, d.height, d.is_primary); 
           }
        }
    }
    if let Some(screenshot_format) = args.screenshot_format {
        let backend = Backend::new((
            args.backendkind,
            screenshot_format,
            ))?;
        _=backend.verify_requirements()?;
    }
    Ok(())
}
