#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpSocket,
};
use tokio::sync::{mpsc, Mutex};

#[derive(Debug)]
struct State {
    socket: Option<TcpSocket>,
    sink: Option<OwnedWriteHalf>,
    armageddon_output_tx: Option<tokio::sync::mpsc::Sender<Vec<u8>>>,
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tracing_subscriber::fmt().init();

    let socket = TcpSocket::new_v4().expect("unable to create socket");

    let (armageddon_output_tx, mut armageddon_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .manage(Mutex::new(State {
            socket: Some(socket),
            sink: None,
            armageddon_output_tx: Some(armageddon_output_tx),
        }))
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = armageddon_output_rx.recv().await {
                        process_stream_output(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            test,
            connect_to_armageddon,
            process_armageddon_input
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test(input: &str) {
    tracing::info!("Received command: {}", input);
}

#[tauri::command]
async fn connect_to_armageddon(state: tauri::State<'_, Mutex<State>>) -> Result<(), String> {
    let mut state = state.lock().await;

    if state.armageddon_output_tx.is_none() {
        return Ok(());
    }

    if let Some(socket) = state.socket.take() {
        let stream = socket
            .connect(
                "206.72.195.251:4050"
                    .parse()
                    .expect("unable to parse Ginka address"),
            )
            .await
            .unwrap();
        let (stream, sink) = stream.into_split();
        let stream = BufReader::new(stream);

        let _ = state.sink.insert(sink);

        if let Some(armageddon_output_tx) = state.armageddon_output_tx.take() {
            tauri::async_runtime::spawn(async move {
                process_stream(stream, armageddon_output_tx).await
            });
        }
    };

    Ok(())
}

#[tauri::command]
async fn process_armageddon_input(
    input: &str,
    state: tauri::State<'_, Mutex<State>>,
) -> Result<(), String> {
    tracing::info!("armageddon input: {:?}", input);

    let mut state = state.lock().await;

    if let Some(sink) = state.sink.as_mut() {
        // Armageddon requires carriage return at the end of input:
        let input = &[input.as_bytes(), b"\r\n"].concat()[..];

        sink.write_all(input).await.map_err(|e| {
            tracing::error!("write to sink got error: {}", e.to_string());
            e.to_string()
        })?;
    };

    Ok(())
}

fn process_stream_output<R: tauri::Runtime>(message: Vec<u8>, manager: &impl tauri::Manager<R>) {
    tracing::info!("armageddon output: {:?}", message);
    manager.emit_all("armageddon_output", message).unwrap();
}

async fn process_stream(
    mut stream_reader: BufReader<OwnedReadHalf>,
    output_tx: mpsc::Sender<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read current current data in the TcpStream
    while let Ok(received) = stream_reader.fill_buf().await {
        let received = received.to_vec();

        stream_reader.consume(received.len());
        output_tx.send(received).await?;
    }

    tracing::info!("process_stream received EOF");
    Ok(())
}
