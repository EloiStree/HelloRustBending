use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    println!("Start sending random numbers to the server... port 7000");

    // Bind the UDP socket to an available port on the local machine
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind socket");
    let mut rng = rand::thread_rng();

    loop {
        // Generate a random number between 1 and 100
        let value: i32 = rng.gen_range(-100..101);
        let bytes = value.to_be_bytes();

        // Send the random number to the specified address and port
        socket.send_to(&bytes, "127.0.0.1:7000").expect("Failed to send data");

        // Sleep for 1 second before sending the next number
        println!("Sent data: {}", value);
        thread::sleep(Duration::from_secs(1));
    }
}
