use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hey 👋! Welcome to the game! Enter the value!");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Less than random number!"),
            Ordering::Greater => println!("Greater than random number!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
