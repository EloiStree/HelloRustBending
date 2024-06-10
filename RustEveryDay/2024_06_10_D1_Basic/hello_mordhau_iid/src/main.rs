use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

use rand::prelude::*;

trait IntegerCommandPusherIID {
    
    fn push_integer(&self, cmd: i32);
}


struct TargetIpPort {
    ip: String,
    port: String
}
struct TargetLocalPort {
    port: String
}

struct LocalGate {
    port: String
}
struct ServerOpenPort {
    ip: String,
    port: String
}

impl IntegerCommandPusherIID for UdpSocket {
    fn push_integer(&self, cmd: i32) {
        let message_bytes = cmd.to_le_bytes();
        self
            .send(&message_bytes)
            .expect("Failed to send message");
    }
}

impl  IntegerCommandPusherIID for TargetIpPort {
    fn push_integer(&self, cmd: i32) {

        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let message_bytes = cmd.to_le_bytes();
        let address= format!("{}:{}",&self.ip, &self.port);
        socket
            .send_to(&message_bytes,address)
            .expect("Failed to send message");
    }

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
impl  IntegerCommandPusherIID for TargetLocalPort {
    fn push_integer(&self, cmd: i32) {
        
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let message_bytes = cmd.to_le_bytes();
        let address= format!("127.0.0.1:{}", &self.port);
        socket
            .send_to(&message_bytes,address)
            .expect("Failed to send message");
    }

}

impl  IntegerCommandPusherIID for ServerOpenPort {
    fn push_integer(&self, cmd: i32) {
        
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let message_bytes = cmd.to_le_bytes();
        let address= format!("{}:{}",&self.ip, &self.port);
        socket
            .send_to(&message_bytes,address)
            .expect("Failed to send message");
        
    }

}


/***
 * 
 * 
 public struct A {
 
     pbulic float value;

 }

 public interface IntegerCommandPusherIID{
 
    public void pushInteger(int cmd);
 
 }
 public class APusher implements IntegerCommandPusherIID{
 
    private A a;
    public void pushInteger(int cmd){
        //send cmd to A
    }
 
 }

 public class UtilityA{
 
    public static void setValue(A a, float value){
        a.value = value;
    
    }
 }
 
 public Class A{
 
    public float value;

    public void setValue(float value){
        this.value = value;
    
    }
     
     }
 * 
 * 
 */





 fn push_integer(pusher: &impl IntegerCommandPusherIID , cmd: i32) {
    pusher.push_integer(cmd);
}



fn push_integer_local_gate(pusher: &Vec<LocalGate>, cmd: i32) {
    for item in pusher {
        push_integer(item, cmd);
    }
}
fn push_integer_server_gate(pusher: &Vec<ServerOpenPort>  , cmd: i32) {
    for item in pusher {
        push_integer(item, cmd);
    }
}



fn main() {


    let mut rng = rand::thread_rng();

    // OPTION A
    let local_gates: Vec<LocalGate> = vec![
        LocalGate { port: String::from("3614") },
        LocalGate { port: String::from("1234") }
    ];

    let server_open_ports: Vec<ServerOpenPort> = vec![
        ServerOpenPort { ip: String::from("81.240.94.97"), port: String::from("4513") }
    ];

  

    let ms_between_commands: u64= 100;
    println!("Hello, world!");

    
    let mut i:i32 = 0;

    loop{
        i+=1;
        if i>60 {
            i=0;
        }


        for local_gate in &local_gates {
            push_integer(local_gate, i);
        }
        for server_open_port in &server_open_ports {
            push_integer(server_open_port, i);
        }
        
        thread::sleep(Duration::from_millis(ms_between_commands));


        let value: i32 = rng.gen_range(-0..61);
        push_integer_local_gate(&local_gates, value);
        push_integer_server_gate(&server_open_ports, value);


        
        thread::sleep(Duration::from_millis(ms_between_commands));



    }
}


