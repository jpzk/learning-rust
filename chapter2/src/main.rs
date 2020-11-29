use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
This project was a hands-on way to introduce you to many new Rust concepts: 
let, match, methods, associated functions, the use of external crates, and more.
In the next few chapters, you’ll learn about these concepts in more detail. 
Chapter 3 covers concepts that most programming languages have, such as 
variables, data types, and functions, and shows how to use them in Rust. 
Chapter 4 explores ownership, a feature that makes Rust different from other 
languages. Chapter 5 discusses structs and method syntax, and Chapter 6 explains
how enums work. 
*/

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        println!("You guess: {}", guess);

        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;    
            }
        }
    }
}
