mod json_schemas;

use warp::Filter;
use futures::{StreamExt, SinkExt};
use warp::ws::Message;
use crate::json_schemas::{parse};

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_socket)
        });

    warp::serve(ws_route)
        .run(([0, 0, 0, 0], 3000))
        .await;
}

async fn handle_socket(ws: warp::ws::WebSocket) {
    let (mut tx, mut rx) = ws.split();
    println!("WebSocket connection established");

    while let Some(Ok(msg)) = rx.next().await {
        if msg.is_text() {
            let req = match parse(msg.to_str().unwrap()) {
                Ok(ok) => {
                    ok
                }
                Err(err) => {
                    println!("Failed to deserialize request: {}", err);
                    tx.send(Message::close()).await.unwrap();
                    break;
                }
            };

            println!("{:?}", req);
            tx.send(Message::text("ok")).await.unwrap();
        }
        else if msg.is_close() {
            break;
        }
    }

    println!("WebSocket disconnected");
}
