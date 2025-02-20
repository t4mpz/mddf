mod utils;
mod argscommands;
mod scrapper;
mod structures;
mod pdfer;
mod screens;
mod options;
use std::env::args;
use utils::utils::get_env;

fn gen_using_args() -> Vec<String> {
  if get_env("MODE") != "debug" { return args().collect(); }
  else{
    let mut internal_args: Vec<String> = Vec::new();
    internal_args.push("mddf".to_string()); // has to have this first arg so it corresponds to the system
    internal_args.push("q".to_string());
    internal_args.push(String::from("--no-add"));
    return internal_args;
  }
}

fn args_command_selector(argc: Vec<String>) {
  let dft_val = "".to_string();
  let main_arg = argc.get(1).unwrap_or(&dft_val);
  if *main_arg == "dc".to_string() {
    let chapters_to_download: i32 = match argc.contains(&"o".to_string()) {
      true => {
        let pos: i32 = argc.binary_search(&"o".to_string()).unwrap().try_into().unwrap_or(0) + 1;
        let upos: usize = pos.try_into().unwrap();
        let arg: &String = argc.get(upos).unwrap();
        arg.parse().unwrap_or(0)
      }
      false => 0
    };
    let download_options = options::download::gen_download_options(argc.get(2).unwrap(),
                                                                                               chapters_to_download, 
                                                                                               argc.contains(&"q".to_string()));
    argscommands::argscommands::download_using_options(download_options);
  }
  else if *main_arg == "q".to_string() {
    argscommands::argscommands::search_manga_chapter(&"yotsuba".to_string());
  }
}


fn main() {
  let argc: Vec<String> = gen_using_args();
  let mut result_options = options::search_results::gen_base_result_display_options();
  println!("Add info {}", result_options.additional_info);
  argc[2..].iter().for_each(|f| { println!("{}", f); });
  result_options = argscommands::argscommands::args_to_result_options(argc[2..].to_vec(), result_options);
  println!("Add info {}", result_options.additional_info);
  println!("href {}", result_options.href);
  // args_command_selector(argc);
  // let results = argscommands::argscommands::search_manga_chapter(&"yotsuba".to_string());
}
