use std::net::UdpSocket;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures::StreamExt;
use futures::sink::SinkExt;
use std::thread;
use tokio::runtime::Runtime;
use std::net;
use std::net::IpAddr;


use std::sync::Mutex;
use std::collections::VecDeque;
use lazy_static::lazy_static;

struct StaticQueue {
    queue: VecDeque<Vec<u8>>,
}

impl StaticQueue {
    fn new() -> Self {
        StaticQueue {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, data: Vec<u8>) {
        self.queue.push_back(data);
    }

    fn dequeue(&mut self) -> Option<Vec<u8>> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}

lazy_static! {
    static ref QUEUE: Mutex<StaticQueue> = Mutex::new(StaticQueue::new());
}

fn enqueue_data(data: Vec<u8>) {
    let mut queue = QUEUE.lock().unwrap();
    queue.enqueue(data);
}

fn dequeue_data() -> Option<Vec<u8>> {
    let mut queue = QUEUE.lock().unwrap();
    queue.dequeue()
}

fn is_queue_empty() -> bool {
    let queue = QUEUE.lock().unwrap();
    queue.is_empty()
}

fn queue_length() -> usize {
    let queue = QUEUE.lock().unwrap();
    queue.len()
}





async fn handle_connection(stream: tokio::net::TcpStream) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("New WebSocket connection: {:?}", ws_stream.get_ref().peer_addr().unwrap());

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        while let Some(Ok(message)) = ws_receiver.next().await {
            match message {
                Message::Text(txt) => {
                    println!("Received text message: {}", txt);
                    if ws_sender.send(Message::Text(txt)).await.is_err() {
                        println!("Error sending message");
                        break;
                    }
                }
                Message::Binary(bin) => {
                    println!("Received binary message");
                    if ws_sender.send(Message::Binary(bin)).await.is_err() {
                        println!("Error sending binary message");
                        break;
                    }
                }
                Message::Close(_) => {
                    println!("Connection closed");
                    break;
                }
                _ => {}
            }
        }
    } else {
        println!("Error during the WebSocket handshake");
    }
}

async fn run_server( ) {
    let addr = "0.0.0.0:1237";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server is running at {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
        println!("Test");
    }
}
fn main() {
    println!("Hello, world!");

    println!("Local UDP: 1236  , Server Port : 1237");
    
    let mut clients_list: Vec<tokio::net::TcpStream> = Vec::new();

    let server_thread = thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(run_server( )); 
    });

    // TO DO: Should learn how to kill the thread when app close.
   // let _ = server_thread.join();
    
    let udp_ipv4_and_port :String= "127.0.0.1:1236".to_string();
    println!("UDP Server is running at {}", udp_ipv4_and_port);
    let socket = UdpSocket::bind(udp_ipv4_and_port).expect("Failed to bind socket");
    let mut buffer = [0; 1024];
    loop {

        let (size, _) = socket.recv_from(&mut buffer).expect("Failed to receive data");
        // Process the received data
        let data = &buffer[..size];
        println!("Received: {:?} from {:?}", data, socket.local_addr().unwrap());
        if data.len() == 4 {
            let num: i32 = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            println!("Received as i32: {}", num);
            enqueue_data(data.to_vec());
        }
    }


}
