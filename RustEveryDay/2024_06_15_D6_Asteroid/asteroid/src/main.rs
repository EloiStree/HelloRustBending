use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    let mut asteroid_selected: AsteroidCreationEvent;
    let mut pool_item_index = 1;
    let mut asteroid_map: HashMap<i32, AsteroidCreationEvent> = HashMap::new();

    

    thread::spawn(move || {

        let socket = UdpSocket::bind("0.0.0.0:2571").expect("Failed to bind socket");
        let mut buf = [0; 1024];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    let data = &buf[..size];
                    // Process the received data
                    // TODO: Implement your logic here

                    if data.len() >10  {
                        if data[0]==20 {
                            let pool_id= data[1];
                            let pool_item_index = data[2] as i32;
                            println!("Received data from {}: Asteroid Item {}", addr, pool_item_index);
                            // let mut asteroid_selected = asteroid_map.get(&pool_item_index);
                            // if let Some(asteroid_selected) = asteroid_selected {
                            //     // Do something with asteroid_selected
                                
                            //     println!("Existing asteroid {}", pool_item_index);
                            // }

                            // else {
                            //     let asteroid = AsteroidCreationEvent {
                            //         m_pool_id: data[1],
                            //         m_pool_item_index: pool_item_index,
                            //         m_server_utc_now_ticks: 0,
                            //         m_start_position_x: 0,
                            //         m_start_position_y: 0,
                            //         m_start_position_z: 0,
                            //         m_start_rotation_euler_x: 0,
                            //         m_start_rotation_euler_y: 0,
                            //         m_start_rotation_euler_z: 0,
                            //         m_start_direction_x: 0,
                            //         m_start_direction_y: 0,
                            //         m_start_direction_z: 0,
                            //         m_speed_in_meters_per_second: 0.0,
                            //         m_collider_radius: 0.0,
                            //     };
                            //     asteroid_map.insert(pool_item_index, asteroid);
                            //     println!("Create asteroid");
                            //     println!("Received data from {}: Asteroid Item {}", addr, pool_item_index);
                            // }
                         

                        }
                    }


                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    break;
                }
            }
        }
    });

    let mut input = String::new();

    loop {
        print!("Please enter your name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input_display = input.trim();
        println!("Hello, {}!", input_display);
        input.clear();
    }
}



struct AsteroidCreationEvent {
    m_pool_id: u8,
    m_pool_item_index: i32,
    m_server_utc_now_ticks: i64,
    m_start_position_x: i32,
    m_start_position_y: i32,
    m_start_position_z: i32,
    m_start_rotation_euler_x: i32,
    m_start_rotation_euler_y: i32,
    m_start_rotation_euler_z: i32,
    m_start_direction_x: i32,
    m_start_direction_y: i32,
    m_start_direction_z: i32,
    m_speed_in_meters_per_second: f32,
    m_collider_radius: f32,
}