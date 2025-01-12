mod utils;
mod scrapper;
mod structures;
mod pdfer;
use pdfer::pdfer::download_image;

fn main() {
    // let body = scrapper::scrapper::retrieve_body("https://chapmanganato.to/manga-xe953687".to_string()).unwrap_or("No body found".to_string());
    // let mut items = scrapper::scrapper::fetch_chapters(body);
    // items.iter_mut().for_each(|item| println!("{}", item.href));
    let _ = download_image("https://v10.mkklcdnv6tempv4.com/img/tab_10/00/23/30/xe953687/vol_0_chapter_1_try_try_try_summer_vacation_1_40/1-o.jpg".to_string(), "testing".to_string()).unwrap();
}
