
use std::env;
fn main() {

    let args: Vec<String> = env::args().collect();

    let my_int: i32 = args[1].parse().unwrap();

    let mut vec_nombre : Vec<i32> = Vec::new();
    let mut vec_nombre_premiers : Vec<i32> = Vec::new();
    

    for i in 2..my_int {
        vec_nombre.push(i);
        vec_nombre_premiers.push(i);
    }

    for toto in vec_nombre {
        vec_nombre_premiers.retain(|x| x.eq(&toto) || (x % &toto != 0));
    }

    println!("{:?}",vec_nombre_premiers);
    
}
