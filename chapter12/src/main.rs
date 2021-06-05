use std::process;
use chapter12::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = chapter12::run(config) { 
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// whats the second type parameter here? Box<dyn Error>
/*
For the error type, we used the trait object Box<dyn Error> (and we’ve 
    brought std::error::Error into scope with a use statement at the top). 
    We’ll cover trait objects in Chapter 17. 
*/

