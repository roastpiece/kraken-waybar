use std::{
    error::Error, fmt::{Debug, Display}, process::Termination
};

use crate::{kraken::AckResponse, waybar::WaybarUpdate};

#[allow(dead_code)]
pub(crate) enum ExitResult {
    Ok,
    MissingSymbolArgument,
    Disconnected,
    WebSocketError(Box<dyn Error>),
    ApiError(AckResponse),
}

impl Display for ExitResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitResult::MissingSymbolArgument => write!(
                f,
                "Usage: kraken-waybar <symbol> // e.g. kraken-waybar ETH/CHF"
            ),
            ExitResult::Disconnected => write!(f, "Disconnected"),
            ExitResult::WebSocketError(e) => write!(f, "{}", e),
            ExitResult::ApiError(e) => write!(
                f,
                "{}: {}",
                e.method,
                e.error.as_ref().unwrap_or(&"Unknown error".to_string())
            ),
            ExitResult::Ok => Ok(()),
        }
    }
}

impl Debug for ExitResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitResult::MissingSymbolArgument => write!(f, "MissingSymbolArgument"),
            ExitResult::Disconnected => write!(f, "Disconnected"),
            ExitResult::WebSocketError(_) => write!(f, "WebSocketError"),
            ExitResult::ApiError(_) => write!(f, "ApiError"),
            ExitResult::Ok => write!(f, "Ok"),
        }
    }
}

impl Termination for ExitResult {
    fn report(self) -> std::process::ExitCode {
        match self {
            ExitResult::Ok => std::process::ExitCode::SUCCESS,
            e => {
                println!("{}", WaybarUpdate::from(e));
                // if we exit with non 0 code here, waybar wont show the module
                std::process::ExitCode::SUCCESS
            }
        }
    }
}

impl From<ExitResult> for WaybarUpdate {
    fn from(value: ExitResult) -> Self {
        WaybarUpdate {
            text: format!("Error: {:?}", value),
            tooltip: format!("{}", value),
            class: "error".to_owned(),
            alt: "error".to_owned(),
            ..Default::default()
        }
    }
}
