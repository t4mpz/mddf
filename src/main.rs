mod utils;
mod scrapper;
mod structures;
mod pdfer;
use pdfer::pdfer::{download_image, mesh_scraps};

fn main() {
    // let body = scrapper::scrapper::retrieve_body("https://chapmanganato.to/manga-xe953687".to_string()).unwrap_or("No body found".to_string());
    // let mut items = scrapper::scrapper::fetch_chapters(body);
    // items.iter_mut().for_each(|item| println!("{}", item.href));
    let test_images: Vec<String> = [
        "https://v10.mkklcdnv6tempv4.com/img/tab_10/00/23/30/xe953687/vol_0_chapter_1_try_try_try_summer_vacation_1_40/1-o.jpg".to_string(),
        "https://v10.mkklcdnv6tempv4.com/img/tab_10/00/23/30/xe953687/vol_0_chapter_1_try_try_try_summer_vacation_1_40/2-o.jpg".to_string(),
        "https://v10.mkklcdnv6tempv4.com/img/tab_10/00/23/30/xe953687/vol_0_chapter_1_try_try_try_summer_vacation_1_40/3-o.jpg".to_string()
    ].to_vec();
    let scraps = test_images.iter().map(|src| {
        download_image(src.to_string(), "teste".to_string()).unwrap()
    }).collect();
    let _ = mesh_scraps("teste".to_string(), scraps, "yotsuba capitulos 1".to_string()).unwrap();
    println!("Might work");
}
