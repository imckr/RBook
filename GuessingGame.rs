use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()  {

    let secret = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Enter the guess");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        println!("your guess : {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    }
    
}