use cgmath::prelude::*;

use std::io;

fn main() {
    println!("Guess the number! ♥");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

/*
fn main() {
    let vec = [1,2,3,4];
    for i in vec
    println!("Hello, world!");
}
*/