use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter your guess");

    let random_number = random_range(0..100);

    let mut guess = String::new();

    loop {
        guess.clear();
        let _ = io::stdin().read_line(&mut guess);
        let parsed_guess: i8 = match guess.trim().parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                println!("This is a number game noob");
                continue;
            }
        };

        match parsed_guess.cmp(&random_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
