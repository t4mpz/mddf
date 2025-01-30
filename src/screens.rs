pub mod screens{
  use crate::argscommands::argscommands;
  use indicatif::ProgressBar;
  use crate::structures::structures;
  use cli_table::{format::Justify, Table, Style};

  

  // Indexes the query search results and returns it as a text to be 
  // implemented in a std::io::out
  // it can include the results href or just the titles using the include_links option
  // which by default its true
  pub fn result_renderer(results: Vec<structures::SearchResult>, include_links: Option<bool>) -> Vec<String>{
    let include_href = include_links.unwrap_or(true);
    let parsed_string_results: Vec<String> = results.iter().enumerate().map(| key_result | {
      let result = key_result.1;
      let key = key_result.0;
      let href_parsed = if include_href  { format!("({})", result.href).to_string()} else  { "".to_string() };
      format!("[{}] {} {}", key, result.story_name, href_parsed).to_string()
    }).collect();
    return parsed_string_results;
  }
}