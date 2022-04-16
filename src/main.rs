use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for \"{}\"", config.query);
  println!("In file \"{}\"", config.path);

  if let Err(e) = run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  println!("Running for \"{}\"", config.query);
  let _contents = fs::read_to_string(config.path).expect("Something went wrong reading the file");

  // println!("With text:\n{}", _contents);

  Ok(())
}

struct Config {
  query: String,
  path: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() <= 2 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let path = format!("scripts/{}.txt", args[2].clone());

    Ok(Config { query, path })
  }
}
