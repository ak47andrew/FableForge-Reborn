use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use common::{dump, parse, CSPacket, SCPacket};

pub async fn async_main(
    mut to_ws_rx: mpsc::Receiver<CSPacket>,
    from_ws_tx: mpsc::Sender<SCPacket>,
) {
    let ws = loop {
        match tokio_tungstenite::connect_async("ws://127.0.0.1:3000/ws").await {
            Ok((ws, _)) => break ws,
            Err(e) => {
                println!("Retrying WS connect: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    };
    println!("Connected");

    let (mut write, mut read) = ws.split();

    // write loop
    let write_task = tokio::spawn(async move {
        while let Some(msg) = to_ws_rx.recv().await {
            let _ = write.send(dump(msg).into()).await;
        }
    });

    // read loop
    let read_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            if let Ok(msg) = msg {
                let _ = from_ws_tx.send(parse(msg.to_string().as_str()).unwrap()).await;
            }
        }
    });

    let _ = tokio::join!(write_task, read_task);
}