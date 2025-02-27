pub mod pdfer{
  use jpeg_to_pdf::JpegToPdf;
  use std::io::{BufWriter, Write};
  use std::fs::{File, read, exists, read_dir, DirEntry};
  use crate::utils::utils::{get_env, mk_tmp_scraps_folder, clear_scraps_folder, fix_title_to_path};
  use curl::easy::{List, Easy};
  use crate::structures::structures::ImagePage;
  
  
  fn generate_headers() -> List{
    let mut headers = List::new();
    let referer = get_env("REFERER_TO_HEADER").to_string();
    let headline = format!("Referer: {}", referer);
    headers.append(&headline).unwrap();
    return headers;
  }

  fn download_image(scrap_path: String, image_src: String, image_title: String) -> Result<String, Box<dyn std::error::Error>>{
    let parsed_title = fix_title_to_path(image_title);
    let path = scrap_path + "/" + &parsed_title + ".jpg";
    println!("Working with path {}", path);
    let _ = File::create(&path)?;
    let mut file = File::options().write(true).append(true).open(&path).unwrap();
    let headers = generate_headers();
    let mut handler = Easy::new();
    handler.url(&image_src).unwrap();
    handler.http_headers(headers).unwrap();
    handler.write_function(move |data| {
      if let Err(e) = file.write_all(data) {
        panic!("Error writing the file {}", &e);
      }
      Ok(data.len())
    }).unwrap();
    handler.perform()?;
    Ok(path)
  }

  pub fn download_images(images: Vec<ImagePage>) -> Vec<String>{
    let scraps_folder = mk_tmp_scraps_folder();
    let images_path = images.iter().map(|img| {
      download_image(scraps_folder.to_string(), img.src.to_string(), img.title.to_string()).unwrap()
    }).collect();
    return images_path;
  }

  pub fn mesh_scraps(scraps: Vec<String>, manga_chapter_title: &String) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!("./{}.pdf", manga_chapter_title);
    let _file = File::create(&path)?;
    let pdf = JpegToPdf::new();
    let n_pdf = pdf.add_images(scraps.iter().map(|scrap| {read(scrap).unwrap()}));
    if let Err(e) = n_pdf.create_pdf(&mut BufWriter::new(_file)) {
      panic!("Error writing the pdf {}", &e);
    }
    let _ = clear_scraps_folder();
    Ok(path)
  }

  /*
   Returns the ammount of pdf files inside the $pwd
   it works only to update the story itself
   you still have to provide the url to the mangas that will be added
   */
  pub fn list_pdfs() -> i32{
    let pwd_contents = read_dir(".").unwrap();
    let pdfs: Vec<Result<DirEntry, std::io::Error>> = pwd_contents.into_iter().filter(|direntry| {
      let name = direntry.as_ref().unwrap().file_name().to_str().unwrap().to_string();
      name.contains(".pdf")
    }).collect();
    return pdfs.iter().count().try_into().unwrap();
  }
}