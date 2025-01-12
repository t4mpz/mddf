pub mod pdfer{
  use reqwest;
  use pdf_writer;
  use jpeg_to_pdf;
  use std::io::copy;
  use std::io::Cursor;
  use std::fs::File;
  use crate::utils::utils::get_env;
  use anyhow::Result;
  // use crate::structures::structures::ImagePage;


  pub fn download_image(image_src: String, image_title: String) -> Result<(), Box<dyn std::error::Error>>{
    let path = get_env("SCRAPS_FOLDER") + "/" + &image_title + ".jpg";
    let _ = File::create(&path);
    let mut file = File::options().write(true).append(true).open(&path).unwrap();
    let resp = reqwest::blocking::get(&image_src)?;
    let mut resp_body = Cursor::new(resp.bytes()?);
    copy(&mut resp_body, &mut file).expect("failed to write");
    Ok(())
  }


}