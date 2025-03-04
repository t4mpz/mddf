use scraper::{Html, Selector};

/*
 Checks if the received body has a DDOS content mark or meta
 this logic makes the fantoccini client wait for the full contents
 if the body is in the current DDOS prevention screen
 it has to be better polished, because the DDOS screen have more 
 important details then the actual here
 */
pub fn check_body(body: &String) -> bool {
  let html = Html::parse_document(body);
  let meta_selector = Selector::parse("meta").unwrap();
  let mut meta_tags = html.select(&meta_selector);
  meta_tags.any(| meta  | {
    meta.attr("ddos_prevention").unwrap_or("") == "true"
  })
}