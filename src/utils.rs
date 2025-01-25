pub mod utils{
  use std::fs;
  use dotenvy;
  use std::env::current_dir;
  use regex::Regex;

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
  
}