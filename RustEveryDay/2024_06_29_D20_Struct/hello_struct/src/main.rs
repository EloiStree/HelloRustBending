use unity::*;

mod unity;

struct UnityVector3Tuple(f32,f32,f32);
struct TagGoalHolder;
struct TagGaolFlag;


struct UdpSenderAuto {
    ip: String,
    port: u32,
    message_to_send: String,
}

impl UdpSenderAuto {
    fn new(ip: String, port: u32, message_to_send: String) -> UdpSenderAuto {
        let t =UdpSenderAuto {
            ip,
            port,
            message_to_send,
        };
        t.send();
        return t;
    }

    fn send(&self) {
        println!(
            "Sending message '{}' to {}:{}",
            self.message_to_send, self.ip, self.port
        );
    } 
}

//Destroyer
impl Drop for UdpSenderAuto {
    fn drop(&mut self) {
        println!("Stop existing Udp Sender Auto");
    }
}





fn main() {
    println!("Hello, world!");

    let mut v = UnityVector3 { x: 1.0, y: 2.0, z: 3.0 };
    let q = UnityQuaternion { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    let v3tuple=UnityVector3Tuple(1.0,2.0,3.0);
    let tag_goal_holder=TagGoalHolder;
    let tag_goal_flag=TagGaolFlag;


    v.set(1.0,2.0,3.0);


    //Déstructuration
    //https://blog.guillaume-gomez.fr/Rust/1/10
    // "Split" a sepcific value of a object 
    match  v {
       UnityVector3{  z, ..}  => {
            println!("v.z = {}", v.z)
        },
    }
    let UnityVector3{x,y,..}=v;
    println!("x = {}, y = {}", x, y);

    println!("Test Hello");
    {
        
        let udp_sender = UdpSenderAuto::new("127.0.0.1".to_string(), 1234, "Hello".to_string());
    }
    println!("Test Bye");

    
}
