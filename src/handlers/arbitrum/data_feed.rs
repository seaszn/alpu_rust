use futures::{SinkExt, StreamExt};
use std::time::Instant;

use websocket_lite::{ClientBuilder, Message, Opcode, Result};

use crate::{env, handlers::arbitrum::types::RelayMessage};

pub async fn init() -> Result<()> {
    let builder = ClientBuilder::from_url(env::RUNTIME_CONFIG.feed_endpoint.clone());
    let mut stream = builder.async_connect().await?;

    while let Some(msg) = stream.next().await {
        if let Ok(m) = msg {
            match m.opcode() {
                Opcode::Text => {
                    let now = Instant::now();

                    let result = RelayMessage::from(m.as_text().unwrap());
                    if result.is_some() {
                        // _ = task::spawn(async{
                        handle_relay_message(result.unwrap());
                        // });
                    }

                    println!("{:?}", now.elapsed())
                }

                Opcode::Ping => stream.send(Message::pong(m.into_data())).await?,

                Opcode::Close => {
                    break;
                }
                Opcode::Pong | Opcode::Binary => {}
            }
        }
    }

    Ok(())
}

fn handle_relay_message(_message: RelayMessage) {
    // println!("message received");
}
