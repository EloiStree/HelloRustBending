use std::net::UdpSocket;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use std::thread;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

#[tokio::main]
async fn main() {

    println!("Hello, world!");
    println!("Push UDP bytes to 1236 port with broadcast to all websocket client on port 1237");

    // Shared list of WebSocket clients
    let clients: Arc<Mutex<Vec<UnboundedSender<Message>>>> = Arc::new(Mutex::new(Vec::new()));

    let udp_thread_clients: Arc<Mutex<Vec<UnboundedSender<Message>>>> = clients.clone();
    let udp_thread: thread::JoinHandle<_> = thread::spawn(move || {
        let udp_ipv4_and_port: String = "127.0.0.1:1236".to_string();
        println!("UDP Server is running at {}", udp_ipv4_and_port);
        let socket: UdpSocket = UdpSocket::bind(udp_ipv4_and_port).expect("Failed to bind socket");
        let mut buffer = [0; 1024];
        loop {
            let (size, _) = socket.recv_from(&mut buffer).expect("Failed to receive data");
            let data = &buffer[..size];
            println!("Received: {:?} from {:?}", data, socket.local_addr().unwrap());

            let clients = udp_thread_clients.lock().unwrap();
            for client in clients.iter() {
                if let Err(e) = client.send(Message::Binary(data.to_vec())) {
                    eprintln!("Error sending message to client: {:?}", e);
                }
            }
        }
    });

    let tcp_server_clients = clients.clone();
    let tcp_server_thread = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(run_server(tcp_server_clients));
    });

    udp_thread.join().unwrap();
    tcp_server_thread.join().unwrap();
}

async fn run_server(clients: Arc<Mutex<Vec<UnboundedSender<Message>>>>) {
    let addr = "0.0.0.0:1237";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("WebSocket server is running at {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, clients.clone()));
    }
}

async fn handle_connection(
    stream: TcpStream,
    clients: Arc<Mutex<Vec<UnboundedSender<Message>>>>,
) {
    if let Ok(mut ws_stream) = accept_async(stream).await {
        let (tx, mut rx) = unbounded_channel();
        {
            let mut clients = clients.lock().unwrap();
            clients.push(tx);
        }

        loop {
            tokio::select! {
                Some(message) = ws_stream.next() => match message {
                    Ok(message) => {
                        if message.is_close() {
                            break;
                        }
                    },
                    Err(_) => break,
                },
                Some(message) = rx.recv() => {
                    if ws_stream.send(message).await.is_err() {
                        break;
                    }
                }
            }
        }

        // Remove the client from the list when the connection is closed
        {
            let mut clients = clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    } else {
        println!("Error during the WebSocket handshake");
    }
}
