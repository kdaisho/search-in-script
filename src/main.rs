use std::env;
use std::process;

// #[allow(unused_imports)]
use search_in_script::Config;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for \"{}\"", config.query);
  println!("In file \"{}\"", config.path);

  if let Err(e) = search_in_script::run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}
