use std::net::UdpSocket;

fn main() {
    println!("Hello, world!");

    let socket = UdpSocket::bind("127.0.0.1:1236").expect("Failed to bind socket");
    let mut buffer = [0; 1024];

    

    loop {
        let (size, _) = socket.recv_from(&mut buffer).expect("Failed to receive data");


        // Process the received data
        let data = &buffer[..size];
        println!("Received: {:?} from {:?}", data, socket.local_addr().unwrap());
        if data.len() == 4  {
            let num: i32 = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            println!("Received as i32: {}", num);
           
        }
    }


    
}
