use std::io;
fn main() {

    println!("Welcome to guessing game!");

    println!("Please input the guess number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail to read line");

    println!("your guess is {}", guess);
}
