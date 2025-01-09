use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::waybar::WaybarUpdate;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct TickerSubscribe {
    method: String,
    params: TickerSubscribeParams,
    req_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
struct TickerSubscribeParams {
    channel: String,
    event_trigger: Option<String>,
    symbol: Vec<String>,
    snapshot: Option<bool>,
}

impl TickerSubscribe {
    pub(crate) fn bbo(symbol: &str) -> TickerSubscribe {
        TickerSubscribe {
            method: "subscribe".to_string(),
            params: TickerSubscribeParams {
                channel: "ticker".to_string(),
                event_trigger: Some("bbo".to_string()),
                symbol: vec![symbol.to_string()],
                snapshot: None,
            },
            req_id: None,
        }
    }

    pub(crate) fn trades(symbol: &str) -> TickerSubscribe {
        TickerSubscribe {
            method: "subscribe".to_string(),
            params: TickerSubscribeParams {
                channel: "ticker".to_string(),
                event_trigger: Some("trades".to_string()),
                symbol: vec![symbol.to_string()],
                snapshot: None,
            },
            req_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct TickerUpdateData {
    pub(crate) symbol: String,
    pub(crate) bid: f64,
    pub(crate) bid_qty: f64,
    pub(crate) ask: f64,
    pub(crate) ask_qty: f64,
    pub(crate) last: f64,
    pub(crate) volume: f64,
    pub(crate) vwap: f64,
    pub(crate) low: f64,
    pub(crate) high: f64,
    pub(crate) change: f64,
    pub(crate) change_pct: f64,
}

impl From<&TickerUpdateData> for WaybarUpdate {
    fn from(value: &TickerUpdateData) -> Self {
        let trend = if value.change_pct > 0.0 { "positive" } else { "negative" }.to_string();
        return WaybarUpdate {
            text: format!("{} {:.1}", value.symbol, value.ask),
            alt: trend.clone(),
            tooltip: format!("{:.2} / {:.2}\n24h: {:+.1}%", value.bid, value.ask, value.change_pct),
            percentage: value.change_pct,
            class: trend.clone(),
            ..Default::default()
        };
    }
}
