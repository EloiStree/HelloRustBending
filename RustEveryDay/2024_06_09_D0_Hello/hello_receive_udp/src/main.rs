
use std::net::UdpSocket;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    println!("Start receiving random numbers from the client... port 7000");
    let socket = UdpSocket::bind("127.0.0.1:7000")?;
    loop {
        let mut buf = [0; 4]; // Buffer to hold the received bytes
        let (amt, src) = socket.recv_from(&mut buf)?;

        let number = i32::from_be_bytes(buf); // Convert the bytes back to an integer

        println!("Received: {} from {}", number, src);
    }
    Ok(())
}
