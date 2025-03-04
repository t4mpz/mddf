pub mod scrapper{
  use fantoccini::wd::Capabilities;
  use serde_json;
  use crate::structures::structures::{self, ImagePage, ListedChapter, SearchResult};
  use scraper::{html, Selector, ElementRef};
  use std::process::{Command, Child};

  pub async fn start_chromedriver() -> Child{
    let output = Command::new("chromedriver")
                                .arg("--port=38073")
                                .arg("--silent")
                                .spawn()
                                .expect("Chromedriver failed to start");
    return output;
  }

  pub fn hardkill_chromedriver() -> Result<(), Box<dyn std::error::Error>> { 
    // used when some part of the program went wrong and have to dump the whole process
    // it will run a killall chromedriver
    let _ = Command::new("killall").arg("chromedriver").spawn();
    Ok(())
  }

  #[tokio::main]
  pub async fn retrieve_body(url: String) -> Result<String, Box<dyn std::error::Error>>{
    let mut child_driver = start_chromedriver().await;
    let cap: Capabilities = serde_json::from_str(r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,).unwrap();
    let client = fantoccini::ClientBuilder::native()
    .capabilities(cap)
    .connect("http://localhost:38073").await.expect("No webdriver? ");
    if let Err(e) =  client.goto(&url).await {
      panic!("Couldnt connect with the mangakalot href: {}", e);
    }
    // should be checking for the DDOS page here
    // should also check for the network response huh genious

    let binding = client.current_url().await?;
    let client_url = binding.as_str();
    assert_eq!(url, client_url); 

    let body = client.source().await?;

    client.close().await?;
    child_driver.kill()?;

    Ok(body)
  }

  fn image_item_to_obj(item: ElementRef<'_>) -> structures::ImagePage{
    let src = item.attr("src").unwrap_or("no source found").to_string();
    assert!(src != "no source found", "one of the images has no source, how?");
    let title = item.attr("title").unwrap().to_string(); // this one can be empty
    let alt = item.attr("alt").unwrap().to_string();
    return ImagePage{src, title, alt}
  }

  fn panel_story_to_result(panel_story_item: ElementRef<'_>) -> structures::SearchResult {
    let item_children: Vec<ElementRef<'_>> = panel_story_item.child_elements().collect();
    let story_name_a_vector: Vec<ElementRef<'_>> = panel_story_item.select(
      &Selector::parse("h3.story_name>a"
    ).unwrap()).collect();
    let last_chapter_a_vector: Vec<ElementRef<'_>> = panel_story_item.select(
      &Selector::parse("em.story_chapter>a"
    ).unwrap()).collect();
    let last_chapter: String = last_chapter_a_vector.into_iter()
                                                    .map(|a| { a.inner_html() })
                                                    .collect::<String>();
    let add_info_vector: Vec<ElementRef<'_>> = panel_story_item.select(
      &Selector::parse("span")
      .unwrap()).collect();
    let additional_info: Vec<String> = add_info_vector.into_iter()
                                                    .map(|a| { a.inner_html() }).collect();
    let href = item_children[0].attr("href").unwrap_or("no href found").to_string();
    let story_name = story_name_a_vector[0].inner_html();
    let a_children: Vec<ElementRef<'_>> = item_children[0].child_elements().collect();
    let img_href: String = a_children[0].attr("src").unwrap_or("no img source found").to_string();

    return SearchResult {
      href,
      img_href,
      story_name,
      last_chapter,
      additional_info
    }
  }

  fn chapter_item_to_obj(item: ElementRef<'_>) -> ListedChapter{
    let a_href_selector = Selector::parse("a").unwrap();
    let date_selector = Selector::parse("span.chapter-time").unwrap();
    let a_href = item.select(&a_href_selector)
                              .map(|a | a.attr("href").unwrap_or("No Link found"))
                              .collect::<String>();
    let chapter_title = item.select(&a_href_selector)
                                    .map(|a | a.attr("title").unwrap_or("No title found"))
                                    .collect::<String>();
    let release_date = item.select(&date_selector)
                                    .map(|span| span.text().collect::<String>())
                                    .collect::<String>();
    return structures::ListedChapter{ href: a_href, title: chapter_title, position: 0, updated_raw_date: release_date};
  }

  pub fn fetch_chapters(body: String) -> Vec<ListedChapter>{
    let html_content = html::Html::parse_document(&body);
    let unlisted_chapters_items_selector = Selector::parse("li").unwrap();
    let items = html_content.select(&unlisted_chapters_items_selector)
                                                .map(|item| chapter_item_to_obj(item))
                                                .collect();
    return items;
  }

  pub fn fetch_images(body: String) -> Vec<structures::ImagePage>{
    let html_content = html::Html::parse_document(&body);
    let container_selector = Selector::parse("div.container-chapter-reader > img").unwrap();
    let chapters = html_content.select(&container_selector);
    let images = chapters.map(|chapter_image | image_item_to_obj(chapter_image)).collect();
    return images;
  }

  pub fn fetch_image_results(body: String) -> Vec<structures::SearchResult>{
    let html_content = html::Html::parse_document(&body);
    let container_selector = Selector::parse("div.story_item").unwrap();
    let html_results = html_content.select(&container_selector);
    return html_results.map(panel_story_to_result).collect();
  }
  
}