mod utils;
mod argscommands;
mod scrapper;
mod structures;
mod pdfer;
mod screens;
use std::env::args;
use utils::utils::get_env;

fn gen_using_args() -> Vec<String> {
  if get_env("MODE") != "debug" { return args().collect(); }
  else{
    let mut internal_args: Vec<String> = Vec::new();
    internal_args.push("mddf".to_string()); // has to have this first arg so it corresponds to the system
    internal_args.push("q".to_string());
    return internal_args;
  }
}

fn args_command_selector(argc: Vec<String>) {
  let dft_val = "".to_string();
  let main_arg = argc.get(1).unwrap_or(&dft_val);
  if *main_arg == "dc".to_string() {
    println!("recong the arg");
    argscommands::argscommands::download_manga_chapter(argc.get(2).unwrap(), argc.get(3).unwrap());
  }
  else if *main_arg == "q".to_string() {
    argscommands::argscommands::search_manga_chapter(&"yotsuba".to_string());
  }
}


fn main() {
  let argc: Vec<String> = gen_using_args();
  // TODO add args size verification
  // args_command_selector(argc);
}
