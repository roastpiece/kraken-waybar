use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct WaybarUpdate {
    pub(crate) text: String,
    pub(crate) alt: String,
    pub(crate) tooltip: String,
    pub(crate) class: String,
    pub(crate) percentage: f64,
}

impl Display for WaybarUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
