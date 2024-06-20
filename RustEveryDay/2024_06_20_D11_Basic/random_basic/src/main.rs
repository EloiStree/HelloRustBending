
use rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("Random number: {}", return_random(1.0, 10.0));
    let (x,y) = return_random_v2(1.0, 10.0);
    println!("Random number: x: {}, y: {}", x, y);


    //https://blog.guillaume-gomez.fr/Rust/1/7

    let mut var = 0i32;
    let var2 = (var = 1i32);
    println!("var: {}", var);
    println!("var2: {:?}", var2);
    println!("test_expression: {}", test_expression(50));
   
    let mut i: i32 = 0;

    while i < 10 {
        
        let x:i32 = return_random(1.0, 20.0) as i32;
        println!("While: {}", test_expression(x));
        i += 1;
    }

    i=0;
    loop{

        let x:i32 = return_random(1.0, 20.0) as i32;
        println!("Loop: {}", test_expression(x));
        if i == 10 || x==1 {
            break;
        }
        i += 1;

    }
    
    for a in 2..5{
        println!("For: {}", a);
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("Vec For: {}", i);
    }

    for (position, valeur) in (6..10).enumerate() {
        println!("position = {} et valeur = {}", position, valeur);
    }
    let v = vec!["a", "b", "c", "d"]; // On crée un vecteur.

    for (position, value) in v.iter().enumerate() { // On itère sur ses valeurs.
        println!("position = {} et value = \"{}\"", position, value);
    }



    // SEEM COOL but holy shit ^^

    'global: for _ in 0..10 {
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                // on arrête la boucle qui s'appelle global
                if x > 3 { break 'global; }
    
                // on continue la boucle sur x
                if x % 2 == 0 { continue 'outer; }
    
                // on continue la boucle sur y
                if y % 2 == 0 { continue 'inner; }
    
                println!("x: {}, y: {}", x, y);
            }
        }
    }

  

    'X:for xx in 0..100 {
        'Y:for yy in 0..100 {
            'Z:for zz in 0..100 {
                println!("I: X{} Y{} Z{}", xx,yy,zz);
                if  xx+yy+zz%42==4 { break 'X;}
                if  xx+yy*zz%7==5 { break 'Y;}
                if  xx*yy+zz%9==2 { break 'Z;}
            }   
        }
    }
}


fn return_random( min : f32, max:f32)->f32{
    return rand::thread_rng().gen_range(min..max);
}
fn return_random_v2( min : f32, max:f32)->(f32,f32){
    return (rand::thread_rng().gen_range(min..max), rand::thread_rng().gen_range(min..max));
}


//Il reste cependant un dernier point à éclaircir : println! et tous les appels ayant un '!' ne sont pas des fonctions, ce sont des macros.


fn test_expression(x: i32) -> i32 {
    if x < 0 {
        println!("{} < 0", x);
        -1// THE FUCK ?
    } else if x == 0 {
        println!("{} == 0", x);
        0
    } else {
        println!("{} > 0", x);
        1
    }
}