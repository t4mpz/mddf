pub mod argscommands {
  use crate::scrapper::scrapper;
  use crate::structures::structures::{ListedChapter, SearchResult};
  use crate::utils::utils;
  use crate::options::download::DownloadOptions;
  use crate::options::search_results::ResultDisplayOptions;
  use crate::pdfer::pdfer;

  fn download_manga_chapter(chapter_url: &String, chapter_title: &String){
    let borrowed_url = chapter_url.clone();
    println!("is downloading");
    let borrowed_title = utils::fix_title_to_path(chapter_title.clone());
    let pages = scrapper::fetch_images(scrapper::retrieve_body(borrowed_url).unwrap());
    let downloaded_pages = pdfer::download_images(pages);
    let _ = pdfer::mesh_scraps(downloaded_pages, borrowed_title);
  }
  
  pub fn search_manga_chapter(search_terms: &String) -> Vec<SearchResult> {
    let bd = scrapper::retrieve_body(
      format!("{}/{}", utils::get_env("DEFAULT_SEARCH_LINK"), utils::fix_title_to_path(search_terms.to_string()))
    ).unwrap();
    let results = scrapper::fetch_image_results(bd);
    return results;
  }

  pub fn download_story(story_url: &String) {
    let chapters = scrapper::fetch_chapters(scrapper::retrieve_body(story_url.to_string()).unwrap());
    chapters.iter().for_each(|cpt| {
      download_manga_chapter(&cpt.href, &cpt.title);
    });
  }

  pub fn download_using_options(options: DownloadOptions) {
    let chapters = scrapper::fetch_chapters(scrapper::retrieve_body(options.story_href.to_string()).unwrap());
    // TODO: There's a new DDOS guard, i must bypass it, I'll do this later, but the code is gud here, no bugs, just the guard
    // TODO: Just print the body so you can get a glance about the ddos guard
    chapters.into_iter().for_each(|chapter | {
      if !options.silent {
        println!("Downloading chapter {}...", chapter.title);
      }
      download_manga_chapter(&chapter.href, &chapter.title);
    });
  }

  /*
   Changes the display options accordinly with the args received at the 
   query command (ex. mddf search <name> --no-imgurl --no-add).
   */
  pub fn args_to_result_options(args: Vec<String>, mut result_options: ResultDisplayOptions) -> ResultDisplayOptions{
    let no_image_url: String = String::from("--no-imgurl");
    let no_href: String = String::from("--no-href");
    let no_name: String = String::from("--no-name");
    let no_numeric: String = String::from("--no-count");
    let no_additional: String = String::from("--no-add");
    let no_last_chapter: String = String::from("--no-last-chapter");

    args.iter().for_each(|a | {
      match a {
        _ if a.clone() == no_image_url => result_options.img_href = false,
        _ if a.clone() == no_name => result_options.story_name = false,
        _ if a.clone() == no_numeric => result_options.counting = false,
        _ if a.clone() == no_href => result_options.href = false,
        _ if a.clone() == no_additional => result_options.additional_info = false,
        _ if a.clone() == no_last_chapter => result_options.last_chapter = false,
        _ => panic!("Invalid argument option")
      }
    });
    return result_options;
  }
  
}