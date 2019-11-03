use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Pleas, input your guess");
    println!("the sicret number is {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    println!("You guessed:{}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You winn!")
    }

}
