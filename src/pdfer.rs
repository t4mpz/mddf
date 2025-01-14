pub mod pdfer{
  use pdf_writer;
  use jpeg_to_pdf::JpegToPdf;
  use std::io::{BufWriter, Write};
  use std::fs::{File, read};
  use crate::utils::utils::get_env;
  use curl::easy::{List, Easy};
  
  fn generate_headers() -> List{
    let mut headers = List::new();
    let referer = get_env("REFERER_TO_HEADER").to_string();
    let headline = format!("Referer: {}", referer);
    headers.append(&headline).unwrap();
    return headers;
  }

  pub fn download_image(image_src: String, image_title: String) -> Result<String, Box<dyn std::error::Error>>{
    let path = get_env("SCRAPS_FOLDER") + "/" + &image_title + ".jpg";
    let _ = File::create(&path)?;
    let mut file = File::options().write(true).append(true).open(&path).unwrap();
    let headers = generate_headers();
    let mut handler = Easy::new();
    handler.url(&image_src).unwrap();
    handler.http_headers(headers).unwrap();
    handler.write_function(move |data| {
      // let mut resp_body = Cursor::new(resp.bytes()?);
      if let Err(e) = file.write_all(data) {
        panic!("Error writing the file {}", &e);
      }
      Ok(data.len())
    }).unwrap();
    handler.perform()?;
    Ok(path)
  }

  pub fn mesh_scraps(final_pdf: String, scraps: Vec<String>, manga_chapter_title: String) -> Result<String, Box<dyn std::error::Error>> {
    let path = get_env("CHAPTERS_FOLDER") + "/" + &manga_chapter_title + ".pdf";
    let _file = File::create(&path)?;
    let pdf = JpegToPdf::new();
    let n_pdf = pdf.add_images(scraps.iter().map(|scrap| {read(scrap).unwrap()}));
    if let Err(e) = n_pdf.create_pdf(&mut BufWriter::new(_file)) {
      panic!("Error writing the pdf {}", &e);
    }
    Ok(path)
  }


}