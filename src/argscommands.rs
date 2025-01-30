pub mod argscommands {
  use crate::scrapper::scrapper;
  use crate::structures::structures::SearchResult;
use crate::utils::utils;
  use crate::pdfer::pdfer;

  pub fn download_manga_chapter(chapter_url: &String, chapter_title: &String){
    let borrowed_url = chapter_url.to_string();
    let borrowed_title = utils::fix_title_to_path(chapter_title.to_string());
    let pages = scrapper::fetch_images(scrapper::retrieve_body(borrowed_url).unwrap());
    let downloaded_pages = pdfer::download_images(pages);
    let _ = pdfer::mesh_scraps(downloaded_pages, borrowed_title); // TODO create url to parsable file title
  }
  
  pub fn search_manga_chapter(search_terms: &String) -> Vec<SearchResult> {
    let bd = scrapper::retrieve_body(
      format!("{}/{}", utils::get_env("DEFAULT_SEARCH_LINK"), utils::fix_title_to_path(search_terms.to_string()))
    ).unwrap();
    let results = scrapper::fetch_image_results(bd);
    // results.iter().for_each(|f | println!("{}", f.img_href));
    return results;
  }

  pub fn download_story(story_url: &String) {
    let chapters = scrapper::fetch_chapters(scrapper::retrieve_body(story_url.to_string()).unwrap());
    chapters.iter().for_each(|cpt| {
      download_manga_chapter(&cpt.href, &cpt.title);
    });
  }
  
}