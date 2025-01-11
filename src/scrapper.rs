pub mod scrapper{
  use fantoccini::wd::Capabilities;
  use serde_json;
  use crate::structures::structures::{self, ImagePage, ListedChapter};
  use scraper::{html, Selector, ElementRef};

  #[tokio::main]
  pub async fn retrieve_body(url: String) -> Result<String, Box<dyn std::error::Error>>{
    let cap: Capabilities = serde_json::from_str(r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,).unwrap();
    let client = fantoccini::ClientBuilder::native()
    .capabilities(cap)
    .connect("http://localhost:38073").await.expect("No webdriver?");

    client.goto(&url).await?;
    let binding = client.current_url().await?;
    let client_url = binding.as_str();
    assert_eq!(url, client_url); 

    let body = client.source().await?;

    client.close().await?;

    Ok(body)
  }

  fn image_item_to_obj(item: ElementRef<'_>) -> structures::ImagePage{
    let src = item.attr("src").unwrap_or("no source found").to_string();
    assert!(src != "no source found", "one of the images has no source, how?");
    let title = item.attr("title").unwrap().to_string(); // this one can be empty
    let alt = item.attr("alt").unwrap().to_string();
    return ImagePage{src, title, alt}
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
    .map(|item| chapter_item_to_obj(item)).collect();
    return items;
  }

  pub fn fetch_images(body: String) -> Vec<structures::ImagePage>{
    let html_content = html::Html::parse_document(&body);
    let container_selector = Selector::parse("div.container-chapter-reader > img").unwrap();
    let chapters = html_content.select(&container_selector);
    let images = chapters.map(|chapter_image | image_item_to_obj(chapter_image)).collect();
    return images;
  }
}