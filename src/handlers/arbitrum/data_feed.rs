use futures::{SinkExt, StreamExt};
use websocket_lite::{ClientBuilder, Message, Opcode, Result};

use crate::env;

pub async fn init() -> Result<()> {
    let builder = ClientBuilder::from_url(env::RUNTIME_CONFIG.feed_endpoint.clone());
    let mut stream =  builder.async_connect().await?;

    while let Some(msg) = stream.next().await {
        if let Ok(m) = msg {
            match m.opcode() {
                Opcode::Text => {
                    println!("{}", m.as_text().unwrap());
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
