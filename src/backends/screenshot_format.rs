use std::str::FromStr;
use crate::haki_error::HakiError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenshotFormat {
    FullScreen,
    Window,
    Region,
}

impl FromStr for ScreenshotFormat {
    type Err = HakiError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "full-screen" => Ok(Self::FullScreen),
            "window"      => Ok(Self::Window),
            "region"      => Ok(Self::Region),
            other         => Err(HakiError::UnknownFormat(other.to_string())),
        }
    }
}
