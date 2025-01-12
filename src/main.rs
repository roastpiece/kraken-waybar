mod error;
mod kraken;
mod ticker;
mod waybar;

use error::ExitResult;
use kraken::{AckResponse, DataResponse};
use std::env;
use ticker::{TickerSubscribe, TickerUpdateData};
use tungstenite::{connect, http::Uri, ClientRequestBuilder, Message};
use waybar::WaybarUpdate;

fn main() -> ExitResult {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return ExitResult::MissingSymbolArgument;
    }

    let uri: Uri = "wss://ws.kraken.com/v2".parse().unwrap();
    let builder = ClientRequestBuilder::new(uri);

    let (mut socket, _) = connect(builder).unwrap();
    let subscribe_message = TickerSubscribe::bbo(&args[1]);

    if let Err(e) = socket.send(subscribe_message.into()) {
        return ExitResult::WebSocketError(e.into());
    }

    loop {
        match socket.read() {
            Ok(message) => match message {
                Message::Text(text) => {
                    if let Ok(ack) = serde_json::from_str::<AckResponse>(&text) {
                        if !ack.success {
                            return ExitResult::ApiError(ack);
                        }
                    }

                    if let Ok(data) = serde_json::from_str::<DataResponse>(&text) {
                        match data.channel.as_str() {
                            "ticker" => {
                                let ticker_data: Vec<TickerUpdateData> =
                                    serde_json::from_value(data.data).unwrap();
                                let waybar_update = WaybarUpdate::from(&ticker_data[0]);
                                println!("{waybar_update}");
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
            Err(e) => return ExitResult::WebSocketError(e.into()),
        }
    }

    return ExitResult::Disconnected;
}
