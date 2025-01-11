mod utils;
mod scrapper;
mod structures;

fn main() {
    std::env::args().for_each(|arg| println!("{}", arg));
    let body = scrapper::scrapper::retrieve_body("https://chapmanganato.to/manga-xe953687".to_string()).unwrap_or("No body found".to_string());
    let mut items = scrapper::scrapper::fetch_chapters(body);
    items.iter_mut().for_each(|item| println!("{}", item.href));
}
