use std::str::FromStr;
use std::env;
mod utils;
mod screenshot_format;
use screenshot_format::ScreenshotFormat;
use crate::backends::utils::check_slurp_grim;
use crate::haki_error::HakiError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackendKind {
    Wayland,
    SlurpGrim,
}
impl FromStr for BackendKind {
    type Err = HakiError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "wayland"    => Ok(Self::Wayland),
            "slurp-grim" => Ok(Self::SlurpGrim),
            other        => Err(HakiError::UnknownBackend(other.to_string())),
        }
    } 
}
trait BackendFormat {
   fn capture_window(&self) -> String;
}
#[derive(Debug)]
pub struct Backend {
    kind: BackendKind,
    format: ScreenshotFormat,
    display_index: u8,
    output_path: String,
    path_image: String,
}
impl Default for Backend {
   fn default() -> Self {
      Self {
           kind: BackendKind::Wayland
          ,format: ScreenshotFormat::Region
          ,display_index: 0
          ,output_path: String::new()
          ,path_image: String::new() 
      } 
   } 
}
impl Backend {
    pub fn new(setdata:(String,String)) -> Result<Backend,HakiError> {
        let (kind,format)=setdata;
        let mut backend=Backend::default();
        backend.kind   =BackendKind::from_str(&kind)?;
        backend.format =ScreenshotFormat::from_str(&format)?;
        let home_dir   = env::var("HOME").expect("Failed to get HOME directory");
        backend.output_path=format!("{}/Pictures", home_dir);
        Ok(backend)
    }
    pub fn verify_requirements(&self) -> Result<bool,HakiError> {
       match self.kind {
            BackendKind::Wayland => {
            }
            BackendKind::SlurpGrim => {
               check_slurp_grim()?;
            }
       }
       Ok(true)
    }
}
