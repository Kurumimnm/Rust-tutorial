use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // .gen_range(1..101) == 1..=100
    // println!("The secret number is : {}", secret_number);
    // let guess = 5;   不変 immutable
    // mutable→可変
    loop {
        println!("Please input your guess");

        let mut guess = String::new(); // ユーザの入力を格納

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {}", x, y);
}