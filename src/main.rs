use std::fs;

use clap::{ Parser, Subcommand };

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  name: String,
  #[arg(short, long, default_value_t = 1)]
  count: u8,

  #[command(subcommand)]
  command: Command,
}


#[derive(Subcommand, Debug)]
enum Command {
    /// Doc comment
    Init,
}

fn main() {
  let args = Args::parse();

  match args.command {
    Command::Init => {
      fs::create_dir(".git").unwrap();
      fs::create_dir(".git/objects").unwrap();
      fs::create_dir(".git/refs").unwrap();
      fs::write(".git/HEAD", "ref: refs/head/main\n").unwrap();
      println!("Initialized git directory")
    }
  }
}
