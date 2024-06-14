use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

use rand::prelude::*;

trait IntegerCommandPusherIID {
    
    fn push_integer(&self, cmd: i32);
}
struct LocalGate {
    port: String
}

impl  IntegerCommandPusherIID for LocalGate {
    fn push_integer(&self, cmd: i32) {
        
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let message_bytes = cmd.to_le_bytes();
        let address= format!("127.0.0.1:{}", &self.port);
        socket
            .send_to(&message_bytes,address)
            .expect("Failed to send message");
    }
}





trait SetDroneInputAsInteger {
    fn set_drone_input_as_percent11(&mut self,  
        rotate_left_to_right: f32,
        move_down_to_up: f32,
        move_left_to_right:f32,
        move_back_to_forward:f32,);
    fn get_input_as_integer(&self) -> i32;
}

impl SetDroneInputAsInteger for DroneInputAsInteger {
     fn set_drone_input_as_percent11(&mut self,  
        rotate_left_to_right: f32,
        move_down_to_up: f32,
        move_left_to_right:f32,
        move_back_to_forward:f32,) {

            self.rotate_left_to_right = rotate_left_to_right;
            self.move_down_to_up = move_down_to_up;
            self.move_left_to_right = move_left_to_right;
            self.move_back_to_forward = move_back_to_forward;

            self.integer_input_as_i32 = 0;
            self.integer_input_as_i32+=DroneUtility::turn_percent_to_99(self.move_back_to_forward);
            self.integer_input_as_i32+=DroneUtility::turn_percent_to_99(self.move_left_to_right )*100;
            self.integer_input_as_i32+=DroneUtility::turn_percent_to_99(self.move_down_to_up )*10000;
            self.integer_input_as_i32+=DroneUtility::turn_percent_to_99(self.rotate_left_to_right) *1000000;


            if self.drone_id < 0{
                let math_abs = self.drone_id.abs();
                self.integer_input_as_i32+=(self.drone_id.abs() as i32)* 100000000;
                self.integer_input_as_i32*=-1;
            }
            else{

                self.integer_input_as_i32+=(self.drone_id as i32 )* 100000000;
            }
           
        }
    
     fn get_input_as_integer(&self) -> i32 {
        
        return self.integer_input_as_i32;
    }
}


struct DroneUtility;

impl DroneUtility {
    pub fn turn_percent_to_99(percent11:f32) -> i32 {
        if percent11 < -1.0 || percent11 > 1.0 {
            panic!("percent11 must be between -1.0 and 1.0");
        }
        if percent11 == 0.0 {
            return 0;
        }
        
        let t:f32 = (((percent11 +1.0)/2.0)*98.0)+1.0;
        return  t as i32;
    }
}

//-20 99 88 77 66

struct DroneInputAsInteger {
    drone_id: i8,
    rotate_left_to_right: f32,
    move_down_to_up: f32,
    move_left_to_right:f32,
    move_back_to_forward:f32,
    integer_input_as_i32: i32
}










fn main() {

    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let ms_between_commands: u64= 100;
    let drone_client: LocalGate =  LocalGate { port: String::from("2560") };
    let mut drone_input = DroneInputAsInteger {
        drone_id: -1,
        rotate_left_to_right: 0.0,
        move_down_to_up: 0.0,
        move_left_to_right: 0.0,
        move_back_to_forward: 0.0,
        integer_input_as_i32: 0
    };

    
    let mut previous_integer:i32 = drone_input.get_input_as_integer();
    let mut current_integer:i32 = drone_input.get_input_as_integer();

    loop{    
        drone_input.set_drone_input_as_percent11(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        current_integer = drone_input.get_input_as_integer();
        if current_integer != previous_integer {
            previous_integer = current_integer;
            println!("Input {}",current_integer.to_string());
            drone_client.push_integer(current_integer);
        }
        thread::sleep(Duration::from_millis(ms_between_commands));

    }
}


