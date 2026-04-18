use rand::random_range;
use std::io;

fn main() {
    println!("Enter your guess");

    let random_number = random_range(0..100);

    loop {
        let mut guess = String::new();
        let _ = io::stdin().read_line(&mut guess);
        let guess: i8 = match guess.trim().parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                println!("This is a number game noob");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too low"),
            std::cmp::Ordering::Greater => println!("Too High"),
            std::cmp::Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
