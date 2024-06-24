use std::string;

fn main() {
    println!("Hello, Enum!");
    println!("Source: https://blog.guillaume-gomez.fr/Rust/1/9");
    move_avatar(Direction::Up);
    move_avatar(Direction::Down);
    move_avatar(Direction::Left);
    move_avatar(Direction::Right);


    let mut size = SizePlayer::Small;
    player_size(size);
     size = SizePlayer::Unkown;
    player_size(size);

    let ipv4= IPKind::IPV4(127, 0, 0, 1);
    let ipv6= IPKind::IPV6(0, 0, 0, 0);


    let action = Action::Bouger {
        piece: Piece::Pion,
        nouvelle_position: Position::LeftRightDownUp {
            left_right: 1,
            down_up: 2,
        }, 
    };

    DebugLogPrint::A.print("Hello");
    DebugLogPrint::B.print("Hello");


    Quaternion::Vector4(1.0, 2.0, 3.0, 4.0).to_string();
    Quaternion::Euler(1.0, 2.0, 3.0).to_string();
  let mut q =  Quaternion::Vector4(0.0,0.0,0.0,0.0);
  println!("Quaternion is valide {:?}", q.is_valide());
    q =  Quaternion::Vector4(1.0,2.0,3.0,4.0);
    println!("Quaternion is valide {:?}", q.is_valide());

}   

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right")
    }
}


enum SizePlayer {
    Small=150,
    Medium=175,
    Large=200,
    Unkown
}

fn player_size(size: SizePlayer) {
    match size {
        SizePlayer::Small => println!("Small {:?}",  (size as i32)),
        SizePlayer::Medium => println!("Medium {:?}", (size as i32)),
        SizePlayer::Large => println!("Large {:?}", (size as i32)),
        SizePlayer::Unkown => println!("Unkown")
    }
}

enum IPKind {
    IPV4(u8, u8, u8, u8),
    IPV6(u32, u32, u32, u32),
}

enum Piece {
    Pion,
    Tour,
    Cavalier,
    Fou,
    Reine,
    Roi,
}
enum Position {
    LeftRightDownUp{left_right:u8, down_up :u8},
    RightLeftDownUp{right_left:u8, down_up :u8},
    
}

enum Action {
    Bouger { piece: Piece, nouvelle_position: Position },
    Passer,
}


enum DebugLogPrint{
 A,
 B,
 C,
 D,

}

impl DebugLogPrint {
    pub fn print(&self, message: &str) {
        match self {
            DebugLogPrint::A => println!("A:{:?}", message),
            DebugLogPrint::B => println!("B:{:?}", message),
            DebugLogPrint::C => println!("C:{:?}", message),
            DebugLogPrint::D => println!("D:{:?}", message),
        }
    }
}

enum Quaternion{
    Vector4(f32, f32, f32, f32),
    Euler(f32, f32, f32),
}

impl Quaternion {
    pub fn to_string(&self) -> String {
        match self {
            Quaternion::Vector4(x, y, z, w) => format!("Vector4({}, {}, {}, {})", x, y, z, w),
            Quaternion::Euler(x, y, z) => format!("Euler({}, {}, {})", x, y, z),
        }
    }
    pub fn is_valide(&self) -> bool {
        
        match self {
            Quaternion::Vector4(x, y, z, w) => {
                if *x == 0.0 && *y == 0.0 && *z == 0.0 && *w == 0.0 {
                    return false;
                }
                return true;
            },
            Quaternion::Euler(_,_,_) => return true,
        }
    }
    
}