
use rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("Random number: {}", return_random(1.0, 10.0));
    let (x,y) = return_random_v2(1.0, 10.0);
    println!("Random number: x: {}, y: {}", x, y);


    //https://blog.guillaume-gomez.fr/Rust/1/7
    

}


fn return_random( min : f32, max:f32)->f32{
    return rand::thread_rng().gen_range(min..max);
}
fn return_random_v2( min : f32, max:f32)->(f32,f32){
    return (rand::thread_rng().gen_range(min..max), rand::thread_rng().gen_range(min..max));
}


//Il reste cependant un dernier point à éclaircir : println! et tous les appels ayant un '!' ne sont pas des fonctions, ce sont des macros.