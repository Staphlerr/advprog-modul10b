use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
        .connect()
        .await?;
    let (mut writer, mut reader) = ws_stream.split();

    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    loop {
        tokio::select! {
            // 1) Read a line from stdin and send it as a text message
            line = lines.next_line() => {
                if let Some(text) = line? {
                    writer.send(Message::text(text)).await?;
                } else {
                    break; // EOF
                }
            }

            // 2) Read a message from the WebSocket and print it if it’s text
            incoming = reader.next() => {
                match incoming {
                    Some(Ok(msg)) if msg.is_text() => {
                        if let Some(txt) = msg.as_text() {
                            println!("{}", txt);
                        }
                    }
                    Some(Ok(_)) => { /* ignore binary/ping/etc */ }
                    Some(Err(e)) => {
                        eprintln!("WS error: {}", e);
                        break;
                    }
                    None => break, // server closed
                }
            }
        }
    }

    Ok(())
}
