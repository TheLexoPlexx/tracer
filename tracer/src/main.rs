use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use poem::{
    EndpointExt, IntoResponse, Route, Server, get, handler,
    listener::TcpListener,
    web::{
        Data, Path,
        websocket::{Message, WebSocket},
    },
};
use serde_json::from_str;
use structure::WsRequest;
use tokio::spawn;
use util::WsResponse;

mod structure;
mod util;

#[handler]
fn websocket(
    Path(name): Path<String>,
    ws: WebSocket,
    sender: Data<&tokio::sync::mpsc::Sender<String>>,
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

        // Task: Nachrichten vom Server an den Client schicken
        // Hier müsstest du einen eigenen Channel für Antworten haben, z.B.:
        // while let Some(response) = ... {
        //     if sink.send(Message::Text(response)).await.is_err() {
        //         break;
        //     }
        // }
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Starting server...");

    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
    }
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or("4000".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());

    let channel = tokio::sync::mpsc::channel::<String>(32);
    let app = Route::new().at("/:name", get(websocket.data(channel.0)));

    Server::new(TcpListener::bind(format!("{host}:{port}")))
        .run(app)
        .await
}
