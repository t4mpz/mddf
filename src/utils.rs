pub mod utils{
  use std::fs;
  use dotenvy;

  pub fn write_base_env(file_name: String){
    let contents = "SCRAPS_FOLDER=scraps\nDEFAULT_FOLDER=downloads";
    let _ = fs::write(file_name, contents);
  }

  pub fn get_env(key: &str) -> String{ return dotenvy::var(key).unwrap_or("".to_string()); }
}