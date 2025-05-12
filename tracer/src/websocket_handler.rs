use futures_util::{SinkExt, StreamExt};
use poem::web::websocket::{Message, WebSocket};
use poem::web::{Data, Path};
use poem::{IntoResponse, handler};
use serde::Deserialize;
use serde_json::from_str;
use tokio::spawn;
use tokio::sync::mpsc::Sender;

use crate::util::WsResponse;

#[derive(Debug, Deserialize)]
struct WsRequest {
    _name: String,
    _message: String,
}

#[handler]
pub fn websocket(
    Path(name): Path<String>,
    ws: WebSocket,
    sender: Data<&Sender<String>>,
) -> impl IntoResponse {
    let sender = sender.clone();

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        println!("WebSocket connection established for: {name}");

        let response = WsResponse::new("connected".to_string());

        sink.send(Message::Text(serde_json::to_string(&response).unwrap()))
            .await
            .expect("Failed to send message to channel");

        // Task: Nachrichten vom Client empfangen und an den Server schicken
        let sender_clone = sender.clone();
        spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(text) = msg {
                    match from_str::<WsRequest>(&text) {
                        Ok(json) => {
                            println!("Received message in JSON: {:#?} from {name}", json);
                            sink.send(Message::Text(format!("{name}: {text}")))
                                .await
                                .unwrap();
                        }
                        Err(_) => {
                            println!("Received message in plain text: {text} from {name}");
                        }
                    }

                    sink.send(Message::Text(format!("{name}: {text}")))
                        .await
                        .unwrap();
                    if sender_clone.send(format!("{name}: {text}")).await.is_err() {
                        println!("Failed to send message to channel");
                        break;
                    }
                }
            }
        });
    })
}
