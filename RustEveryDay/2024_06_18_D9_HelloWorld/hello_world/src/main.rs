#![allow(warnings)] // This ignore warning for the whole file
use std::any::type_name;
use regex::Regex;



fn print_type_of<T>(_: &T) {
    //kind of typeof()
    println!("{}", type_name::<T>());
}

fn main() {

    println!("Hello, world!");

    //https://blog.guillaume-gomez.fr/Rust/1/4

    #[allow(warnings)] // This ignore warning for this variable or the following function
    let a_variable_that_cannot_change = 10;

    #[allow(warnings)]
    let mut a_variable_that_can_change = 10;    
    a_variable_that_can_change += 20;

    #[allow(warnings)]
    let a_declared_variable_type: i32 = 10;
    let a_lazy_fucker_variable_type = 42u8; 
    // Look like 4f but less readable and more exact

    //// Those are the min max value
    println!("i16: min = {}, max = {}", i16::MIN, i16::MAX);
    println!("i32: min = {}, max = {}", i32::MIN, i32::MAX);
    println!("i64: min = {}, max = {}", i64::MIN, i64::MAX);
    println!("i128: min = {}, max = {}", i128::MIN, i128::MAX);
    println!("u8: min = {}, max = {}", u8::MIN, u8::MAX);
    println!("u16: min = {}, max = {}", u16::MIN, u16::MAX);
    println!("u32: min = {}, max = {}", u32::MIN, u32::MAX);
    println!("u64: min = {}, max = {}", u64::MIN, u64::MAX);
    println!("u128: min = {}, max = {}", u128::MIN, u128::MAX);
    println!("f32: min = {}, max = {}", f32::MIN, f32::MAX);
    println!("f64: min = {}, max = {}", f64::MIN, f64::MAX);


    print_type_of(&a_lazy_fucker_variable_type);

    let mut i: u32 = 42;
    i+=2;
    let mut a: bool=true;
    let mut b: bool=false;
    let mut c: bool=false;


    let mut iPointer: isize= 0;// singed integer pointer 32 or 64 bits depending of the platform
    let mut uPointer: usize= 0;// integer pointer 32 or 64 bits depending of the platform

    let mut _index_maybeUsed: i32=0; // _var means that compiler dont need to warn about it not beeing used.


    let integer_action: [i32; 6] = [0,1,1,2,0,4]; // un changeable sized array

    let mut integer_action_list: Vec<i32> = Vec::new();
    let mut _integer_action_list2: Vec<i32> = vec![0,1,1,2,0,4]; // changeable sized array
    integer_action_list.push(0i32);
    integer_action_list.push(1i32);

    //"{:?}" Format Specifier: This tells Rust to format the value using its Debug implementation.
    // The Debug trait is used to format a value in a programmer-friendly way, 
    //which is typically more verbose and includes additional information useful for debugging.
    println!("{:?}", integer_action_list);


    //"{:#?}" Format Specifier: This tells Rust to format the value using its Debug implementation,
    // but to use “pretty print” format. This means that the output will be nicely indented and
    #[derive(Debug)]
    struct Gauffre {
        x: i32,
        y: i32,
    }

    let gauffre: Gauffre = Gauffre { x: 1, y: 2 };
    println!("G {:?}", gauffre);


    let _first_action_v3: &[i32] = &_integer_action_list2[0..1]; // This is a slice
    let _first_action_array: &[i32] = &integer_action[0..1]; // This is a slice



    if i < 20 {/* This is a commment in a if condition */} 

    if i<20 {}
    else {}

    let true_false_condition =false;
    if true_false_condition {
        // No need to check if true
    }


    if i<20{}
    else if  i>=20 && i <40 {}
    else {}



    //kind of a "Switch" "pattern matching"
    //https://blog.guillaume-gomez.fr/Rust/1/5


        //swith(a){case 0: break;}

    let action_to_apply= "left";

    match action_to_apply {
        "left" => println!("left"),
        "right" => println!("right"),
        //pattern => expression,
        _ => println!("default"),
    }

    // _ equivalent of when "don't care" or "default"/"the rest" in this match case.


    let jump= false;
    match jump {
        true => println!("jump"),
        false => println!("no jump"),
        // _  is not mandatory because all case were used.
    }



    // cool part
    
    let integer_command =2099887766;
    println!("Drone CMD:{}", integer_command/100000000);
    match integer_command {
 
        c if c  /100000000 >0 =>  println!("Target id in game drone"),
        c if c  /100000000 == 0 =>  println!(" Default drone"), 
        c if c  /100000000 < 0 =>  println!("Target Owned drone"),
        
        _ => println!("Command not found"),
    }


    // Don't like but

    let mut index_value =542;
    match index_value {

        500..=600 => println!("Between 500 and 600"),
        // Not fan of it, I am missing some info to enjoy this one.
        // I suppose it is because we can use regex in the match :)
        /// OKKKK see example following
        over @ 500..=1000 => println!("Over 500 but less than 1000: {}", over),
        _ => println!("Not between 500 and 600"),
        
    }

    let mut action_command: i32=2;

    match action_command{
        0 => println!("Action 0"),
        (arrows @  (1 | 2 | 3)) => println!("Action Arrow {:?}",arrows),
        buttons @  4..=6 => println!("Action Button {:?}",buttons),
        _ => println!("Action not found"),
    }


    let text = "Hello, world!";
    let pattern: Regex = Regex::new(r"[Hh]ello").unwrap();

    match pattern.find(text) {
        Some(_) => println!("Pattern found"),
        None => println!("Pattern not found"),
    }

    match pattern.find(text) {
        found @ Some(_) => println!("Pattern found: {:?}", found),
        None => println!("Pattern not found"),
    }


    // Next épidode:https://blog.guillaume-gomez.fr/Rust/1/6

}
