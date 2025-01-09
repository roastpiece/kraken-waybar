use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct WaybarUpdate {
    pub(crate) text: String,
    pub(crate) alt: String,
    pub(crate) tooltip: String,
    pub(crate) class: String,
    pub(crate) percentage: f64,
}
