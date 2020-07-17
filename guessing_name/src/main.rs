use std::cmp::Ordering;
use std::io;
mod hola;
use hola::print_hola;
// use rand::thread_rng;
fn main() {
    println!("Guess the number!");
    // let secret_number = thread_rng().gen_range(1, 101);
    println!("Please input your guess");
    print_hola();

}

fn five() -> i32 {
    if true {
        3
    }else{
        2
    }
}

