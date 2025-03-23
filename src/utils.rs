pub mod utils{
  use std::fs;
  use dotenvy;
  use std::env::current_dir;
  use regex::Regex;
  use image::ImageReader;
  use image::ImageFormat::Jpeg;

  pub fn fix_title_to_path(title: String) -> String {
    let rg = Regex::new(r"\W").unwrap();
    return Regex::replace_all(&rg, &title, "_").to_string();
  }

  pub fn input_to_search(input: String) -> String {
    // will reuse the fix title to path because why not
    let lower = input.to_lowercase();
    return fix_title_to_path(lower);
  }
  
  pub fn write_base_env(file_name: String) {
    let contents = "SCRAPS_FOLDER=scraps\nDEFAULT_FOLDER=downloads";
    let _ = fs::write(file_name, contents);
  }

  pub fn get_env(key: &str) -> String { return dotenvy::var(key).unwrap_or("".to_string()); }

  pub fn mk_tmp_scraps_folder() -> String {
    let scraps_name = get_env("DEFAULT_SCRAPS_FOLDER_NAME");
    let pwd=  current_dir().unwrap().display().to_string();
    let full_path = format!("{}/{}", pwd, scraps_name);
    if !fs::exists(&full_path).unwrap(){
      fs::create_dir(&full_path).expect("Couldn't create default temporary dir for the scraps folder");
    }
    return full_path;
  }

  pub fn clear_scraps_folder() -> Result<(), Box<dyn std::error::Error>> {
    // folder path paramether will be scraps by default
    let folder_path = get_env("DEFAULT_SCRAPS_FOLDER_NAME");
    if let Err(e) = fs::remove_dir_all(folder_path) {
      panic!("Error removing scraps temporary dir {}", e)
    } 
    Ok(())
  }

  pub fn img_to_jpeg(image_path: &String) -> String {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    let img_path = image_path.clone();
    let path_parts: Vec<&str> = img_path.split(".").collect();
    let new_path: String = format!("{}.jpg", path_parts[0]);
    let _ = img.save_with_format(new_path, Jpeg);
    // removes the old image
    if let Err(_) = fs::remove_file(image_path) {
      panic!("Couldn remove older image file name");
    };
    return format!("{}.jpg", path_parts[0]);
  }
  
}