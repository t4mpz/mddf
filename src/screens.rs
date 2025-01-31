pub mod screens{
  use crate::argscommands::argscommands;
  use indicatif::ProgressBar;
  use crate::structures::{structures::SearchResult, structures::ResultDisplayOptions};
  

  fn generate_result_string_by_options(result: SearchResult, option: ResultDisplayOptions) -> String {
    let vectorized_options: Vec<bool> = vec![option.href, option.img_href, option.story_name, option.last_chapter, option.additional_info];
    let vectorized_result: Vec<String> = vec![result.href, result.img_href, result.story_name, result.last_chapter, result.additional_info.join(" | ")];
    let mut summarized_data: Vec<String> = Vec::new();
    for i in 0..vectorized_options.len(){
        if vectorized_options[i] {
          summarized_data.push(vectorized_result[i].clone());
        }
    };
    return summarized_data.join("\t");
  }


  // Indexes the query search results and returns it as a text to be 
  // implemented in a std::io::out
  // it can include the results href or just the titles using the include_links option
  // which by default its true
  pub fn result_renderer(results: Vec<SearchResult>, include_links: Option<bool>) -> String{
    let include_href = include_links.unwrap_or(true);
    let parsed_string_results: Vec<String> = results.iter().enumerate().map(| key_result | {
      let result = key_result.1;
      let key = key_result.0;
      let href_parsed = if include_href  { format!("({})", result.href).to_string()} else  { "".to_string() };
      format!("[{}] {} {}", key, result.story_name, href_parsed).to_string()
    }).collect();
    return parsed_string_results.join("\n");
  }
}