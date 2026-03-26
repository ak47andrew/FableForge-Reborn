use warp::Filter;
use futures::{StreamExt, SinkExt};
use warp::ws::Message;

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
            let content = msg.to_str().unwrap();
            let parts = content.split(" ").collect::<Vec<&str>>();
            let command = *parts.first().unwrap_or(&"LOSER");
            let other = parts[1..].to_vec();
            match command {
                "TOKEN" => {
                    // TODO: finish
                }
                _ => {
                    tx.send(Message::text("Fucking moron toying with protocol. Go fuck yourself")).await.unwrap();
                    tx.send(Message::close()).await.unwrap();
                    break;
                }
            }
            // println!("Got {}", msg.to_str().unwrap());
            // tx.send(msg).await.unwrap(); // echo
        }
        else if msg.is_close() {
            break;
        }
    }

    println!("WebSocket disconnected");
}