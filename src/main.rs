mod utils;
mod scrapper;
mod structures;
mod pdfer;
use std::env::args;
use utils::utils::fix_title_to_path;
use scrapper::scrapper::{fetch_chapters, fetch_images, retrieve_body};
use pdfer::pdfer::{download_images, mesh_scraps};

fn download_manga_chapter(chapter_url: &String){
  let borrowed_url = chapter_url.to_string();
  let pages = fetch_images(retrieve_body(borrowed_url).unwrap());
  let downloaded_pages = download_images(pages);
  println!("Downloaded images");
  let _ = mesh_scraps(downloaded_pages, "teste".to_string()); // TODO create url to parsable file title
}


fn main() {
  let argc: Vec<String> = args().collect();
  // TODO add args size verification
  if *argc.get(1).unwrap() == "dc".to_string() {
    println!("recong the arg");
    download_manga_chapter(argc.get(2).unwrap());
  }
}
