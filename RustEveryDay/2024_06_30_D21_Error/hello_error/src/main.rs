use std::{clone, fs::File};


pub struct Gun{

    pub munition_type: Option<MunitionType>,

}



pub enum MunitionTypeEnum{
    Bullet,
    Rocket,
    Grenade,
    Shell,
    Arrow,
    Dart,
    None,
}

impl MunitionTypeEnum{
    pub fn to_string(&self) -> String{
        match self {
            MunitionTypeEnum::Bullet => "Bullet".to_string(),
            MunitionTypeEnum::Rocket => "Rocket".to_string(),
            MunitionTypeEnum::Grenade => "Grenade".to_string(),
            MunitionTypeEnum::Shell => "Shell".to_string(),
            MunitionTypeEnum::Arrow => "Arrow".to_string(),
            MunitionTypeEnum::Dart => "Dart".to_string(),
            MunitionTypeEnum::None => "None".to_string(),
        }
    }
}



pub struct MunitionType{
  pub munition_type: MunitionTypeEnum,   

}

impl MunitionType{
    pub fn new() -> MunitionType{
        MunitionType{
            munition_type: MunitionTypeEnum::None,
        }
    }
}
impl Clone for MunitionTypeEnum{
    fn clone(&self) -> MunitionTypeEnum{
        match self {
            MunitionTypeEnum::Bullet => MunitionTypeEnum::Bullet,
            MunitionTypeEnum::Rocket => MunitionTypeEnum::Rocket,
            MunitionTypeEnum::Grenade => MunitionTypeEnum::Grenade,
            MunitionTypeEnum::Shell => MunitionTypeEnum::Shell,
            MunitionTypeEnum::Arrow => MunitionTypeEnum::Arrow,
            MunitionTypeEnum::Dart => MunitionTypeEnum::Dart,
            MunitionTypeEnum::None => MunitionTypeEnum::None,
        }
    }
}
impl Clone for MunitionType{
    fn clone(&self) -> MunitionType{
        MunitionType{
            munition_type: self.munition_type.clone(),
        }
    }
}
impl std::fmt::Debug for MunitionType{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "munition_type: {:?}", self.munition_type.to_string())
    }
}




fn main() {
    //https://blog.guillaume-gomez.fr/Rust/1/12
    println!("Hello, world!");

destruct();

    let mut gun = Gun{
        munition_type: None,
    };

    match gun.munition_type {
        Some(munition_type) => {
            println!("munition_type: {:?}", munition_type);
        },
        None => {
            println!("munition_type: None");
        }   
    }
    let muninition_enum = MunitionTypeEnum::Bullet;
    let munition_type = MunitionType{
        munition_type: muninition_enum,
    };


    gun = Gun{
        munition_type: Some(munition_type.clone()),
    };
    match gun.munition_type {
        Some(munition_type) => {
            println!("munition_type: {:?}", munition_type);
        },
        None => {
            println!("munition_type: None");
        }   
    }



    

    // Panic quit the app
    // panic!("I am panicking");

    
    gun.munition_type = None;


    


    // bad();
    // good();
    // good_but_not_the_best();




    println!("I am here");

}

fn bad() {
    // Dangereux à faire. Car ça fait crasher l'appllication;
    let mut fichier = File::open("fichier.txt").expect("erreur lors de l'ouverture");
}

fn good_but_not_the_best() {
    let mut fichier= File::open("fichier.txt").unwrap();
}

fn good() {
    let mut fichier = match File::open("fichier.txt") {
        Ok(fichier) => fichier,
        Err(erreur) => {
            panic!("Erreur lors de l'ouverture du fichier: {:?}", erreur)
        }
    };
}


fn note() {

    let mut v = vec!(1, 2, 3);

    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    // is equivalent to

    let mut v = vec!(1, 2, 3);

    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}


struct Point {
    x: i32,
    y: i32,
}


fn destruct(){

    let origin = Point { x: 1, y: 2 };

    match origin {
        Point { x,.. } => println!("({})", x),
    }
    // est équivalent à :
    if let Point { x, .. } = origin {
        println!("({})", x);
}

}
