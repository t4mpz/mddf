mod utils;
mod scrapper;
mod structures;
mod pdfer;
use std::env::args;
use utils::utils::{fix_title_to_path, get_env};
use pdfer::pdfer::{download_images, mesh_scraps};
use scrapper::scrapper::{fetch_chapters, fetch_images, retrieve_body, fetch_image_results};

fn gen_using_args() -> Vec<String> {
  if get_env("MODE") != "debug" { return args().collect(); }
  else{
    let mut internal_args: Vec<String> = Vec::new();
    internal_args.push("mddf".to_string()); // has to have this first arg so it corresponds to the system
    internal_args.push("q".to_string());
    return internal_args;
  }
}

fn download_manga_chapter(chapter_url: &String){
  let borrowed_url = chapter_url.to_string();
  let pages = fetch_images(retrieve_body(borrowed_url).unwrap());
  let downloaded_pages = download_images(pages);
  println!("Downloaded images");
  let _ = mesh_scraps(downloaded_pages, "teste".to_string()); // TODO create url to parsable file title
}

fn search_manga_chapter(search_terms: &String) {
  let bd = retrieve_body(
    format!("{}/{}", get_env("DEFAULT_SEARCH_LINK"), fix_title_to_path(search_terms.to_string()))
  ).unwrap();
  let results = fetch_image_results(bd);
  results.iter().for_each(|f | println!("{}", f.img_href));
}

fn main() {
  let argc: Vec<String> = gen_using_args();
  let dft_val = "".to_string();
  // TODO add args size verification
  let main_arg = argc.get(1).unwrap_or(&dft_val);
  if *main_arg == "dc".to_string() {
    println!("recong the arg");
    download_manga_chapter(argc.get(2).unwrap());
  }
  else if *main_arg == "q".to_string() {
    search_manga_chapter(&"yotsuba".to_string());
  }
}
