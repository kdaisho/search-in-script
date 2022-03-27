use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args);
  Config::yell(&args[2]);

  println!("Searching for {}", config.query);
  println!("In file {}", config.path);

  let _contents = fs::read_to_string(config.path).expect("Something went wrong reading the file");
}

struct Config {
  query: String,
  path: String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    let query = args[1].clone();
    let path = args[2].clone();

    Config { query, path }
  }

  fn yell(path: &str) {
    println!("PATH IS {}!!!", path);
  }
}
