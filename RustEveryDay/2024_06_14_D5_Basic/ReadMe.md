
Slice is nice :)

Allow to recovert part of an array.



Read HID from Rust.
``` rust

use hidapi::HidApi;
use std::thread;
use std::time::Duration;



fn main() {
    println!("Hello, Xbox!");

    // Initialize the HID API
    let api = HidApi::new().unwrap();

    // Search for Xbox controller based on vendor and product IDs
    let vendor_id = 0x045e; // Microsoft vendor ID for Xbox controllers
    let product_id = 0x028e; // Xbox 360 controller product ID 
    let mut device = match api.device_list().find(|d| d.vendor_id() == vendor_id && d.product_id() == product_id) {
        Some(device) => device,
        None => {
            eprintln!("Xbox controller not found.");
            return;
        }
    };

    // Open the Xbox controller device
    let mut device_handle = match device.open_device(&api) {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("Failed to open device: {}", e);
            return;
        }
    };

    // Continuously read input from the Xbox controller
    loop {
        let mut buffer = [0u8; 8];
        match device_handle.read(&mut buffer) {
            Ok(_) => {
                // Process the data from the buffer (you might want to parse and interpret this data)
                println!("Received data: {:?}", buffer);
                // Unfold the buffer to display the information
                let button_state = buffer[0];
                let left_thumb_x = buffer[1];
                let left_thumb_y = buffer[2];
                let right_thumb_x = buffer[3];
                let right_thumb_y = buffer[4];
                let left_trigger = buffer[5];
                let right_trigger = buffer[6];
                let dpad_state = buffer[7];

                println!("Button State: {}", button_state);
                println!("Left Thumb X: {}", left_thumb_x);
                println!("Left Thumb Y: {}", left_thumb_y);
                println!("Right Thumb X: {}", right_thumb_x);
                println!("Right Thumb Y: {}", right_thumb_y);
                println!("Left Trigger: {}", left_trigger);
                println!("Right Trigger: {}", right_trigger);
                println!("DPad State: {}", dpad_state);
                
                thread::sleep(Duration::from_millis(200));

            },
            Err(e) => {
                eprintln!("Error reading from device: {}", e);
                break; // Exit loop on error
            }
        }
    }
}

```
