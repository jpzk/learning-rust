use std::error::Error;
use std::fs;

pub struct Config {
  query: String,
  filename: String
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }
      Ok(Config {
          query: args[1].clone(),
          filename: args[2].clone()
      })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{ 
  let contents = fs::read_to_string(config.filename)?;
  println!("With text:\n{}", contents);
  Ok(())
}