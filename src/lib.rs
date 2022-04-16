use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub path: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() <= 2 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let path = format!("scripts/{}.txt", args[2].clone());

    Ok(Config { query, path })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  println!("Running for \"{}\"", config.query);
  let _contents = fs::read_to_string(config.path).expect("Something went wrong reading the file");
  // println!("With text:\n{}", _contents);

  Ok(())
}
