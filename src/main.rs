use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  let (query, path) = parse_config(&args);

  println!("{} - {}", query, path);

  fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let path = &args[2];

    (query, path)
  }

  // let query = &args[1];
  // let filename = &args[2];

  // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

  // println!("With text:\n{}", contents);
}
