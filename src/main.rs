use std::{env::args, fs};

fn main() {
  let args: Vec<String> = args().collect();
  if args[1] == "init" {
    fs::create_dir(".git").unwrap();
    fs::create_dir("./git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write("./git/HEAD", "refs/head/main\n").unwrap();
    println!("Initialized git directory");
  } else {
    println!("Unknow command: {}", args[1])
  }
}