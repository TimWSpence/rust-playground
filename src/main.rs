use rand::Rng;
use std::io;

mod bar;
mod show;
use crate::show::Show;

fn main() {
    println!("{}", "Show test".to_string().show());

    println!("{}", ["Array".to_string(), "show".to_string(), "test".to_string()].show());

    println!("Guess the number!");

    println!("Please input your guess.");

    let secret = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();

    let x = foo();

    let y = bar::bar::baz(1);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    println!("The secret number is: {}", secret);
}

fn foo() -> i32 {
    1
}
