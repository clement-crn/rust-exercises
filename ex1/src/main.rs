use std::io;
use std::io::*;
extern crate read_input;
use read_input::prelude::*;

fn main() {
    println!("\n \nBienvenue dans le Wojak Quizz ! \n");
    println!("Quelle est la bonne rep ? 1 ou 2 ?\n\n");

    let mut correct = String::new();
    correct.push_str("1");

    let input: String = input_new().msg("Please input your name: ").get();

    if input == correct {
        println!("bravo !");
    } else {
        println!("non");
    }
}
