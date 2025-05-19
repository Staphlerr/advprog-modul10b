use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use tokio::sync::broadcast::error::RecvError;

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
        // 1) client → server
        incoming = ws_stream.next() => {
            match incoming {
                Some(Ok(msg)) => {
                    if let Some(text) = msg.as_text() {
                        let framed = format!("[{}] {}", addr, text);
                        let _ = bcast_tx.send(framed);
                    }
                }
                Some(Err(e)) => {
                    eprintln!("WS error from {}: {}", addr, e);
                    break;
                }
                None => break, // client disconnected
            }
        },

        // 2) server → client (broadcast channel)
        res = bcast_rx.recv() => {
            match res {
                Ok(msg) => {
                    // send it as a text frame
                    ws_stream.send(Message::text(msg)).await?;
                }
                Err(RecvError::Lagged(_)) => {
                    // we fell behind; just keep going
                }
                Err(RecvError::Closed) => {
                    // channel closed → no more broadcasts
                    break;
                }
            }
        }
    }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel::<String>(16);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr}");

        let tx = bcast_tx.clone();
        tokio::spawn(async move {
            match ServerBuilder::new().accept(socket).await {
                Ok((_req, ws_stream)) => {
                    if let Err(e) = handle_connection(addr, ws_stream, tx).await {
                        eprintln!("Connection {} error: {}", addr, e);
                    }
                }
                Err(upgrade_err) => {
                    eprintln!("WebSocket handshake with {} failed: {}", addr, upgrade_err);
                }
            }
        });
    }
}
