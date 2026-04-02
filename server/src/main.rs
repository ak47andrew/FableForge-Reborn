mod state;

use std::collections::HashMap;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::Mutex;
use warp::Filter;
use futures::{StreamExt, SinkExt};
use warp::ws::Message;
use common::{dump, parse, CSPacket, SCPacket};
use crate::state::{with_state, GameState, SharedState, NEXT_CLIENT_ID};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(GameState {
        clients: HashMap::new(),
        tokens: HashMap::new(),
    }));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_state(state))
        .map(|ws: warp::ws::Ws, state| {
            ws.on_upgrade(move |socket| handle_socket(socket, state))
        });

    warp::serve(ws_route)
        .run(([0, 0, 0, 0], 3000))
        .await;
}

async fn handle_socket(ws: warp::ws::WebSocket, state: SharedState) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let index = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    tokio::task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });
    println!("WebSocket connection established");

    {
        let mut state = state.lock().await;
        state.clients.insert(index, tx.clone());

        let sender = tx.clone();
        for value in state.tokens.values() {
            sender.send(Message::text(dump(
                CSPacket::AddToken {token: *value}
            ))).unwrap();
        }
    }

    while let Some(Ok(msg)) = ws_rx.next().await {
        if msg.is_text() {
            let resp = match parse::<CSPacket>(msg.to_str().unwrap()) {
                Ok(packet) => {
                    println!("Received: {:?}", packet);
                    {
                        let mut state = state.lock().await;

                        match packet {
                            CSPacket::MoveToken { token } => {
                                state.tokens.insert(token.id, token);
                                let packet = SCPacket::MoveToken {
                                    token: state.tokens[&token.id]
                                };
                                for (_, client) in state.clients.iter() {
                                    client.send(Message::text(dump(packet.clone()))).unwrap();
                                }

                                SCPacket::Ok
                            },
                            CSPacket::AddToken { token } => {
                                state.tokens.insert(token.id, token);

                                let packet = SCPacket::AddToken {
                                    token: state.tokens[&token.id]
                                };
                                for (_, client) in state.clients.iter() {
                                    client.send(Message::text(dump(packet.clone()))).unwrap();
                                }

                                SCPacket::Ok
                            },
                            CSPacket::DeleteToken { token } => {
                                state.tokens.remove(&token);

                                let packet = SCPacket::DeleteToken {
                                    token
                                };
                                for (_, client) in state.clients.iter() {
                                    client.send(Message::text(dump(packet.clone()))).unwrap();
                                }

                                SCPacket::Ok
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to deserialize request: {}", err);
                    tx.send(Message::close()).unwrap();
                    break;
                }
            };

            tx.send(Message::text(dump(resp))).unwrap();
        }
        else if msg.is_close() {
            break;
        }
    }

    {
        let mut game = state.lock().await;
        game.clients.remove(&index);
    }

    println!("WebSocket disconnected");
}
