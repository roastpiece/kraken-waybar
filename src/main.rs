mod ticker;
mod kraken;
mod waybar;

use std::env;
use kraken::{AckResponse, DataResponse};
use waybar::WaybarUpdate;
use websocket::{ClientBuilder, Message, OwnedMessage};
use ticker::{TickerSubscribe, TickerUpdateData};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <symbol>", args[0]);
        return;
    }

    let mut client = ClientBuilder::new("wss:///ws.kraken.com/v2")
        .unwrap()
        .connect_secure(None)
        .unwrap();

    let subscribe_message =
        Message::text(serde_json::to_string(&TickerSubscribe::bbo(&args[1])).unwrap());

    client.send_message(&subscribe_message).unwrap();

    for message in client.incoming_messages() {
        if let Err(message) = message {
            println!("Error: {:?}", message);
            break;
        }

        if let Ok(message) = message {
            match message {
                OwnedMessage::Text(text) =>{
                    if let Ok(_ack) = serde_json::from_str::<AckResponse>(&text) {
                        // ignored
                    }

                    if let Ok(data) = serde_json::from_str::<DataResponse>(&text) {
                        match data.channel.as_str() {
                            "ticker" => {
                                let ticker_data: TickerUpdateData = serde_json::from_value(data.data[0].clone()).unwrap();
                                let waybar_update = WaybarUpdate::from(&ticker_data);
                                println!("{}", waybar_update);
                            },
                            _ => {}
                        }
                    
                    }
                },
                OwnedMessage::Close(_) => break,
                _ => {}
            }
        }
    }
}

